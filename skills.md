# Working in signal-mirror

Read `ARCHITECTURE.md` and the shared standard Interface before changing this
contract.

- Treat `ethos/interface.ethos` as the sole schema authority.
- Keep component payloads and checkpoint artifacts opaque.
- Import shared identities from their producer instead of copying them.
- Keep the Interface role-free and generated Rust encoded-only.
- Mint explicit declaration and variant seats; never derive identity from
  spelling, position, or content.
- Put structural, Dotos, rkyv, route, and Signal behavior in
  `src/schema/lib/behavior.rs`.
- Do not add mirror actors, storage, validation policy, listeners, or component
  record types here.
- Regenerate only with
  `SIGNAL_MIRROR_UPDATE_INTERFACE_ARTIFACTS=1 cargo build --all-features`, then
  prove an ordinary build is fresh.
- Run default and all-feature tests, formatting, clippy with warnings denied,
  rustdoc with warnings denied, and `nix flake check --all-systems`.
