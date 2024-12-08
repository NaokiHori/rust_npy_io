# `rust_npy_io`

A Rust library for reading and writing [`NPY`](https://numpy.org/doc/stable/reference/generated/numpy.lib.format.html) files.

## Dependency

- [`regex`](https://docs.rs/regex/latest/regex/)

## Example

Refer to the example code in [`main.rs`](https://github.com/NaokiHori/rust_npy_io/blob/main/src/main.rs).

## Caveat

This crate is essentially a Rust implementation of my [`existing library`](https://github.com/NaokiHori/SimpleNpyIO) in C, and is intended to be for personal-use.

Currently, it supports only simple N-dimensional arrays and does not handle all possible `NPY` data structures (e.g., nested arrays).

## Reference

- [`format.py`](https://github.com/numpy/numpy/blob/main/numpy/lib/format.py)
- [`npy.lib.format`](https://numpy.org/doc/stable/reference/generated/numpy.lib.format.html)

