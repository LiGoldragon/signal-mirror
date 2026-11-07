# Upgrades

## 5.0.0 → 5.1.0

First Datom/Protos 0.33 adoption. Authored Ethos, library, build script and
canonical examples are unchanged. Nine generated aliases become tuple
structs with Datom derives. The frame fixture now constructs HeadQuery(None)
and the independent FNV digest fixture wraps its unchanged numeric value in
Signal ContractDigest. The authored-source digest and greeting/mismatch
assertions remain unchanged.

Selected cohort: Datom eabead4 / Protos 3c7ade3 (0.33.0), Signal 7afb68b
(10.3.0), Ethos Zero 5a5daca (19.2.0), versus the historical 0.31 codec/parser
and Signal 7.0 cohort. Registry packages/checksums remain unchanged.
The existing dependency graph assertions now expect 0.33 instead of 0.31.

Canonical comparison uses compact() to retain exact single-line equality
despite the current vertical printer; exhaustive root matches and all 15
original canonical lines remain. ReaderBudget construction is checked.
Raw witnesses separately use textualize(), not compact normalization.

Historical e2a2a58 source with its own lock/tools and the candidate cover the
five Query and ten Response canonical root values. Archives/frames are
byte-equal 15/15, each local frame restores, and both directions of typed
binary/text cross-reads succeed. Raw printed text is equal 6/15;
balanced-string/token inspection verifies layout-only differences in the
remaining nine samples. This is bounded nested/value/refusal evidence, not
exhaustive source-API, storage, runtime or peer compatibility proof.

Checks cover x86_64-linux only; other declared systems are omitted.
The canonical worktree's unrelated dependency-boundary name edit is
preserved separately and is not adopted here. No consumer/runtime rollout
is performed by this source release.
