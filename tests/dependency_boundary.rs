use std::{fs, path::PathBuf, process::Command};

fn cargo_tree(edges: &str, extra: &[&str]) -> String {
    let mut command = Command::new("cargo");
    command.args(["tree", "--edges", edges, "--no-default-features"]);
    command.args(extra);
    let output = command.output().expect("run cargo tree");
    assert!(output.status.success(), "status: {:?}", output.status);
    String::from_utf8(output.stdout).expect("dependency tree")
}

#[test]
fn default_runtime_tree_excludes_bootstrap_and_retired_crates() {
    let tree = cargo_tree("normal", &[]);
    assert!(tree.contains("signal-standard v0.3.0"), "{tree}");
    for forbidden in [
        "core-ethos",
        "name-table",
        "nota",
        "rust-logos",
        concat!("schema", "-language"),
        "schema-rust",
        "sema-translator",
        "signal-sema-translator",
        "structural-codec",
    ] {
        assert!(
            !tree.contains(forbidden),
            "runtime contains {forbidden}:\n{tree}"
        );
    }
}

#[test]
fn build_tree_has_only_the_exact_corrected_generator_and_standard() {
    let tree = cargo_tree("build", &[]);
    let generator_lines = tree
        .lines()
        .filter(|line| line.contains("schema-rust v"))
        .collect::<Vec<_>>();
    assert!(!generator_lines.is_empty(), "{tree}");
    assert!(
        generator_lines.iter().all(|line| line.contains(
            "schema-rust v0.15.0 (https://github.com/LiGoldragon/schema-rust.git?rev=9e36587c85bd69357e9042729ba2df0052799756#9e36587c)"
        )),
        "{tree}"
    );
    assert!(
        tree.contains("signal-standard.git?rev=d5a4a545e61dafec30667f2af38ca503ab6d6d3f#d5a4a545")
    );
    assert!(!tree.contains(concat!("schema", "-language")), "{tree}");
    assert_eq!(
        include_str!("../Cargo.lock")
            .matches("name = \"schema-rust\"")
            .count(),
        1
    );
}

#[test]
fn dotos_is_the_only_text_projection_opt_in() {
    let tree = cargo_tree("normal", &["--features", "dotos-text"]);
    assert!(tree.contains("dotos"), "{tree}");
    assert!(!tree.contains("nota"), "{tree}");
}

#[test]
fn obsolete_emitter_vocabulary_is_absent_from_active_build_inputs() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let build = fs::read_to_string(root.join("build.rs")).expect("build source");
    let cargo = fs::read_to_string(root.join("Cargo.toml")).expect("cargo manifest");
    for obsolete in [
        concat!("Generation", "Driver"),
        concat!("Generation", "Plan"),
        concat!("Module", "Emission"),
        concat!("ContractCrate", "Build"),
        concat!("CargoSchema", "Metadata"),
        concat!("Dependency", "Schema"),
        concat!("schema", "-language"),
    ] {
        assert!(!build.contains(obsolete), "build.rs contains {obsolete}");
        assert!(!cargo.contains(obsolete), "Cargo.toml contains {obsolete}");
    }
}
