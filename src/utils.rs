use rand::{thread_rng, Rng};

pub fn get_random_character() -> char {
    // choose a random character between `a` and `z`
    // TODO make character set configurable (both w/ preset and regex)
    let mut rng = thread_rng();
    let random_char : u8 = rng.gen_range(97, 122+1);
    random_char as char
}
