use rand::{thread_rng, Rng};

fn get_random_character() -> char {
    // choose a random character between `a` and `z`
    let mut rng = thread_rng();
    let random_char : u8 = rng.gen_range(97, 122+1);
    random_char as char
}

fn main() {
    // set password length
    // TODO command line argument
    let password_len : u8 = 16;

    // keep track of how many characters we've got
    let mut num_chars : u8 = 0;
    while num_chars < password_len {
        // keep printing characters until we have enough
        print!("{}", get_random_character());
        num_chars += 1;
    }
    println!(); // new line
}
