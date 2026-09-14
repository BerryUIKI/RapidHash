use rapidhash_i18n::{validate_catalog_pair, validate_locales};
use std::path::{Path, PathBuf};

fn locales_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../locales")
}

#[test]
fn shipped_catalogs_have_matching_structure() {
    validate_locales(&locales_dir())
        .expect("shipped catalogs must be valid and structurally equal");
}

#[test]
fn invalid_fluent_syntax_is_rejected() {
    let result = validate_catalog_pair("message = Valid", "this is not valid", "test.ftl");
    assert!(result.is_err());
}

#[test]
fn missing_messages_are_rejected() {
    let result = validate_catalog_pair(
        "first = First\nsecond = Second",
        "first = First",
        "test.ftl",
    );
    assert!(result.is_err());
}

#[test]
fn different_attributes_are_rejected() {
    let result = validate_catalog_pair(
        "message = Value\n    .description = Description",
        "message = Value\n    .label = Label",
        "test.ftl",
    );
    assert!(result.is_err());
}

#[test]
fn different_variables_are_rejected() {
    let result = validate_catalog_pair(
        "message = Hello, { $name }",
        "message = Hello, { $user }",
        "test.ftl",
    );
    assert!(result.is_err());
}
