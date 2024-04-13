
# WTHR

wthr is a command-line tool for fetching weather information based on IP location built with rust.

## Features

- Retrieve weather information for your current location or any specified IP address.
- Display temperature, maximum temperature, and minimum temperature.

## Installation

1. Make sure you have Rust and Cargo installed.
2. Clone this repository to your local machine:

   ```sh
   git clone https://github.com/mahmoudalnkeeb/wthr.git
   ```

3. Open project directory:

   ```sh
   cd wthr
   ```

4. Build the project using Cargo:

   ```sh
   cargo build --release
   ```

5. Run the executable:

   ```sh
   ./target/release/wthr
   ```

6. Add it to Enviroment Variables (PATH) to access it from anywhere.
    
    ```sh
    export PATH="$PATH:/path/to/executable/directory"
    ```

## Usage

```
wthr [OPTIONS]

OPTIONS:
  -i, --ip <IP_ADDRESS>    Sets the IP address to use for location lookup
  -h, --help               Prints help information
  -V, --version            Prints version information
```

### Examples

- Fetch weather information for your current location:

  ```sh
  wthr
  ```

- Fetch weather information for a specific IP address:

  ```sh
  wthr --ip 123.456.789.012
  ```

## TODOs

1. [ ] Allow users to specify the location by city name or ZIP code for more flexibility.

2. [ ] Enhance the CLI interface with better formatting and error messages for improved user experience.

3. [ ] Implement robust error handling to gracefully handle network failures and API errors.
