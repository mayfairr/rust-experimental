use rand;
use rand::Rng;

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()_+-=[]{};':,./<>?";

struct PasswordGenerator {
    length: u8,
    include_uppercase: bool,
    include_numbers: bool,
    include_symbols: bool,
    output_password: String,
}

impl PasswordGenerator {
    fn new(length: u8, include_uppercase: bool, include_numbers: bool, include_symbols: bool, debug_enabled: bool) -> String {
        
        let mut gen = PasswordGenerator  {
            length,
            include_uppercase,
            include_numbers,
            include_symbols,
            output_password: String::new(),
        };

        gen.generate_password();

        if debug_enabled {
            gen.print_debug_message();
        }

        return gen.output_password;
    }

    fn generate_password(&mut self) {
       for _ in 0..self.length {
           let mut rng = rand::thread_rng();
           let random_number = rng.gen_range(0..100);

           if random_number < 25 && self.include_uppercase {
               self.output_password.push(self.get_random_uppercase());
           } else if random_number < 50 && self.include_numbers {
               self.output_password.push(self.get_random_number());
           } else if random_number < 75 && self.include_symbols {
               self.output_password.push(self.get_random_symbol());
           } else {
               self.output_password.push(self.get_random_lowercase());
           }
       }
    }

    fn get_random_lowercase(&self) -> char {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..LOWERCASE.len());

        return LOWERCASE[random_number] as char;
    }

    fn get_random_uppercase(&self) -> char {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..UPPERCASE.len());

        return UPPERCASE[random_number] as char;
    }

    fn get_random_number(&self) -> char {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..NUMBERS.len());

        return NUMBERS[random_number] as char;
    }

    fn get_random_symbol(&self) -> char {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..SYMBOLS.len());

        return SYMBOLS[random_number] as char;
    }
   
    fn print_debug_message(&self){
        println!("Length: {}", self.length);
        println!("Include uppercase: {}", self.include_uppercase);
        println!("Include numbers: {}", self.include_numbers);
        println!("Include symbols: {}", self.include_symbols);
        println!("Output password: {}", self.output_password);
    }

}

fn main() {
 let password = PasswordGenerator::new(10, true, true, true, true); 
}

