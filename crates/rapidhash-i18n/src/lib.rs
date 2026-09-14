//! Fluent catalog validation for RapidHash.

use fluent_syntax::{ast, parser};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

/// Canonical locale used as the structural source of truth.
pub const CANONICAL_LOCALE: &str = "en";

/// Locales shipped by the initial scaffold.
pub const SUPPORTED_LOCALES: &[&str] = &[CANONICAL_LOCALE, "zh-CN"];

/// Catalog domains required for every supported locale.
pub const CATALOG_FILES: &[&str] = &["common.ftl", "cli.ftl", "desktop.ftl"];

#[derive(Debug, Clone, PartialEq, Eq)]
struct MessageSignature {
    value_variables: BTreeSet<String>,
    attributes: BTreeMap<String, BTreeSet<String>>,
}

/// A catalog validation failure with source context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    context: String,
    reason: String,
}

impl ValidationError {
    fn new(context: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            context: context.into(),
            reason: reason.into(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.context, self.reason)
    }
}

impl std::error::Error for ValidationError {}

fn collect_pattern_variables<S: AsRef<str>>(
    pattern: &ast::Pattern<S>,
    variables: &mut BTreeSet<String>,
) {
    for element in &pattern.elements {
        if let ast::PatternElement::Placeable { expression } = element {
            collect_expression_variables(expression, variables);
        }
    }
}

fn collect_expression_variables<S: AsRef<str>>(
    expression: &ast::Expression<S>,
    variables: &mut BTreeSet<String>,
) {
    match expression {
        ast::Expression::Inline(inline) => collect_inline_variables(inline, variables),
        ast::Expression::Select { selector, variants } => {
            collect_inline_variables(selector, variables);
            for variant in variants {
                collect_pattern_variables(&variant.value, variables);
            }
        }
    }
}

fn collect_inline_variables<S: AsRef<str>>(
    expression: &ast::InlineExpression<S>,
    variables: &mut BTreeSet<String>,
) {
    match expression {
        ast::InlineExpression::VariableReference { id } => {
            variables.insert(id.name.as_ref().to_owned());
        }
        ast::InlineExpression::FunctionReference { arguments, .. } => {
            collect_argument_variables(arguments, variables);
        }
        ast::InlineExpression::TermReference {
            arguments: Some(arguments),
            ..
        } => collect_argument_variables(arguments, variables),
        ast::InlineExpression::Placeable { expression } => {
            collect_expression_variables(expression, variables);
        }
        _ => {}
    }
}

fn collect_argument_variables<S: AsRef<str>>(
    arguments: &ast::CallArguments<S>,
    variables: &mut BTreeSet<String>,
) {
    for argument in &arguments.positional {
        collect_inline_variables(argument, variables);
    }
    for argument in &arguments.named {
        collect_inline_variables(&argument.value, variables);
    }
}

fn parse_signatures(
    source: &str,
    context: &str,
) -> Result<BTreeMap<String, MessageSignature>, ValidationError> {
    let resource = parser::parse(source).map_err(|(_, errors)| {
        ValidationError::new(context, format!("invalid Fluent syntax: {errors:?}"))
    })?;
    let mut signatures = BTreeMap::new();

    for entry in resource.body {
        let ast::Entry::Message(message) = entry else {
            continue;
        };
        let message_id = message.id.name.to_string();
        let mut value_variables = BTreeSet::new();
        if let Some(value) = &message.value {
            collect_pattern_variables(value, &mut value_variables);
        }

        let mut attributes = BTreeMap::new();
        for attribute in message.attributes {
            let attribute_id = attribute.id.name.to_string();
            let mut variables = BTreeSet::new();
            collect_pattern_variables(&attribute.value, &mut variables);
            if attributes.insert(attribute_id.clone(), variables).is_some() {
                return Err(ValidationError::new(
                    context,
                    format!("duplicate attribute '{message_id}.{attribute_id}'"),
                ));
            }
        }

        let signature = MessageSignature {
            value_variables,
            attributes,
        };
        if signatures.insert(message_id.clone(), signature).is_some() {
            return Err(ValidationError::new(
                context,
                format!("duplicate message '{message_id}'"),
            ));
        }
    }

    if signatures.is_empty() {
        return Err(ValidationError::new(
            context,
            "catalog contains no messages",
        ));
    }
    Ok(signatures)
}

/// Validate message, attribute, and variable parity between two Fluent resources.
pub fn validate_catalog_pair(
    canonical_source: &str,
    target_source: &str,
    context: &str,
) -> Result<(), ValidationError> {
    let canonical = parse_signatures(canonical_source, context)?;
    let target = parse_signatures(target_source, context)?;
    if canonical != target {
        return Err(ValidationError::new(
            context,
            format!(
                "catalog structure differs from English; expected {canonical:?}, found {target:?}"
            ),
        ));
    }
    Ok(())
}

fn catalog_file_names(locale_dir: &Path) -> Result<BTreeSet<String>, ValidationError> {
    let entries = fs::read_dir(locale_dir).map_err(|error| {
        ValidationError::new(locale_dir.display().to_string(), error.to_string())
    })?;
    let mut names = BTreeSet::new();

    for entry in entries {
        let entry = entry.map_err(|error| {
            ValidationError::new(locale_dir.display().to_string(), error.to_string())
        })?;
        let path = entry.path();
        if path.extension().is_some_and(|extension| extension == "ftl") {
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                return Err(ValidationError::new(
                    path.display().to_string(),
                    "catalog file name is not valid Unicode",
                ));
            };
            names.insert(name.to_owned());
        }
    }
    Ok(names)
}

/// Validate catalog files, Fluent syntax, and structural parity for all locales.
pub fn validate_locales(locales_dir: &Path) -> Result<(), ValidationError> {
    let required_files = CATALOG_FILES
        .iter()
        .map(|file| (*file).to_owned())
        .collect::<BTreeSet<_>>();

    for locale in SUPPORTED_LOCALES {
        let locale_dir = locales_dir.join(locale);
        let actual_files = catalog_file_names(&locale_dir)?;
        if actual_files != required_files {
            return Err(ValidationError::new(
                locale_dir.display().to_string(),
                format!("expected {required_files:?}, found {actual_files:?}"),
            ));
        }
    }

    for file in CATALOG_FILES {
        let canonical_path = locales_dir.join(CANONICAL_LOCALE).join(file);
        let canonical = fs::read_to_string(&canonical_path).map_err(|error| {
            ValidationError::new(canonical_path.display().to_string(), error.to_string())
        })?;

        for locale in SUPPORTED_LOCALES.iter().skip(1) {
            let target_path = locales_dir.join(locale).join(file);
            let target = fs::read_to_string(&target_path).map_err(|error| {
                ValidationError::new(target_path.display().to_string(), error.to_string())
            })?;
            validate_catalog_pair(&canonical, &target, &target_path.display().to_string())?;
        }
    }
    Ok(())
}
