use std::path::PathBuf;

#[test]
fn current_signal_preserves_imports_roots_and_payload_fields() {
    let source = signal_mirror::MIRROR_SIGNAL_SOURCE;
    assert!(source.starts_with("Signal\n[ signal_standard:[ ObjectDigest StandardSocket ] ]"));
    for declaration in [
        "Append.EntrySuffix",
        "Restore.RestoreQuery",
        "Appended.AppendReceipt",
        "Restored.RestoreBundle",
        "EntryEnvelope.{ CommitSequence Option<ObjectDigest> ObjectDigest PayloadBytes }",
    ] {
        assert!(source.contains(declaration), "missing {declaration}");
    }
}

#[test]
fn generated_signal_is_the_sole_active_contract() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert!(root.join("ethos/signal.ethos").is_file());
    assert!(signal_mirror::MIRROR_SIGNAL_RUST.contains("pub enum Query"));
    assert!(signal_mirror::MIRROR_SIGNAL_RUST.contains("pub enum Response"));
}
