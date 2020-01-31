# rust_nbc
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
