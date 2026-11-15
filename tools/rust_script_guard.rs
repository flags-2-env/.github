#![forbid(unsafe_code)]

//! Zero-dependency guard for new Python test and automation entry points.
//! This governance repo contains no Python product runtime; application-code
//! exceptions are reviewed in owning product repositories, never inferred here.

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Component, Path};
use std::process;

fn policy_violation(path: &str, first_line: &str) -> Option<&'static str> {
    let name = Path::new(path)
        .file_name()?
        .to_string_lossy()
        .to_ascii_lowercase();

    if name.ends_with(".py") || name.ends_with(".pyw") {
        return Some("Python scripts/tests are prohibited; use Rust");
    }
    if matches!(name.as_str(), "pytest.ini" | "tox.ini" | ".python-version") {
        return Some("new Python test/tool configuration is prohibited");
    }
    let shebang = first_line.trim_start().to_ascii_lowercase();
    if shebang.starts_with("#!") && shebang.contains("python") {
        return Some("Python interpreter shebang is prohibited; use Rust");
    }
    None
}

fn audit_changed_path(path: &Path) -> Result<(), String> {
    if path.is_absolute()
        || path
            .components()
            .any(|component| component == Component::ParentDir)
    {
        return Err(format!("unsafe changed path: {}", path.display()));
    }
    let metadata = fs::symlink_metadata(path)
        .map_err(|_| format!("cannot inspect changed path: {}", path.display()))?;
    if !metadata.file_type().is_file() {
        return Err(format!("non-file changed path requires review: {}", path.display()));
    }

    // Read only a bounded prefix to inspect shebangs. Do not print contents.
    let mut prefix = Vec::new();
    File::open(path)
        .map_err(|_| format!("cannot open changed path: {}", path.display()))?
        .take(512)
        .read_to_end(&mut prefix)
        .map_err(|_| format!("cannot inspect changed file: {}", path.display()))?;
    let first_line = String::from_utf8_lossy(&prefix)
        .lines()
        .next()
        .unwrap_or("")
        .to_owned();
    match policy_violation(&path.to_string_lossy(), &first_line) {
        Some(reason) => Err(format!("{}: {reason}", path.display())),
        None => Ok(()),
    }
}

fn run() -> Result<(), String> {
    let mut paths = Vec::new();
    io::stdin()
        .read_to_end(&mut paths)
        .map_err(|_| "cannot read NUL-delimited Git path input".to_owned())?;
    if !paths.is_empty() && paths.last() != Some(&0) {
        return Err("Git changed-path stream must be NUL-terminated".to_owned());
    }
    let mut checked = 0_usize;
    for raw in paths.split(|byte| *byte == 0).filter(|name| !name.is_empty()) {
        let name = std::str::from_utf8(raw)
            .map_err(|_| "changed paths must be valid UTF-8 in this repository".to_owned())?;
        audit_changed_path(Path::new(name))?;
        checked += 1;
    }
    println!("Rust-first policy checked {checked} added/modified paths");
    Ok(())
}

fn main() {
    if let Err(reason) = run() {
        eprintln!("{reason}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::policy_violation;

    #[test]
    fn rejects_python_scripts_including_mixed_case() {
        for path in [
            "tools/check.py",
            "tests/test_contract.PY",
            ".github/scripts/run.pyw",
            "nested/.hidden.Py",
        ] {
            assert!(policy_violation(path, "").is_some(), "{path}");
        }
    }

    #[test]
    fn rejects_python_test_configuration() {
        for path in ["pytest.ini", "tests/tox.ini", ".python-version"] {
            assert!(policy_violation(path, "").is_some(), "{path}");
        }
    }

    #[test]
    fn rejects_extensionless_python_launchers() {
        for first in [
            "#!/usr/bin/python3",
            "#!/usr/bin/env python3 -I",
            "#!/usr/bin/env -S python -B",
        ] {
            assert!(policy_violation("tools/check", first).is_some(), "{first}");
        }
    }

    #[test]
    fn permits_rust_scripts_and_unrelated_project_files() {
        for (path, first) in [
            ("tools/check.rs", ""),
            ("tests/test_case.rs", ""),
            ("CONTRIBUTING.md", "# Rust-first tests"),
            ("scripts/check.sh", "#!/bin/sh"),
            ("docs/example.md", "python snippets are documentation, not an entry point"),
        ] {
            assert_eq!(policy_violation(path, first), None, "{path}");
        }
    }
}
