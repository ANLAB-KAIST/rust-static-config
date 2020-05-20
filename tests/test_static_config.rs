extern crate static_config;

use std::convert::TryInto;

#[test]
fn test_static_args() {
    assert!(true == static_config::config("level1.bool").try_into().unwrap());
    assert!(-128i8 == static_config::config("level1.i8").try_into().unwrap());
    assert!(-32768i16 == static_config::config("level1.i16").try_into().unwrap());
    assert!(-2147483648i32 == static_config::config("level1.i32").try_into().unwrap());
    assert!(-9223372036854775808i64 == static_config::config("level1.i64").try_into().unwrap());
    assert!("string value with escape \"!!\""
        .eq(static_config::config("level1.string").try_into().unwrap()));

    assert!(
        false
            == static_config::config("level1.level2.bool")
                .try_into()
                .unwrap()
    );
    assert!(
        255u8
            == static_config::config("level1.level2.u8")
                .try_into()
                .unwrap()
    );
    assert!(
        65535u16
            == static_config::config("level1.level2.u16")
                .try_into()
                .unwrap()
    );
    assert!(
        4294967295u32
            == static_config::config("level1.level2.u32")
                .try_into()
                .unwrap()
    );
    assert!(
        9223372036854775807u64
            == static_config::config("level1.level2.u64")
                .try_into()
                .unwrap()
    );
    assert!(
        "string value with escape \"\\\"".eq(static_config::config("level1.level2.string")
            .try_into()
            .unwrap())
    );

    let _array: [u8; static_config::CONST_USIZE.LEVEL1_LEVEL2_U8];

    assert!(255usize == static_config::CONST_USIZE.LEVEL1_LEVEL2_U8);
    assert!(65535usize == static_config::CONST_USIZE.LEVEL1_LEVEL2_U16);
    assert!(4294967295usize == static_config::CONST_USIZE.LEVEL1_LEVEL2_U32);
    assert!(9223372036854775807usize == static_config::CONST_USIZE.LEVEL1_LEVEL2_U64);
}
