extern crate num_cpus;
extern crate static_config;

#[test]
fn test_cpu_count() {
    assert_eq!(num_cpus::get(), static_config::CPU_COUNT);
    println!("CPU Count: {}", static_config::CPU_COUNT);
}
