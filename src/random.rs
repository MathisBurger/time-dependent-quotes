

pub(crate) fn get_random_string(length: i32) -> String {
    rand::thread_rng()
        .gen_acscii_chars()
        .take(length)
        .collect::<String>()
}