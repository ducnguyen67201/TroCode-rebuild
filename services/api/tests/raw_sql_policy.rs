use std::{fs, path::Path};

fn rust_files(directory: &Path, files: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(directory).expect("Rust source directory should be readable") {
        let path = entry.expect("source entry should be readable").path();
        if path.is_dir() {
            rust_files(&path, files);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
}

fn violations(source: &str) -> Vec<String> {
    let mut violations = Vec::new();
    let sqlx = ["sqlx", "::"].concat();
    for suffix_parts in [
        ["query", "("],
        ["query", "!("],
        ["query_as", "("],
        ["query_as", "!("],
        ["query_scalar", "("],
        ["query_scalar", "!("],
    ] {
        let suffix = suffix_parts.concat();
        let marker = format!("{sqlx}{suffix}");
        if source.contains(&marker) {
            violations.push(marker);
        }
    }

    for parts in [
        ["Statement", "::from_string"],
        ["Statement", "::from_sql_and_values"],
        ["execute", "_unprepared"],
    ] {
        let marker = parts.concat();
        if source.contains(&marker) {
            violations.push(marker);
        }
    }

    let verbs = [
        "SELECT", "INSERT", "UPDATE", "DELETE", "CREATE", "ALTER", "DROP", "PRAGMA",
    ];
    for (index, _) in source.match_indices('"') {
        let literal_start = source[index + 1..].trim_start().to_ascii_uppercase();
        if let Some(verb) = verbs.iter().find(|verb| {
            literal_start.strip_prefix(**verb).is_some_and(|rest| {
                rest.chars()
                    .next()
                    .is_some_and(|character| character.is_whitespace() || character == '(')
            })
        }) {
            violations.push(format!("SQL string literal beginning with {verb}"));
        }
    }
    violations
}

#[test]
fn rust_api_has_no_embedded_raw_sql() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    rust_files(&root.join("src"), &mut files);
    rust_files(&root.join("tests"), &mut files);

    let mut failures = Vec::new();
    for file in files {
        let source = fs::read_to_string(&file).expect("Rust source should be readable");
        for violation in violations(&source) {
            failures.push(format!("{}: {violation}", file.display()));
        }
    }
    assert!(
        failures.is_empty(),
        "Rust application queries must use SeaORM/SeaQuery:\n{}",
        failures.join("\n")
    );
}

#[test]
fn policy_detects_query_apis_escape_hatches_and_sql_literals() {
    let samples = [
        [["sqlx", "::"].concat(), ["query", "("].concat()].concat(),
        [["Statement", "::from_string"].concat(), ["", "("].concat()].concat(),
        ["\"SE", "LECT 1\""].concat(),
    ];
    for sample in samples {
        assert!(!violations(&sample).is_empty(), "missed marker: {sample}");
    }
}
