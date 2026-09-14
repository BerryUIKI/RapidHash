//! Localization and message resolution for the CLI adapter.
//!
//! Embeds the repository Fluent catalogs (locales/en and locales/zh-CN)
//! and provides fallback-aware message formatting.

use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use std::env;
use unic_langid::LanguageIdentifier;

const EN_COMMON: &str = include_str!("../../../locales/en/common.ftl");
const EN_CLI: &str = include_str!("../../../locales/en/cli.ftl");
const EN_DESKTOP: &str = include_str!("../../../locales/en/desktop.ftl");

const ZH_COMMON: &str = include_str!("../../../locales/zh-CN/common.ftl");
const ZH_CLI: &str = include_str!("../../../locales/zh-CN/cli.ftl");
const ZH_DESKTOP: &str = include_str!("../../../locales/zh-CN/desktop.ftl");

/// Manages localized message catalogs for the CLI.
pub struct I18n {
    bundle: FluentBundle<FluentResource>,
    fallback_bundle: FluentBundle<FluentResource>,
}

impl I18n {
    /// Initialize I18n with locale resolution precedence:
    /// 1. Explicit override option (`--locale`).
    /// 2. Environment variables (`LANGUAGE`, `LC_ALL`, `LANG`).
    /// 3. Canonical default ("en").
    pub fn new(locale_override: Option<&str>) -> Self {
        let requested_tag = locale_override
            .map(|s| s.to_string())
            .or_else(detect_env_locale)
            .unwrap_or_else(|| "en".to_string());

        let target_lang: LanguageIdentifier = requested_tag
            .parse()
            .unwrap_or_else(|_| "en".parse().unwrap());

        let bundle = build_bundle(&target_lang);
        let fallback_bundle = build_bundle(&"en".parse().unwrap());

        Self {
            bundle,
            fallback_bundle,
        }
    }

    /// Format a message key with optional interpolation string arguments.
    pub fn format(&self, id: &str, args: &[(&str, &str)]) -> String {
        let mut fluent_args = FluentArgs::new();
        for &(k, v) in args {
            fluent_args.set(k, FluentValue::from(v));
        }
        let args_ref = if args.is_empty() {
            None
        } else {
            Some(&fluent_args)
        };

        let mut errors = vec![];

        // Try selected locale first
        if let Some(msg) = self.bundle.get_message(id) {
            if let Some(pattern) = msg.value() {
                let value = self.bundle.format_pattern(pattern, args_ref, &mut errors);
                return value.to_string();
            }
        }

        // Fallback to English
        errors.clear();
        if let Some(msg) = self.fallback_bundle.get_message(id) {
            if let Some(pattern) = msg.value() {
                let value = self
                    .fallback_bundle
                    .format_pattern(pattern, args_ref, &mut errors);
                return value.to_string();
            }
        }

        id.to_string()
    }
}

fn build_bundle(lang: &LanguageIdentifier) -> FluentBundle<FluentResource> {
    let mut bundle = FluentBundle::new(vec![lang.clone()]);
    bundle.set_use_isolating(false);

    let (common_src, cli_src, desktop_src) = if lang.language.as_str() == "zh" {
        (ZH_COMMON, ZH_CLI, ZH_DESKTOP)
    } else {
        (EN_COMMON, EN_CLI, EN_DESKTOP)
    };

    if let Ok(res) = FluentResource::try_new(common_src.to_string()) {
        let _ = bundle.add_resource(res);
    }
    if let Ok(res) = FluentResource::try_new(cli_src.to_string()) {
        let _ = bundle.add_resource(res);
    }
    if let Ok(res) = FluentResource::try_new(desktop_src.to_string()) {
        let _ = bundle.add_resource(res);
    }

    bundle
}

fn detect_env_locale() -> Option<String> {
    for var in &["LANGUAGE", "LC_ALL", "LC_MESSAGES", "LANG"] {
        if let Ok(val) = env::var(var) {
            let val = val.trim();
            if !val.is_empty() && val != "C" && val != "POSIX" {
                // Strip encoding suffix like .UTF-8 or @euro
                let clean = val.split(['.', '@']).next().unwrap_or(val);
                return Some(clean.replace('_', "-"));
            }
        }
    }
    None
}
