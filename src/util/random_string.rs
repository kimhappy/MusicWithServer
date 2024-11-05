pub fn random_string(len: usize) -> String {
    use rand::Rng;
    use rand::distributions::Alphanumeric;

    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
