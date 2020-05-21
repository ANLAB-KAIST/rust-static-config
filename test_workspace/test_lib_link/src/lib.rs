use static_config;
use std::convert::TryInto;

pub fn get_new_val() -> &'static str {
    return static_config::config("new.string").try_into().unwrap()
}