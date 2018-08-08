---
title: rust-bindgen
theme: white
---

`about:emilio`

---

## In a nutshell

 * Generates bindings (struct and function definitions) from C / C++ code.

 * [github.com/rust-lang-nursery/rust-bindgen](https://github.com/rust-lang-nursery/rust-bindgen)

::: notes

Ask about familiarity with bindgen maybe.

Maybe a joke about not being multiple `bindgen`s when bindgen was born :)

Remember encouraging people to ask questions and interrupt freely :)

:::

---

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

---

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

---

## C++ works too!*

::: notes

Disclaimer: To some extent, totally not perfect.

:::

---

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

---

```rust
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Foo<T> {
    pub m_member: T,
    pub m_member_ptr: *mut T,
    pub m_member_arr: [T; 1usize],
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct B<T> {
    pub m_member: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
extern "C" {
    #[link_name = "\u{1}_Z3bar3FooIiiE"]
    pub fn bar(foo: Foo<::std::os::raw::c_int>);
}
```

---

### C++ caveats

 * No virtual functions.

 * Templates can get hard.

 * Lots of options to control what's generated.

::: notes

It's generally pretty robust (we use it in Firefox's build, which means a _lot_
of C++ gets parsed), but:

 * No virtual functions ([#27](https://github.com/rust-lang-nursery/rust-bindgen/issues/27)).

   * `bindgen` generates an empty vtable to handle struct layout properly.

   * Probably not hard to handle simple cases (no overloaded virtuals, etc.).

 * Templates can be hard:

   * Bindgen tries to preserve template parameters which generally works, but
     stuff like non-type template parameters, etc makes bindgen output an opaque
     blob for the instantiations of the template.

   * Lots of options to control what gets generated, mostly to be able to
     work-around C++ stuff that bindgen can't handle.

:::

---

# Ways to use bindgen

---

## Ahead of time

---

```shell
$ cargo install bindgen
$ bindgen /usr/include/bzlib.h -o bzlib.rs
```

---

 * Avoids build-time dependencies / slower build times
   ([cargo/cargo#1359](https://github.com/rust-lang/cargo/issues/1359)).

 * Slightly easier to shoot yourself in the foot.

::: notes

Specially in debug mode, bindgen is slow, and cargo has no way to compile
a build-dependency in release mode.

Compiling for multiple targets may get annoying.

:::


---

## At build time

Generating bindings via `build.rs`.

::: notes

This is what we use for Firefox. Supports multiple architectures correctly and
such.

Specially recommended for everything involving non-public APIs or APIs that
don't guarantee binary compatibility / rely on system-dependent stuff for struct
layout / C++.

Avoids footguns (handles bindings that differ per system / platform
/ architecture).

 * You _could_ generate bindings ahead of time for every platform and arch
   combination that you want to build for, but that sounds like a pain :)

If the C code you're building is in the same repo, no need to manually
regenerate bindings and such, `cargo build` takes care of it.

:::

---

```toml
# Cargo.toml
[build-dependencies]
bindgen = "*"
```

---

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

---

```rust
// src/lib.rs
#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
```

---

## `bindgen` {rocks, sucks}, how can I {make it better, fix it}?

TL;DR: Definitely lots of works to do :)

---

### Better docs.

Improvements or clarification to the documentation are always welcome!

 * [docs.rs/bindgen](https://docs.rs/bindgen)
 * [rust-lang-nursery.github.io/rust-bindgen](https://rust-lang-nursery.github.io/rust-bindgen/)

---

### Little fixes and improvements

 * Fix alignment of incomplete array fields in C.

 * API additions.

 * `#[repr(transparent)]`.

 * Everything tagged with <span style="padding: 10px; border-radius: 5px; background: green; color: white; font-weight: bold">help wanted</span>

 * If you're blocked on an issue, please ping, happy to fix or mentor ASAP :).

::: notes

Note that even for issues not tagged with `help wanted` welcome help!

:::

---

### Bigger / funnier projects

 * Support inline functions (using `c2rust`, maybe?).

 * Support virtual C++ methods.

 * Use Clang's C++ APIs.

::: notes

Use Clang's C++ instead of libclang to get better diagnostics and support more
stuff.

:::

---

## Questions?

::: notes

 * Lots of stuff I have not covered which may or may not be interesting:

 * What's the setup in Firefox?

 * Walk-through through the options, maybe improving documentation while at it?
   Probably too boring.

 * Let people know you're happy to talk after the event.

:::

---

# Example

  [imagemagick.org/script/magick-wand.php](https://www.imagemagick.org/script/magick-wand.php)

::: notes

Generate the bindings on stage, then go to the repo to show the finished
program, or show it locally.

 * [github.com/emilio/bindgen-talk](https://github.com/emilio/bindgen-talk)

:::

---

# Thanks!

  * [github.com/emilio/bindgen-talk](https://github.com/emilio/bindgen-talk)
