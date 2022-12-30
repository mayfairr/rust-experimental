# Password Generator

This is a Rust program that generates random passwords based on a set of user-specified parameters.

## Features

- Generates passwords of user-specified length
- Allows user to specify whether to include uppercase letters, numbers, and symbols in the password
- Provides a debug mode that prints the password length, inclusion of uppercase letters, numbers, and symbols, and the final output password

## Usage

To use the password generator, you can call the `PasswordGenerator::new` function, passing in the following arguments:

- `length`: The length of the password to be generated. This argument must be a positive integer.
- `include_numbers`: A boolean value indicating whether to include numbers in the password.
- `include_symbols`: A boolean value indicating whether to include symbols in the password.
- `debug_enabled`: A boolean value indicating whether to enable debug mode. If debug mode is enabled, the program will print the password length, inclusion of uppercase letters, numbers, and symbols, and the final output password.

Here's an example of how you might use the password generator:

```rust
let password = PasswordGenerator::new(10, true, true, true, true);
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
