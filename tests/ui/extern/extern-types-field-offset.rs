// run-fail
// check-run-results
// exec-env:RUST_BACKTRACE=0
// normalize-stderr-test: "(core/src/panicking\.rs):[0-9]+:[0-9]+" -> "$1:$$LINE:$$COL"
#![feature(extern_types)]

extern "C" {
    type Opaque;
}

struct Newtype(Opaque);

struct S {
    i: i32,
    a: Opaque,
}

fn main() {
    // Projecting to the newtype works, because it is always at offset 0.
    let x: &Newtype = unsafe { &*(1usize as *const Newtype) };
    let field = &x.0;

    // This needs to compute the field offset, but we don't know the type's alignment,
    // so this panics.
    let x: &S = unsafe { &*(1usize as *const S) };
    let field = &x.a;
}
