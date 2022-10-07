use rand::distributions::Alphanumeric;
use rand::Rng;

pub(crate) fn get_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}