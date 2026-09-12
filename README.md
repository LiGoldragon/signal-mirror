# signal-mirror

The ordinary Interface of the payload-blind version-control mirror.

`ethos/signal.ethos` is the sole schema authority. It expresses append,
checkpoint, object-notice, restore, and head-observation relations while
leaving component payloads opaque. Content addresses and transport endpoints
are imported from `signal`; Rust is a generated projection.

See `ARCHITECTURE.md` for the boundary and `skills.md` before editing.
