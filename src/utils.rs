use rand::{thread_rng, Rng};

// TODO docstring?
pub fn get_random_character() -> char {
    // choose a random character between `a` and `z`
    // TODO make character set configurable (both w/ preset and regex)
    let mut rng = thread_rng();
    let random_char : u8 = rng.gen_range(97, 122+1);
    random_char as char
}

#[cfg(test)]
mod tests {
    use super::*;

    // BEGIN HELPER FUNCTIONS

    // gets 256 characters and counts how many of each were returned
    fn get_char_counts(n : u32) -> Vec<u8> {
        let mut char_counts = vec![0; 26];
        let mut random_char : u8;
        for _ in 0..n {
            random_char = get_random_character() as u8;
            char_counts[(random_char - 97) as usize] += 1;
        }
        char_counts
    }

    // TODO docstring?
    fn chi_squared_uniform(input : Vec<u8>, expected_value : f32) -> f32 {
        let num_categories = input.len() as f32;
        let mut statistic = 0.0;
        for i in 0..num_categories as u8 {
            // residual squared over expected
            statistic += (input[i as usize] as f32 - expected_value).powf(2.0) /
                expected_value;
        }
        statistic
    }

    // END HELPER FUNCTIONS
    // BEGIN TESTS

    #[test]
    fn chi_squared_uniform_ones() {
        let all_ones : Vec<u8> = vec![1; 50];
        let chi_squared_result = chi_squared_uniform(all_ones, 1.0);
        assert_eq!(chi_squared_result, 0.0);
    }

    #[test]
    fn chi_squared_uniform_static_random() {
        let random_data : Vec<u8> = vec![16, 215, 16, 14, 41, 39, 167, 121, 31,
        109, 180, 138, 34, 131, 40, 15, 214, 178, 49, 240, 187, 42, 62, 53, 23,
        130, 204, 19, 97, 57, 210, 228, 133, 208, 12, 53, 224, 0, 157, 114, 68,
        116, 229, 138, 130, 117, 205, 171, 117, 14];
        let chi_squared_result = chi_squared_uniform(random_data, 127.5);
        assert_eq!((chi_squared_result * 100.0).round() / 100.0, 2289.59);
    }

    #[test]
    fn get_random_character_single() {
        // get_random_character should return a single lowercase letter
        let random_char : u8 = get_random_character() as u8;
        assert!((97..122+1).contains(&random_char),
            "get_random_character() returned {}, which is not in [97, 122]",
            random_char);
    }

    #[test]
    fn get_random_character_coverage() {
        // get_random_character should return any of the 26 lowercase letters
        let char_counts = get_char_counts(256);
        for i in 0..26 {
            if char_counts[i] == 0 {
                panic!("get_random_character() did not return any characters \
                    with value {}", i + 97);
            }
        }
    }

    #[test]
    fn get_random_character_distribution() {
        // get_random_character should return a fair distribution
        let num_chars : u32 = 2600;
        let char_counts = get_char_counts(num_chars);
        // for k=25, p=0.05 => chi squared = 37.652
        let chi_squared_result = chi_squared_uniform(char_counts, 100.0);
        assert!(chi_squared_result < 37.652,
            "chi_squared(char_counts) returned {} > 37.652",
            chi_squared_result);
    }

    // END TESTS
}
