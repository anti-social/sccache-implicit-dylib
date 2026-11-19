# sccache breaks static linking of libstdc++ on musl

A binary linked from a crate built through sccache crashes with SIGSEGV
before `main`, in a C++ static initializer. The same build without sccache
works. The cache contents don't matter: it fails on a cache miss too.

## Reproduce

Requires [cross](https://github.com/cross-rs/cross). `Containerfile` adds
sccache 0.19.0 to the cross-rs `x86_64-unknown-linux-musl` image and sets
`RUSTC_WRAPPER=sccache`.

```sh
# with sccache: crashes
cross build --release --target x86_64-unknown-linux-musl
./target/x86_64-unknown-linux-musl/release/sccache-fail
# Segmentation fault (exit 139)

cargo clean

# without sccache: works
RUSTC_WRAPPER= cross build --release --target x86_64-unknown-linux-musl
./target/x86_64-unknown-linux-musl/release/sccache-fail
# two = 2
```

## What it does

`dummy-sys` compiles `dummy.cpp`, a C++ library with a static
`std::unordered_map<std::string, int>`, into `libdummy.a`. Its build script
links it the way the `cc` crate does by default:

```
cargo:rustc-link-lib=static=dummy
cargo:rustc-link-lib=stdc++
```

## Cause

sccache parses `-l stdc++` and writes it back as `-l dylib=stdc++`
([src/compiler/rust.rs#L1262](https://github.com/mozilla/sccache/blob/43c90af1a6e4821bde82ff79f8c4099010777f98/src/compiler/rust.rs#L1262)):

```rust
// If no kind is specified, the default is dylib.
(name, None) => ("dylib".to_owned(), name),
```

That assumption is wrong. For rustc, a library without a kind is not the same
as `dylib`. On a target that links statically by default, such as
`x86_64-unknown-linux-musl`, rustc links it statically. An explicit `dylib`
forces dynamic linking. rustc records the kind in the crate's metadata (the
rlib's `lib.rmeta` is the only member that differs). The final static binary
then treats libstdc++ as a shared library, its references are left
unrelocated, and the static initializer jumps to an invalid address.

sccache also rewrites `--crate-type lib` to `rlib` and splits `--flag=value`
arguments, but those are equivalent.

## Workaround

Give the library an explicit kind, so sccache has nothing to rewrite. For
crates using `cc`:

```sh
CXXSTDLIB_x86_64_unknown_linux_musl=static:-bundle=stdc++
```

(`static=stdc++` fails: rustc then needs to find `libstdc++.a` itself to
bundle it.)
