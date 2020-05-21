use test_lib_link::get_new_val;

#[test]
fn test_new_val() {
    assert!(
        "1ce50a37".eq(get_new_val())
    );
}