# idna_adapter

This crate abstracts over a Unicode back end for the `idna` crate.

To work around the lack of [`global-features`](https://internals.rust-lang.org/t/pre-rfc-mutually-excusive-global-features/19618) in Cargo, this crate allows the top level `Cargo.lock` to choose an alternative Unicode back end for the `idna` crate by pinning a version of this crate.

`idna` depends on version 1 of this crate. The version stream 1.2.x uses ICU4X and the version stream 1.1.x uses unicode-rs. That is, if you take no action, Cargo will choose the 1.2.x version stream i.e. ICU4X. To choose unicode-rs instead, run `cargo update -p idna_adapter --precise 1.1.0` in the top-level directory of your application.

