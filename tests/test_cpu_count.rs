extern crate num_cpus;
extern crate static_config;

#[test]
fn test_cpu_count() {
    let mut cpu_count_env = 0usize;
    for (env, val) in std::env::vars() {
        if env == "SYSTEM_CPU_COUNT" {
            if let Ok(_cpu_count) = val.parse() {
                cpu_count_env = _cpu_count;
            }
        }
    }
    if cpu_count_env == 0 {
        assert_eq!(num_cpus::get(), static_config::CPU_COUNT);
    } else {
        assert_eq!(cpu_count_env, static_config::CPU_COUNT);
    }
    println!("CPU Count: {}", static_config::CPU_COUNT);
}
