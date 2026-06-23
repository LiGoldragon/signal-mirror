use schema_rust::build::ContractCrateBuild;

fn main() {
    ContractCrateBuild::from_environment(
        "signal-mirror",
        "0.1.1",
        "SIGNAL_MIRROR_UPDATE_SCHEMA_ARTIFACTS",
    )
    .expect_fresh();
}
