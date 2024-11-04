pub fn env_var(name: &str) -> String {
    std::env::var(name).expect(&format!("{} not set", name))
}
