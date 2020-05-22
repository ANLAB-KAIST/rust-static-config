# Static Config

[![Build Status](https://jenkins.kaist.ac.kr/buildStatus/icon?job=ANLAB-KAIST%2Frust-static-config%2Fmaster)](https://jenkins.kaist.ac.kr/job/ANLAB-KAIST/job/rust-static-config/job/master/)

## What is this?

If configuration values are obtained from `env` or `toml`, Rust compiler considers those values as dynamic values.
Thus, Rust compiler cannot optimized them as constant values.
Furthermore, a fixed-length array only accepts integers with const types.
A constant with non-const type (e.g. `let val = 3;`) cannot be used.

`static-config` reads the content of `static_config.toml` at the crate root and embedded the values as a static rust source file.
Values from `static_config.toml` can be accessed as pre-defined constants.

## How to use

1. Add this library to your dependency list.
1. Put `static_config.toml` to the root of your crate/workspace (where you type `cargo` commands).
1. (Warning) This library will not work if the target directory is changed via env vars or cargo options.
1. Values from the TOML file is obtained through `static_config::config` API.
1. To use integer values as length of a fixed-length array, all integer values within the system's `usize` range are also defined in `static_config::CONST_USIZE`.

## Example of `static_config.toml`

```{.toml}
[level1]
string = "string value with escape \"!!\""
bool = true
i8 = -128
i16 = -32768
i32 = -2147483648
i64 = -9223372036854775808
# i128 is not yet supported in toml 0.5

[level1.level2]
string = "string value with escape \"\\\""
bool = false
u8 = 255
u16 = 65535
u32 = 4294967295
u64 = 9223372036854775807 # Max int type is i64
# u128 is not yet supported in toml 0.5
```

The following command is fully optimized-out into a const value.

```{.rs}
assert!(true == static_config::config("level1.bool").try_into().unwrap());
```

If you want a `const usize` type to be used in array length:

```{.rs}
assert!(255usize == static_config::CONST_USIZE.LEVEL1_LEVEL2_U8);
```

## `static_config::CPU_COUNT`

`static_config` provides a `CPU_COUNT` constant measured by `num_cpus` crate.
