use rapidhash_core::{
    resolve_manifest_path, to_portable_manifest_path, traverse_paths, CoreError, TraversalOptions,
};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::AtomicBool;

#[test]
fn path_escape_attacks_are_rejected() {
    let root = Path::new(if cfg!(windows) {
        r"C:\app\root"
    } else {
        "/app/root"
    });

    // Parent directory traversal escape
    assert!(matches!(
        resolve_manifest_path(root, "../secret.txt"),
        Err(CoreError::PathEscape { .. })
    ));

    assert!(matches!(
        resolve_manifest_path(root, "subdir/../../secret.txt"),
        Err(CoreError::PathEscape { .. })
    ));

    // NUL byte injection
    assert!(matches!(
        resolve_manifest_path(root, "file.txt\0.bad"),
        Err(CoreError::PathEscape { .. })
    ));

    // Absolute POSIX path
    assert!(matches!(
        resolve_manifest_path(root, "/etc/passwd"),
        Err(CoreError::PathEscape { .. })
    ));

    // Windows drive letters / alternate data streams
    assert!(matches!(
        resolve_manifest_path(root, "C:\\Windows\\System32"),
        Err(CoreError::PathEscape { .. })
    ));

    assert!(matches!(
        resolve_manifest_path(root, "test.txt:stream"),
        Err(CoreError::PathEscape { .. })
    ));
}

#[test]
fn valid_relative_paths_are_confined() {
    let root = Path::new(if cfg!(windows) {
        r"C:\app\root"
    } else {
        "/app/root"
    });

    let resolved = resolve_manifest_path(root, "subdir/file.txt").expect("valid relative path");
    assert_eq!(resolved, root.join("subdir").join("file.txt"));

    // Redundant dots
    let resolved_dots =
        resolve_manifest_path(root, "./subdir/./sub2/../file.txt").expect("valid dots");
    assert_eq!(resolved_dots, root.join("subdir").join("file.txt"));
}

#[test]
fn to_portable_manifest_path_uses_forward_slashes() {
    let p = Path::new("subdir").join("nested").join("file.txt");
    assert_eq!(to_portable_manifest_path(&p), "subdir/nested/file.txt");
}

#[test]
fn traversal_is_deterministic_and_respects_options() {
    let temp_dir = std::env::temp_dir().join("rapidhash_traversal_test_det");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(temp_dir.join("b_dir")).unwrap();
    fs::create_dir_all(temp_dir.join("a_dir")).unwrap();

    File::create(temp_dir.join("b_dir").join("file2.txt"))
        .unwrap()
        .write_all(b"2")
        .unwrap();
    File::create(temp_dir.join("b_dir").join("file1.txt"))
        .unwrap()
        .write_all(b"1")
        .unwrap();
    File::create(temp_dir.join("a_dir").join("file3.txt"))
        .unwrap()
        .write_all(b"3")
        .unwrap();
    File::create(temp_dir.join(".hidden.txt"))
        .unwrap()
        .write_all(b"h")
        .unwrap();

    // Default traversal: hidden included, sorted deterministically
    let options = TraversalOptions::default();
    let files = traverse_paths(std::slice::from_ref(&temp_dir), &options, None)
        .expect("traversal succeeds");

    let relative_paths: Vec<String> = files
        .iter()
        .map(|f| {
            f.path
                .strip_prefix(&temp_dir)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/")
        })
        .collect();

    assert_eq!(
        relative_paths,
        vec![
            ".hidden.txt",
            "a_dir/file3.txt",
            "b_dir/file1.txt",
            "b_dir/file2.txt",
        ]
    );

    // Exclude hidden
    let no_hidden_opts = TraversalOptions {
        include_hidden: false,
        ..Default::default()
    };
    let files_no_hidden = traverse_paths(std::slice::from_ref(&temp_dir), &no_hidden_opts, None)
        .expect("traversal succeeds");

    let rel_no_hidden: Vec<String> = files_no_hidden
        .iter()
        .map(|f| {
            f.path
                .strip_prefix(&temp_dir)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/")
        })
        .collect();

    assert_eq!(
        rel_no_hidden,
        vec!["a_dir/file3.txt", "b_dir/file1.txt", "b_dir/file2.txt",]
    );

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn traversal_enforces_max_depth() {
    let temp_dir = std::env::temp_dir().join("rapidhash_traversal_depth");
    let _ = fs::remove_dir_all(&temp_dir);
    let deep = temp_dir.join("d1").join("d2").join("d3");
    fs::create_dir_all(&deep).unwrap();
    File::create(deep.join("leaf.txt"))
        .unwrap()
        .write_all(b"leaf")
        .unwrap();

    let opts = TraversalOptions {
        max_depth: Some(1), // Allow only root -> d1
        ..Default::default()
    };
    let result = traverse_paths(std::slice::from_ref(&temp_dir), &opts, None);
    assert!(result.is_err());

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn traversal_enforces_max_entries() {
    let temp_dir = std::env::temp_dir().join("rapidhash_traversal_entries");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();
    File::create(temp_dir.join("f1.txt"))
        .unwrap()
        .write_all(b"1")
        .unwrap();
    File::create(temp_dir.join("f2.txt"))
        .unwrap()
        .write_all(b"2")
        .unwrap();
    File::create(temp_dir.join("f3.txt"))
        .unwrap()
        .write_all(b"3")
        .unwrap();

    let opts = TraversalOptions {
        max_entries: Some(2),
        ..Default::default()
    };
    let result = traverse_paths(std::slice::from_ref(&temp_dir), &opts, None);
    assert!(result.is_err());

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn traversal_cancellation() {
    let temp_dir = std::env::temp_dir().join("rapidhash_traversal_cancel");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();
    File::create(temp_dir.join("f1.txt"))
        .unwrap()
        .write_all(b"1")
        .unwrap();

    let cancel = AtomicBool::new(true);
    let opts = TraversalOptions::default();
    let result = traverse_paths(std::slice::from_ref(&temp_dir), &opts, Some(&cancel));
    assert!(matches!(result, Err(CoreError::Cancelled)));

    let _ = fs::remove_dir_all(&temp_dir);
}
