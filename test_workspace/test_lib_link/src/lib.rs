use std::convert::TryInto;

/// Return desired value from static config
pub fn get_new_val() -> &'static str {
    static_config::config("new.string").try_into().unwrap()
}
