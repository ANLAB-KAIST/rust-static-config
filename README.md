# Static Config

[![Build Status](https://jenkins.kaist.ac.kr/buildStatus/icon?job=ANLAB-KAIST%2Frust-static-config%2Fmaster)](https://jenkins.kaist.ac.kr/job/ANLAB-KAIST/job/rust-static-config/job/master/)

## What is this?

If configuration values are obtained from `env` or `toml`, Rust compiler considers those values as dynamic values.
Thus, we cannot use the values to configure a fixed-length array's length.

`static-config` reads the content of `static_config.toml` at the crate root and embedded the values as a static rust source file.
Values from `static_config.toml` can be accessed as pre-defined constants.

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
assert!(true == r::static_config::config("level1.bool").try_into().unwrap());
```

If you want a `const usize` type to be used in array length:

```{.rs}
assert!(255usize == r::static_config::CONST_USIZE.LEVEL1_LEVEL2_U8);
```

## `static_config::CPU_COUNT`

`static_config` provides a `CPU_COUNT` constant which is measured at `cargo build`.
