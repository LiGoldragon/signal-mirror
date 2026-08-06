# signal-mirror

The ordinary Interface of the payload-blind version-control mirror.

`ethos/interface.ethos` is the sole schema authority. It expresses append,
checkpoint, object-notice, restore, and head-observation relations while
leaving component payloads opaque. Content addresses and transport endpoints
are imported from `signal-standard`; Rust is an encoded projection.

See `ARCHITECTURE.md` for the boundary and `skills.md` before editing.
