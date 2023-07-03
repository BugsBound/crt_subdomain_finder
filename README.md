# CRT Subdomain Finder

This application uses the Certificate Transparency Logs provided by crt.sh to find all subdomains of a specified domain.

## Prerequisites

Ensure that you have the following installed:

- Rust 1.53.0 or above
- Cargo (usually comes with Rust)

You can download Rust from the official website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Getting Started

1. Clone the repository:
    ```bash
    git clone https://github.com/BugsBound/crt_subdomain_finder.git
    ```
2. Navigate into the directory:
    ```bash
    cd crt_subdomain_finder
    ```
3. Build the project:
    ```bash
    cargo build
    ```

## Usage

Run the program with the domain as a command line argument:
```bash
cargo run yourdomain.com
```

The program will query the crt.sh database, using the certificate transparency logs to find all subdomains of the specified domain. If no subdomains are found, the program will print "Not found!".

## Error Handling

The program uses the `anyhow` library for error handling. If there are any issues with the request or the processing of the data, the program will print an error message and exit.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](./LICENSE)
