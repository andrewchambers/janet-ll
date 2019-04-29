# janet-ll

This is a rust crate providing low level bindings to the [janet](https://janet-lang.org/) C API.

# versioning scheme

The package major and minor versions follow the janet releases the
headers match, the package is quite strict in enforcing this, as
janet does not have stability in it's API. The patch version does
not necessarily match the janet release.

# feature flags

if the feature link-amalg is enabled, the crate will link the compiled
janet amalgamation, otherwise this crate is header only, which is useful
for building standalone janet modules.

# safety

This crate makes no attempt at ergonomics or memory safety.
It is the bare minimum required to use the janet api from
rust. Higher level crates may come in the future.
