# janet-ll

This is a rust crate providing low level bindings to the [janet](https://janet-lang.org/) C API.

# versioning scheme

The package major and minor versions follow the janet releases the
bundled headers are compatible with. The patch version may, but does
not necessarily match the janet release.

# feature flags

if the feature link-amalg is enabled, the crate will link the compiled
janet amalgamation, otherwise this crate is header only, which is useful
for building standalone janet modules.

# safety

This crate makes no attempt at ergonomics or memory safety.
It is the bare minimum required to use the janet api from
rust. Higher level crates may come in the future.
