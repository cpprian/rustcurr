# rustcurr
A command-line tool written in Rust to converts user amounts between different curriencies using real-time exchange rate data from ExchangeRate-API

## Installation & Configuration
1. Clone the repository
2. Add .env file in the root directory of the project with like in .env.example
   1. Get your API key from [ExchangeRate-API](https://www.exchangerate-api.com/)
   2. Add your API key to the .env file
   3. You can also change cache file path in .env file
3. Run `cargo build --release` in the root directory of the project
4. The binary will be located at `target/release/rustcurr`
5. Run `./rustcurr --help` to see the available commands

## Usage


## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
