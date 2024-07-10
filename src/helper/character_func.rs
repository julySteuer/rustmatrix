use rand::seq::SliceRandom;

pub fn get_random_character() -> char {
    *"asdfghjklöäqwertzuiopüyxcvbnmm,.-1234567890ß" // All the valid characters
        .chars()
        .collect::<Vec<char>>()
        .choose(&mut rand::thread_rng()).unwrap_or(&'#')
}