use std::process::Command;

fn tree(edges: &str, feature: Option<&str>) -> String {
    let mut cargo = Command::new("cargo");
    cargo.args(["tree", "--edges", edges, "--no-default-features"]);
    if let Some(feature) = feature {
        cargo.args(["--features", feature]);
    }
    let output = cargo.output().expect("cargo tree");
    assert!(output.status.success(), "{:?}", output.status);
    String::from_utf8(output.stdout).expect("tree utf8")
}

#[test]
fn default_contract_has_no_retired_codec_or_generator() {
    let tree = tree("normal", None);
    for forbidden in [
        "dotos",
        "signal-frame",
        "schema-rust",
        "core-ethos",
        "sema-translator",
    ] {
        assert!(!tree.contains(forbidden), "{forbidden}: {tree}");
    }
}

#[test]
fn datom_is_opt_in_and_uses_final_identity() {
    let tree = tree("normal", Some("datom"));
    assert!(tree.contains("datom-codec v0.26.3"), "{tree}");
    assert!(tree.contains("protos v0.30.1"), "{tree}");
    assert!(!tree.contains("dotos"), "{tree}");
}
