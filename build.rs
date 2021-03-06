use std::collections::LinkedList;
use std::convert::TryFrom;
use std::env;
use std::fs::*;
use std::io::*;
use std::path::*;

/// Extract CPU count from `num_cpus` crate.
fn cpu_count(out_path: &PathBuf) {
    let cpu_count = num_cpus::get();
    let mut target = File::create(out_path.join("cpu_count.rs")).unwrap();

    target
        .write_fmt(format_args!("pub const CPU_COUNT: usize = {};", cpu_count))
        .ok();
}

/// Root crate dir is not given as cargo env.
/// (https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates)
///
/// Related issue: https://github.com/rust-lang/cargo/issues/3946 This function is modified from
/// https://github.com/mitsuhiko/insta/blob/b113499249584cb650150d2d01ed96ee66db6b30/src/runtime.rs#L67-L88
///
/// However, this cannot capture the root directory when this library is called as an external
/// library.
///
/// Even when a crate is built as an external library, it shares the same `target` directory as its
/// parent crate.  Thus, we try to find the root directory by tracking parent directories from
/// `OUT_DIR` which contains `target` directory as its child.
fn get_cargo_workspace() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut out_dir = Path::new(&out_dir);
    assert!(out_dir.is_dir());
    loop {
        let is_end = out_dir.file_name().unwrap() == "target";

        out_dir = Path::new(out_dir.parent().unwrap());

        if is_end {
            break;
        }
    }

    out_dir.to_path_buf()
}

/// Read `static_config.toml` and generate embedded source file.
fn static_config(project_path: &PathBuf, cargo_root_path: &PathBuf, out_path: &PathBuf) {
    let config_path = cargo_root_path.join("static_config.toml");

    let mut usize_def_string = String::new();
    let mut usize_val_string = String::new();
    let mut match_string = String::new();

    let template_path = project_path
        .join("template")
        .join("static_config.rs.template");
    let target_path = out_path.join("static_config.rs");
    println!("cargo:rerun-if-changed={}", template_path.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", config_path.to_str().unwrap());

    let mut template = File::open(template_path).unwrap();
    let mut target = File::create(target_path).unwrap();

    if config_path.exists() {
        let mut config_file = File::open(config_path).unwrap();

        let mut config_string = String::new();

        config_file.read_to_string(&mut config_string).ok();
        let config = config_string.parse::<toml::Value>().unwrap();

        let mut queue = LinkedList::new();
        queue.push_back((String::from(""), config));
        while let Some((prefix, current)) = queue.pop_front() {
            match current {
                toml::Value::String(val) => {
                    match_string += &format!(
                        "\n\"{}\" => ParamType::STRING(\"{}\"),",
                        prefix,
                        val.replace("\\", "\\\\").replace("\"", "\\\"")
                    );
                }
                toml::Value::Integer(val) => loop {
                    if let Ok(target) = usize::try_from(val) {
                        let const_name = prefix.replace(".", "_").replace(" ", "_").to_uppercase();
                        usize_def_string += &format!("\npub {}: usize,", const_name);
                        usize_val_string += &format!("\n{}: {},", const_name, target);
                    }

                    if let Ok(target) = u8::try_from(val) {
                        match_string += &format!("\n\"{}\" => ParamType::U8({}),", prefix, target);
                        break;
                    }
                    if let Ok(target) = u16::try_from(val) {
                        match_string += &format!("\n\"{}\" => ParamType::U16({}),", prefix, target);
                        break;
                    }
                    if let Ok(target) = u32::try_from(val) {
                        match_string += &format!("\n\"{}\" => ParamType::U32({}),", prefix, target);
                        break;
                    }
                    if let Ok(target) = u64::try_from(val) {
                        match_string += &format!("\n\"{}\" => ParamType::U64({}),", prefix, target);
                        break;
                    }
                    if let Ok(target) = u128::try_from(val) {
                        match_string +=
                            &format!("\n\"{}\" => ParamType::U128({}),", prefix, target);
                        break;
                    }
                    if let Ok(target) = i8::try_from(val) {
                        match_string += &format!("\n\"{}\" => ParamType::I8({}),", prefix, target);
                        break;
                    }
                    if let Ok(target) = i16::try_from(val) {
                        match_string += &format!("\n\"{}\" => ParamType::I16({}),", prefix, target);
                        break;
                    }
                    if let Ok(target) = i32::try_from(val) {
                        match_string += &format!("\n\"{}\" => ParamType::I32({}),", prefix, target);
                        break;
                    }
                    if let Ok(target) = i64::try_from(val) {
                        match_string += &format!("\n\"{}\" => ParamType::I64({}),", prefix, target);
                        break;
                    }
                    if let Ok(target) = i128::try_from(val) {
                        match_string +=
                            &format!("\n\"{}\" => ParamType::I128({}),", prefix, target);
                        break;
                    }

                    panic!("Cannot find suitable integer type")
                },
                toml::Value::Float(val) => {
                    match_string += &format!("\n\"{}\" => ParamType::FLOAT({}),", prefix, val);
                }
                toml::Value::Boolean(val) => {
                    match_string += &format!("\n\"{}\" => ParamType::BOOL({}),", prefix, val);
                }
                toml::Value::Datetime(_) | toml::Value::Array(_) => {
                    panic!("Datetime and arrays are not available in static toml config")
                }
                toml::Value::Table(val) => {
                    for (k, v) in val {
                        let new_prefix = if prefix.is_empty() {
                            k
                        } else {
                            format!("{}.{}", prefix, k)
                        };
                        queue.push_back((new_prefix, v));
                    }
                }
            }
        }
    }

    let mut template_string = String::new();
    template.read_to_string(&mut template_string).ok();
    template_string = template_string.replace("%%MATCH_STRING%%", &match_string);
    template_string = template_string.replace("%%USIZE_DEF%%", &usize_def_string);
    template_string = template_string.replace("%%USIZE_VAL%%", &usize_val_string);
    target.write_fmt(format_args!("{}", template_string)).ok();
}

fn main() {
    let project_path = PathBuf::from(".").canonicalize().unwrap();
    let cargo_root_path = get_cargo_workspace();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap())
        .canonicalize()
        .unwrap();

    cpu_count(&out_path);
    static_config(&project_path, &cargo_root_path, &out_path);
}
