# rust_nbc [![Build Status](https://travis-ci.com/gokberkkocak/rust_nbc.svg?branch=master)](https://travis-ci.com/gokberkkocak/rust_nbc) [![Build Status](https://github.com/gokberkkocak/rust_nbc/workflows/build-test/badge.svg)](https://github.com/gokberkkocak/rust_nbc/actions)
A Rust API for nbc_minisat_all Sat solver with alterations to expose fields and solutions.

There is a solution callback added on the c backend. This solution handler can be accessed by

```
extern "C" {
    fn nbc_register_callback(
        target: *mut MyAwesomeData,
        cb: extern "C" fn(*mut MyAwesomeData, *mut solver),
    ) -> ::std::os::raw::c_int;
}
```
and called to register the callback behaviour afterwards.

The licence info can be seen for the nbc_minisat_all in the vendor folder.
