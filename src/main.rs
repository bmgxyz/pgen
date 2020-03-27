mod utils;

use clap::{App};

fn main() {
    // set up command line arguments
    let mut app = App::new("pgen")
                    .version("0.2.0")
                    .author("Bradley Gannon <bradley@bradleygannon.com>")
                    .about("Generates random passwords")
                    .args_from_usage("
                        [LENGTH] number of characters in the password");
    let matches = app.clone().get_matches();

    // set password length
    const DEFAULT_PASSWORD_LENGTH : u8 = 16;
    // if LENGTH is not a positive integer, then show help and quit
    let password_len = match matches.value_of("LENGTH") {
        Some(length_str) => {
            match length_str.parse::<u8>() {
                Ok(length_int) => {
                    if length_int == 0 {
                        app.print_help()
                            .expect("ERROR: unable to print help");
                        println!("\n\nERROR: LENGTH must be a positive integer");
                        std::process::exit(1);
                    }
                    length_int
                },
                Err(_error) => {
                    app.print_help()
                        .expect("ERROR: unable to print help");
                    println!("LENGTH must be a positive integer");
                    std::process::exit(1);
                }
            }
        },
        None => {
            // if the user didn't specify a LENGTH, use the default
            DEFAULT_PASSWORD_LENGTH
        }
    };

    // keep track of how many characters we've got
    let mut num_chars : u8 = 0;
    while num_chars < password_len {
        // keep printing characters until we have enough
        // TODO give the user the option to have the output hidden on screen
        // (but still copyable)
        print!("{}", utils::get_random_character());
        num_chars += 1;
    }
    println!(); // new line
}
