# rust-bindgen

 * `emilio@mozilla.com`

## In a nutshell.

 * Generates bindings (struct and function definitions) from C / C++ code.

## Generates bindings for C code.

```c
// /usr/include/bzlib.h
BZ_EXTERN int BZ_API(BZ2_bzread) (
      BZFILE* b, 
      void* buf, 
      int len 
   );

BZ_EXTERN int BZ_API(BZ2_bzwrite) (
      BZFILE* b, 
      void*   buf, 
      int     len 
   );
// ...
```

```rust
extern "C" {
    pub fn BZ2_bzRead(
        bzerror: *mut ::std::os::raw::c_int,
        b: *mut BZFILE,
        buf: *mut ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BZ2_bzWrite(
        bzerror: *mut ::std::os::raw::c_int,
        b: *mut BZFILE,
        buf: *mut ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
    );
}
```

## And C++ code [^disclaimer]

```cpp
template<typename T, typename U> class Foo {
    T m_member;
    T* m_member_ptr;
    T m_member_arr[1];
};

template<typename T> class B {
    T m_member { 0 };
};

void bar(Foo<int, int> foo);
```

```rust
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Foo<T> {
    pub m_member: T,
    pub m_member_ptr: *mut T,
    pub m_member_arr: [T; 1usize],
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for Foo<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct B<T> {
    pub m_member: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for B<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    #[link_name = "\u{1}_Z3bar3FooIiiE"]
    pub fn bar(foo: Foo<::std::os::raw::c_int>);
}
```

[^disclaimer]: To some extent, totally not perfect.

### C++ caveats

It's generally pretty robust (we use it in Firefox's build, which means a _lot_
of C++ gets parsed), but:

 * No virtual functions ([#27](https://github.com/rust-lang-nursery/rust-bindgen/issues/27)).

   * Bindgen generates an empty vtable to handle struct layout properly.

   * Probably not hard to handle simple cases (no overloaded virtuals, etc.).

 * Templates can be hard:

   * Bindgen tries to preserve template parameters which generally works, but
     stuff like non-type template parameters, etc makes bindgen output an opaque
     blob for the instantiations of the template.

   * Lots of options to control what gets generated, mostly to be able to
     work-around C++ stuff that bindgen can't handle.

## Ahead of time

```shell
$ cargo install bindgen
$ bindgen /usr/include/bzlib.h -o bzlib.rs
```

 * Avoids build-time dependencies / slower build times.
   * Specially in debug mode, bindgen is slow, and cargo has no way to compile
     a build-dependency in release mode
     ([cargo/cargo#1359](https://github.com/rust-lang/cargo/issues/1359))

 * Slightly easier to shoot yourself in the foot.

## At build time

```toml
# Cargo.toml
[build-dependencies]
bindgen = "*"
```

```rust
// build.rs
let bindings = bindgen::Builder::default()
    .header("/usr/include/bzlib.h")
    .generate()
    .expect("Unable to generate bindings");

let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
```

```rust
// src/lib.rs
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
```

 * Avoids footguns (handles bindings that differ per system / platform
   / architecture).

   * You _could_ generate bindings ahead of time for every platform and arch
     combination that you want to build for, but that sounds like a pain :)

 * If the C code you're building is in the same repo, no need to manually
   regenerate bindings and such, `cargo build` takes care of it.

## `bindgen` {rocks, sucks}, how can I {make it better, fix it}?

 * Definitely lots of works to do.

   * Better docs.

     * [rust-lang-nursery.github.io/rust-bindgen](https://rust-lang-nursery.github.io/rust-bindgen/)

     * Improvements or clarification to the documentation are always welcome!

   * Little fixes and improvements

     * Fix alignment of incomplete array fields in C.

     * Support for `#[repr(transparent)]`, which was recently stabilized (`\o/`)

   * Bigger accomplishments

     * Support inline functions (using `c2rust`, maybe?).

     * Support virtual C++ methods.

     * Use Clang's C++ APIs instead of libclang to get better diagnostics and
       support more stuff.

 * Issues tagged with `help wanted` (or any open issue, really) are welcome to
   help!

   * If you're blocked on an issue, please ping, I can try to fix ASAP or mentor :)

 * API additions to help out with new use cases are more than welcome!

## Questions?

 * Lots of stuff I have not covered which may or may not be interesting:

   * What's the setup in Firefox?

   * Walk-through through the options, maybe improving documentation while at
     it?
