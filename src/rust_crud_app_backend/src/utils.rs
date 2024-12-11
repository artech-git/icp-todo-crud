use ic_rand::rng::RandomNumberGenerator;

// method for generating random string as uid
pub fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

    let mut str_result = "".to_string();
    let mut rand_gen: RandomNumberGenerator<usize> = ic_rand::rng::RandomNumberGenerator::new();
    for _ in 0..length {
        let rand = rand_gen.next();
        let idx = (rand) % CHARSET.len();
        str_result.push(CHARSET[idx] as char);
    }
    str_result
}
