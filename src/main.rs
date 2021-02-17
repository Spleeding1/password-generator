use rand::prelude::*;
use std::io;


/// Character sets.
static LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
static UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static NUMBERS: &str = "1234567890";
static SPECIAL_CHARACTERS: &str = "!@#$%^&*()";

struct GeneratePassword {
    length: u8,
    lowercase: bool,
    uppercase: bool,
    numbers: bool,
    special_characters: bool,
}

impl Default for GeneratePassword {
    fn default() -> Self {
        GeneratePassword {
            length: 4,
            lowercase: false,
            uppercase: false,
            numbers: false,
            special_characters: false,
        }
    }
}

/// Generates a password(String) with the given information.
/// Default is a four-character number.
/// 
/// Example:
/// 
/// let password = GeneratePassword {
///     length: 24,
///     lowercase: true,
///     special_characters: true,
///     ..GeneratePassword::default()
/// }.generate();
/// 
impl GeneratePassword {
    fn generate(&self) -> String {
        
        // String to hold all of the possible characters for the password.
        let mut wanted_characters:String = String::new();
        
        // Used to guarantee that at least one character is used from every set.
        let mut guaranteed: Vec<u8> = Vec::new();
        
        // Add character sets to wanted_characters.
        if self.lowercase {
            wanted_characters.push_str(LOWERCASE);
            guaranteed.push(1);
        }
        if self.uppercase {
            wanted_characters.push_str(UPPERCASE);
            guaranteed.push(2);
        }
        if self.numbers {
            wanted_characters.push_str(NUMBERS);
            guaranteed.push(3);
        }
        if self.special_characters {
            wanted_characters.push_str(SPECIAL_CHARACTERS);
            guaranteed.push(4);
        }

        // Default is numbers if all sets are entered as false.
        let default_set: bool = if !self.lowercase && !self.uppercase && !self.numbers && !self.special_characters {
            wanted_characters.push_str(NUMBERS);
            true
        } else {
            false
        };
        
        // Used to check that all of the guarantees are there.
        let mut complete: bool = false;

        let mut password: String = String::from("");
        
        while !complete {

            // Set length to 4 if < 4 was entered.
            let mut password_length: u8 = if self.length < 4 {
                    4
                } else {
                    self.length
                };

            let mut rng = rand::thread_rng();
            let mut guaranteed_loop: Vec<u8> = guaranteed.clone();

            // Generates guarantees.
            while guaranteed_loop.len() > 0 {
                let mut rand_set = 0;
                if guaranteed_loop.len() > 1 {
                    rand_set = rng.gen_range(0, guaranteed_loop.len() - 1);
                }
                
                let set: u8 = guaranteed_loop[rand_set];
                
                let guaranteed_set: &str = if set == 1 {
                        LOWERCASE
                    } else if set == 2 {
                        UPPERCASE
                    } else if set == 3 {
                        NUMBERS
                    } else if set == 4 {
                        SPECIAL_CHARACTERS
                    } else {
                        ""
                    };

                if guaranteed_set.len() != 0 {
                    let random = rng.gen_range(0, guaranteed_set.len());
                    password.push_str(&guaranteed_set[random..(random + 1)]);
                    password_length = password_length - 1;
                }
                guaranteed_loop.remove(rand_set);
            }
            
            while password_length > 0 {
                let random = rng.gen_range(0, wanted_characters.len());
                password.push_str(&wanted_characters[random..(random + 1)]);
                password_length = password_length - 1;
            }

            // Used to check that all of the guarantees are in the password.
            let mut has_lowercase: bool = false;
            let mut has_uppercase: bool = false;
            let mut has_numbers: bool = false;
            let mut has_special_characters: bool = false;

            for c in password.chars() {
                if LOWERCASE.contains(c) {
                    has_lowercase = true;
                } else if UPPERCASE.contains(c) {
                    has_uppercase = true;
                } else if NUMBERS.contains(c) {
                    has_numbers = true;
                } else if SPECIAL_CHARACTERS.contains(c) {
                    has_special_characters = true;
                } else {
                    continue;
                }

            }

            // Match found characters to wanted characters or default.
            if has_lowercase == self.lowercase && has_uppercase == self.uppercase && has_numbers == self.numbers && has_special_characters == self.special_characters || default_set {
                complete = true;
            }
        }
        password
    }
}

fn read_u8(message: &str) -> u8 {
    let number: u8;
    
    loop {
        let mut input = String::new();
        println!("{}", message);
        io::stdin().read_line(&mut input)
            .expect("Failed to read line!");

        number = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid entry!");
                continue;
            },
        };
        break;
    }
    number
}

fn read_y_n_from_command_line(message: &str) -> bool {
    let input_bool: bool;
    
    loop {
        let mut input = String::new();
        println!("{}", message);
        io::stdin().read_line(&mut input)
            .expect("Failed to read line!");
        if &input.trim().to_ascii_lowercase() == "y" {
            input_bool = true;
            break;
        } else if &input.trim().to_ascii_lowercase() == "n" {
            input_bool = false;
            break;
        } else {
            println!("Invalid entry!");
            continue;
        }
    }
    input_bool
}

fn main() {
    
    loop {
        let lowercase: bool;
        let uppercase: bool;
        let numbers: bool;
        let special_characters: bool;
        
        let all_characters: bool = read_y_n_from_command_line(
            "Would you like to use all characters? (y/n)"
        );
        
        if all_characters {
            lowercase = true;
            uppercase = true;
            numbers = true;
            special_characters = true;
        } else {
            lowercase = read_y_n_from_command_line(
                "Would you like to use lowercase letters? (y/n)"
            );
            uppercase = read_y_n_from_command_line(
                "Would you like to use uppercase letters? (y/n)"
            );
            numbers = read_y_n_from_command_line(
                "Would you like to use numbers? (y/n)"
            );
            special_characters = read_y_n_from_command_line(
                "Would you like to use special characters? (y/n)"
            );
        }
        
        let length: u8 = read_u8(
            "Enter a password length between 4 and 255."
        );
    
        println!("Your new password is:");
        println!("{}", GeneratePassword {
            length: length,
            lowercase: lowercase,
            uppercase: uppercase,
            special_characters: special_characters,
            numbers: numbers,
        }.generate());
        
        let another: bool = read_y_n_from_command_line(
            "Would you like to generate another password? (y/n)"
        );
        
        if !another {
            break;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_password_is_a_string() {
        let password = GeneratePassword {
            ..GeneratePassword::default()
        }.generate();
        assert_eq!(type_of(password), "alloc::string::String");
    }

    #[test]
    fn generated_password_contains_only_numbers_by_default() {
        let password = GeneratePassword {
            length: 60,
            ..GeneratePassword::default()
        }.generate();
        
        for c in password.chars() {
            assert!(NUMBERS.contains(c));
        }
    }

    #[test]
    fn generated_password_is_only_four_character_long_by_default() {
        let password = GeneratePassword {
            ..GeneratePassword::default()
        }.generate();

        assert_eq!(password.len(), 4);
    }

    #[test]
    fn generated_password_is_different_every_time() {
        let password = GeneratePassword { ..GeneratePassword::default() };
        let result1 = password.generate();
        let result2 = password.generate();
        let result3 = password.generate();

        assert_ne!(result1, result2);
        assert_ne!(result2, result3);
        assert_ne!(result1, result3);
    }

    #[test]
    fn generated_password_can_be_different_lengths() {
        let password1 = GeneratePassword {
            length:10,
            ..GeneratePassword::default()
        };
        let result1 = password1.generate();

        let password2 = GeneratePassword {
            length:15,
            ..GeneratePassword::default()
        };
        let result2 = password2.generate();

        let password3 = GeneratePassword {
            length:64,
            ..GeneratePassword::default()
        };
        let result3 = password3.generate();

        assert_eq!(result1.len(), 10);
        assert_eq!(result2.len(), 15);
        assert_eq!(result3.len(), 64);
    }

    #[test]
    fn generated_password_can_be_from_lowercase_only() {
        let password = GeneratePassword {
            length: 60,
            lowercase: true,
            ..GeneratePassword::default()
        };
        let result = password.generate();

        for c in result.chars() {
            assert!(LOWERCASE.contains(c));
        }
    }
    
    #[test]
    fn generated_password_can_be_from_lowercase_and_numbers() {
        let password = GeneratePassword {
            length: 60,
            lowercase: true,
            numbers: true,
            ..GeneratePassword::default()
        }.generate();
        let mut has_numbers = false;
        let mut has_lowercase = false;
        let mut illegal_character = false;

        for c in password.chars() {
            if LOWERCASE.contains(c) {
                has_lowercase = true;
            } else if NUMBERS.contains(c) {
                has_numbers = true;
            } else {
                illegal_character = true;
            }
        }

        assert_eq!(
            (has_lowercase, has_numbers, illegal_character),
            (true, true, false)
        );
    }

    #[test]
    fn generated_password_can_be_from_uppercase_only() {
        let password = GeneratePassword {
            length: 60,
            uppercase: true,
            ..GeneratePassword::default()
        }.generate();

        for c in password.chars() {
            assert!(UPPERCASE.contains(c));
        }
    }
    
    #[test]
    fn generated_password_can_be_from_uppercase_and_numbers() {
        let password = GeneratePassword {
            length: 60,
            uppercase: true,
            numbers: true,
            ..GeneratePassword::default()
        }.generate();
        let mut has_numbers = false;
        let mut has_uppercase = false;
        let mut illegal_character = false;

        for c in password.chars() {
            if UPPERCASE.contains(c) {
                has_uppercase = true;
            } else if NUMBERS.contains(c) {
                has_numbers = true;
            } else {
                illegal_character = true;
            }
        }

        assert_eq!(
            (has_uppercase, has_numbers, illegal_character),
            (true, true, false)
        );
    }
    
    #[test]
    fn generated_password_can_be_from_uppercase_and_lowercase() {
        let password = GeneratePassword {
            length: 60,
            lowercase: true,
            uppercase: true,
            ..GeneratePassword::default()
        }.generate();
        let mut has_uppercase = false;
        let mut has_lowercase = false;
        let mut illegal_character = false;

        for c in password.chars() {
            if LOWERCASE.contains(c) {
                has_lowercase = true;
            } else if UPPERCASE.contains(c) {
                has_uppercase = true;
            } else {
                illegal_character = true;
            }
        }

        assert_eq!(
            (has_lowercase, has_uppercase, illegal_character),
            (true, true, false)
        );
    }

    #[test]
    fn generated_password_can_contain_lowercase_uppercase_and_numbers() {
        let password = GeneratePassword {
            length: 60,
            lowercase: true,
            uppercase: true,
            numbers: true,
            ..GeneratePassword::default()
        }.generate();
        let mut has_uppercase = false;
        let mut has_lowercase = false;
        let mut has_numbers = false;
        let mut illegal_character = false;

        for c in password.chars() {
            if LOWERCASE.contains(c) {
                has_lowercase = true;
            } else if UPPERCASE.contains(c) {
                has_uppercase = true;
            } else if NUMBERS.contains(c) {
                has_numbers = true;
            } else {
                illegal_character = true;
            }
        }

        assert_eq!(
            (has_lowercase, has_uppercase, has_numbers, illegal_character),
            (true, true, true, false)
        );
    }

    #[test]
    fn generated_password_can_be_from_special_characters() {
        let password = GeneratePassword {
            length: 60,
            special_characters: true,
            ..GeneratePassword::default()
        }.generate();
        let mut has_special_characters = false;
        let mut illegal_character = false;

        for c in password.chars() {
            if SPECIAL_CHARACTERS.contains(c) {
                has_special_characters = true;
            } else {
                illegal_character = true;
            }
        }

        assert_eq!(
            (has_special_characters, illegal_character),
            (true, false)
        );
    }

    #[test]
    fn generated_password_can_be_from_numbers_and_special_characters() {
        let password = GeneratePassword {
            length: 60,
            numbers: true,
            special_characters: true,
            ..GeneratePassword::default()
        }.generate();
        let mut has_numbers = false;
        let mut has_special_characters = false;
        let mut illegal_character = false;

        for c in password.chars() {
            if NUMBERS.contains(c) {
                has_numbers = true;
            } else if SPECIAL_CHARACTERS.contains(c) {
                has_special_characters = true;
            } else {
                illegal_character = true;
            }
        }

        assert_eq!(
            (has_numbers, has_special_characters, illegal_character),
            (true, true, false)
        );
    }

    #[test]
    fn generated_password_can_be_from_lowercase_and_special_characters() {
        let password = GeneratePassword {
            length: 60,
            lowercase: true,
            special_characters: true,
            ..GeneratePassword::default()
        }.generate();
        let mut has_lowercase = false;
        let mut has_special_characters = false;
        let mut illegal_character = false;

        for c in password.chars() {
            if LOWERCASE.contains(c) {
                has_lowercase = true;
            } else if SPECIAL_CHARACTERS.contains(c) {
                has_special_characters = true;
            } else {
                illegal_character = true;
            }
        }

        assert_eq!(
            (has_lowercase, has_special_characters, illegal_character),
            (true, true, false)
        );
    }

    #[test]
    fn generated_password_can_be_from_uppercase_and_special_characters() {
        let password = GeneratePassword {
            length: 60,
            uppercase: true,
            special_characters: true,
            ..GeneratePassword::default()
        }.generate();
        let mut has_uppercase = false;
        let mut has_special_characters = false;
        let mut illegal_character = false;

        for c in password.chars() {
            if UPPERCASE.contains(c) {
                has_uppercase = true;
            } else if SPECIAL_CHARACTERS.contains(c) {
                has_special_characters = true;
            } else {
                illegal_character = true;
            }
        }

        assert_eq!(
            (has_uppercase, has_special_characters, illegal_character),
            (true, true, false)
        );
    }

    #[test]
    fn generated_password_can_be_from_all_character_sets() {
        let password = GeneratePassword {
            length: 60,
            lowercase: true,
            uppercase: true,
            numbers: true,
            special_characters: true,
            ..GeneratePassword::default()
        }.generate();
        let mut has_lowercase = false;
        let mut has_uppercase = false;
        let mut has_numbers = false;
        let mut has_special_characters = false;
        let mut illegal_character = false;

        for c in password.chars() {
            if LOWERCASE.contains(c) {
                has_lowercase = true;
            } else if UPPERCASE.contains(c) {
                has_uppercase = true;
            } else if NUMBERS.contains(c) {
                has_numbers = true;
            } else if SPECIAL_CHARACTERS.contains(c) {
                has_special_characters = true;
            } else {
                illegal_character = true;
            }
        }

        assert_eq!(
            (has_lowercase, has_uppercase, has_numbers, has_special_characters, illegal_character),
            (true, true, true, true, false)
        );
    }

    #[test]
    fn generated_password_can_be_from_numbers_lowercase_and_special_characters() {
        let password = GeneratePassword {
            length: 60,
            lowercase: true,
            numbers: true,
            special_characters: true,
            ..GeneratePassword::default()
        }.generate();

        let mut has_lowercase = false;
        let mut has_numbers = false;
        let mut has_special_characters = false;
        let mut illegal_character = false;

        for c in password.chars() {
            if LOWERCASE.contains(c) {
                has_lowercase = true;
            } else if NUMBERS.contains(c) {
                has_numbers = true;
            } else if SPECIAL_CHARACTERS.contains(c) {
                has_special_characters = true;
            } else {
                illegal_character = true;
            }
        }

        assert_eq!(
            (has_lowercase, has_numbers, has_special_characters, illegal_character),
            (true, true, true, false)
        );
    }

    #[test]
    fn generated_password_can_be_from_numbers_uppercase_and_special_characters() {
        let password = GeneratePassword {
            length: 60,
            uppercase: true,
            numbers: true,
            special_characters: true,
            ..GeneratePassword::default()
        }.generate();

        let mut has_uppercase = false;
        let mut has_numbers = false;
        let mut has_special_characters = false;
        let mut illegal_character = false;

        for c in password.chars() {
            if UPPERCASE.contains(c) {
                has_uppercase = true;
            } else if NUMBERS.contains(c) {
                has_numbers = true;
            } else if SPECIAL_CHARACTERS.contains(c) {
                has_special_characters = true;
            } else {
                illegal_character = true;
            }
        }

        assert_eq!(
            (has_uppercase, has_numbers, has_special_characters, illegal_character),
            (true, true,  true, false)
        );
    }
    
    #[test]
    fn generated_password_can_be_from_uppercase_lowercase_and_special_characters() {
        let password = GeneratePassword {
            length: 60,
            lowercase: true,
            uppercase: true,
            special_characters: true,
            ..GeneratePassword::default()
        }.generate();

        let mut has_lowercase = false;
        let mut has_uppercase = false;
        let mut has_special_characters = false;
        let mut illegal_character = false;

        for c in password.chars() {
            if LOWERCASE.contains(c) {
                has_lowercase = true;
            } else if UPPERCASE.contains(c) {
                has_uppercase = true;
            } else if SPECIAL_CHARACTERS.contains(c) {
                has_special_characters = true;
            } else {
                illegal_character = true;
            }
        }

        assert_eq!(
            (has_lowercase, has_uppercase, has_special_characters, illegal_character),
            (true, true, true, false)
        );
    }

    #[test]
    fn generate_password_will_always_contain_at_least_one_character_from_each_set() {
        let mut counter = 20;
        while counter > 0 {
            let password = GeneratePassword {
                numbers: true,
                lowercase: true,
                uppercase: true,
                special_characters: true,
                ..GeneratePassword::default()
            }.generate();

            let mut has_lowercase = false;
            let mut has_uppercase = false;
            let mut has_numbers = false;
            let mut has_special_characters = false;

            for c in password.chars() {
                if LOWERCASE.contains(c) {
                    has_lowercase = true;
                } else if UPPERCASE.contains(c) {
                    has_uppercase = true;
                } else if NUMBERS.contains(c) {
                    has_numbers = true;
                } else if SPECIAL_CHARACTERS.contains(c) {
                    has_special_characters = true;
                } else {
                    continue;
                }
            }

            assert_eq!(
                (has_lowercase, has_uppercase, has_numbers, has_special_characters),
                (true, true, true, true)
            );
            if !has_lowercase && !has_uppercase && !has_numbers && !has_special_characters {
                break;
            }
            counter -= 1;
        }
    }

    #[test]
    fn generate_password_minimum_length_is_four() {
        let password = GeneratePassword {
            length: 3,
            ..GeneratePassword::default()
        }.generate();

        assert_eq!(password.len(), 4);
    }
}