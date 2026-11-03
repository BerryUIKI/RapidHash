use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

fn rapidhash_bin() -> std::path::PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // exit test exe
    if path.ends_with("deps") {
        path.pop();
    }
    path.push(format!("rapidhash{}", std::env::consts::EXE_SUFFIX));
    path
}

#[test]
fn cli_algorithms_subcommand() {
    let bin = rapidhash_bin();
    let output = Command::new(&bin)
        .arg("algorithms")
        .output()
        .expect("should run rapidhash algorithms");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("sha256"));
    assert!(stdout.contains("blake3"));
    assert!(stdout.contains("crc32"));
    assert!(stdout.contains("sha3-256"));
    assert!(stdout.contains("blake2b-512"));
    assert!(stdout.contains("xxh3"));
    assert!(stdout.contains("md5"));

    // JSON mode
    let json_output = Command::new(&bin)
        .args(["--json", "algorithms"])
        .output()
        .expect("should run rapidhash algorithms --json");

    assert!(json_output.status.success());
    let json_stdout = String::from_utf8_lossy(&json_output.stdout);
    assert!(json_stdout.contains("\"id\": \"sha256\""));
}

#[test]
fn cli_hash_and_compare_subcommand() {
    let bin = rapidhash_bin();
    let temp_dir = std::env::temp_dir().join("rapidhash_cli_test_hash");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let file_path = temp_dir.join("sample.txt");
    File::create(&file_path).unwrap().write_all(b"abc").unwrap();

    // rapidhash hash sample.txt (NIST vector ba7816bf...)
    let output = Command::new(&bin)
        .args(["hash", file_path.to_str().unwrap()])
        .output()
        .expect("should run hash");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"));

    // rapidhash compare sample.txt <digest> (Match: exit code 0)
    let compare_match = Command::new(&bin)
        .args([
            "compare",
            file_path.to_str().unwrap(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ])
        .output()
        .expect("should run compare");

    assert_eq!(compare_match.status.code(), Some(0));

    // rapidhash compare sample.txt <wrong_digest> (Mismatch: exit code 1)
    let compare_mismatch = Command::new(&bin)
        .args([
            "compare",
            file_path.to_str().unwrap(),
            "0000000000000000000000000000000000000000000000000000000000000000",
        ])
        .output()
        .expect("should run compare mismatch");

    assert_eq!(compare_mismatch.status.code(), Some(1));

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn cli_hash_extended_algorithms() {
    let bin = rapidhash_bin();
    let temp_dir = std::env::temp_dir().join("rapidhash_cli_test_extended");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let file_path = temp_dir.join("abc.bin");
    File::create(&file_path).unwrap().write_all(b"abc").unwrap();

    // SHA3-256("abc") per FIPS 202.
    let sha3 = Command::new(&bin)
        .args([
            "hash",
            "--algorithm",
            "sha3-256",
            file_path.to_str().unwrap(),
        ])
        .output()
        .expect("should run hash with sha3-256");
    assert!(sha3.status.success());
    assert!(String::from_utf8_lossy(&sha3.stdout)
        .to_lowercase()
        .contains("3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"));

    // XXH64("abc") with a zero seed.
    let xxh64 = Command::new(&bin)
        .args(["hash", "--algorithm", "xxh64", file_path.to_str().unwrap()])
        .output()
        .expect("should run hash with xxh64");
    assert!(xxh64.status.success());
    assert!(String::from_utf8_lossy(&xxh64.stdout)
        .to_lowercase()
        .contains("44bc2cf5ad770999"));

    // BLAKE2b-512("abc") per RFC 7693.
    let blake2b = Command::new(&bin)
        .args([
            "hash",
            "--algorithm",
            "blake2b-512",
            file_path.to_str().unwrap(),
        ])
        .output()
        .expect("should run hash with blake2b-512");
    assert!(blake2b.status.success());
    assert!(String::from_utf8_lossy(&blake2b.stdout)
        .to_lowercase()
        .contains("ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d1"));

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn cli_verify_gnu_manifest() {
    let bin = rapidhash_bin();
    let temp_dir = std::env::temp_dir().join("rapidhash_cli_test_verify");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let target_file = temp_dir.join("sample.txt");
    File::create(&target_file)
        .unwrap()
        .write_all(b"abc")
        .unwrap();

    let manifest_path = temp_dir.join("checksums.sha256");
    let manifest_content =
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  sample.txt\n";
    File::create(&manifest_path)
        .unwrap()
        .write_all(manifest_content.as_bytes())
        .unwrap();

    // Verify manifest
    let output = Command::new(&bin)
        .args(["verify", manifest_path.to_str().unwrap()])
        .output()
        .expect("should verify manifest");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("sample.txt: OK"));

    // Verify with locale zh-CN
    let output_zh = Command::new(&bin)
        .args([
            "--locale",
            "zh-CN",
            "verify",
            manifest_path.to_str().unwrap(),
        ])
        .output()
        .expect("should verify with zh-CN");

    assert!(output_zh.status.success());
    let stdout_zh = String::from_utf8_lossy(&output_zh.stdout);
    assert!(stdout_zh.contains("已验证 1 个文件"));

    let _ = fs::remove_dir_all(&temp_dir);
}
