extern crate toml;
use std::collections::LinkedList;
use std::convert::TryFrom;
use std::env;
use std::fs::*;
use std::io::*;
use std::path::*;

fn cpu_count(out_path: &PathBuf) {
    let cpu_count = std::env::var("SYSTEM_CPU_COUNT")
        .map(|cpu_count_string| -> usize { cpu_count_string.parse().unwrap_or(num_cpus::get()) })
        .unwrap_or(num_cpus::get());
    println!("cargo:rerun-if-env-changed=SYSTEM_CPU_COUNT");

    let mut target = File::create(out_path.join("cpu_count.rs")).unwrap();

    target
        .write_fmt(format_args!("pub const CPU_COUNT: usize = {};", cpu_count))
        .ok();
}

fn get_root_dir() -> PathBuf {
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

    return out_dir.to_path_buf();
}

fn static_config(project_path: &PathBuf, cargo_root_path: &PathBuf, out_path: &PathBuf) {
    let config_path = std::env::var("STATIC_TOML_PATH")
        .map(PathBuf::from)
        .unwrap_or(cargo_root_path.join("static_config.toml"));

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
        loop {
            let current;
            let prefix;
            if let Some((_prefix, _current)) = queue.pop_front() {
                current = _current;
                prefix = _prefix;
            } else {
                break;
            }

            match current {
                toml::Value::String(val) => {
                    match_string += &format!(
                        "\n\"{}\" => ParamType::STRING(\"{}\"),",
                        prefix,
                        val.replace("\\", "\\\\").replace("\"", "\\\"")
                    );
                }
                toml::Value::Integer(val) => loop {
                    if let Some(target) = usize::try_from(val).ok() {
                        let const_name = prefix.replace(".", "_").replace(" ", "_").to_uppercase();
                        usize_def_string += &format!("\npub {}: usize,", const_name);
                        usize_val_string += &format!("\n{}: {},", const_name, target);
                    }

                    if let Some(target) = u8::try_from(val).ok() {
                        match_string += &format!("\n\"{}\" => ParamType::U8({}),", prefix, target);
                        break;
                    }
                    if let Some(target) = u16::try_from(val).ok() {
                        match_string += &format!("\n\"{}\" => ParamType::U16({}),", prefix, target);
                        break;
                    }
                    if let Some(target) = u32::try_from(val).ok() {
                        match_string += &format!("\n\"{}\" => ParamType::U32({}),", prefix, target);
                        break;
                    }
                    if let Some(target) = u64::try_from(val).ok() {
                        match_string += &format!("\n\"{}\" => ParamType::U64({}),", prefix, target);
                        break;
                    }
                    if let Some(target) = u128::try_from(val).ok() {
                        match_string +=
                            &format!("\n\"{}\" => ParamType::U128({}),", prefix, target);
                        break;
                    }
                    if let Some(target) = i8::try_from(val).ok() {
                        match_string += &format!("\n\"{}\" => ParamType::I8({}),", prefix, target);
                        break;
                    }
                    if let Some(target) = i16::try_from(val).ok() {
                        match_string += &format!("\n\"{}\" => ParamType::I16({}),", prefix, target);
                        break;
                    }
                    if let Some(target) = i32::try_from(val).ok() {
                        match_string += &format!("\n\"{}\" => ParamType::I32({}),", prefix, target);
                        break;
                    }
                    if let Some(target) = i64::try_from(val).ok() {
                        match_string += &format!("\n\"{}\" => ParamType::I64({}),", prefix, target);
                        break;
                    }
                    if let Some(target) = i128::try_from(val).ok() {
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
                        let new_prefix = if prefix.len() == 0 {
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
    let cargo_root_path = get_root_dir();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap())
        .canonicalize()
        .unwrap();

    cpu_count(&out_path);
    static_config(&project_path, &cargo_root_path, &out_path);
}
