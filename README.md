# rust-trousers-testsuite-common

Native bindings to the TrouSerS test suite common modules.

Thanks to @ltfschoen for the [writeup](https://users.rust-lang.org/t/linking-with-custom-c-library/637/3) on creating a crate from custom C code.

Bindings generated with

    bindgen -l trousers-bindgen-common -match common.h -o common.rs /path/to/trousers-testsuite/tcg/include/common.h
