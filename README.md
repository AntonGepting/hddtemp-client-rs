# hddtemp_client

[![Build Status](https://github.com/AntonGepting/hddtemp-client-rs/actions/workflows/actions.yml/badge.svg)](https://github.com/AntonGepting/hddtemp-client-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/hddtemp_client.svg)](https://crates.io/crates/hddtemp_client)
[![Documentation](https://docs.rs/hddtemp_client/badge.svg)](https://docs.rs/hddtemp_client)


## Description

`hddtemp_client` is a library for communication with
[hddtemp](https://github.com/guzu/hddtemp/) written in
[Rust](https://www.rust-lang.org/) programming language.  The crate
documentation can be found on the [docs.rs](https://docs.rs/hddtemp_client)
page.


## Usage


1. Add a dependency in your `Cargo.toml`. Versions below `1.0.0` are
    mostly for development and testing purposes (use them in your projects on
    your own risk, further versions may have different API).

    ```text
    [dependencies]
    hddtemp_client = "1.0.0"
    ```

2. Add extern crate in your source file.
    ```
    extern crate hddtemp_client;
    ```

3. Use it's functions

    ### Example 1

    ```
    extern crate hddtemp_client;

    use hddtemp_client::HDDTempClient;

    let devices = HDDTempClient::get("localhost:7634", None, None).unwrap();
    ```

## Testing


## License

`hddtemp_client` library is licensed under the MIT license. Please read the
[license file](LICENSE.md) in the repository for more information.


## See also

- [Rust programming language](https://www.rust-lang.org/)
- [crates.io](https://www.crates.io/)
- [docs.rs](https://www.docs.rs/)
- [hddtemp service](https://github.com/guzu/hddtemp/)