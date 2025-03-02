<center>

## udp-request

[![](https://img.shields.io/crates/v/udp-request.svg)](https://crates.io/crates/udp-request)
[![](https://img.shields.io/crates/d/udp-request.svg)](https://img.shields.io/crates/d/udp-request.svg)
[![](https://docs.rs/udp-request/badge.svg)](https://docs.rs/udp-request)
[![](https://github.com/ltpp-universe/udp-request/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/udp-request/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/udp-request.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/udp-request/)

[Api Docs](https://docs.rs/udp-request/latest/udp_request/)

> A simple UDP request library for sending and receiving UDP packets, designed to handle network communication in Rust applications.

## Installation

To use this crate, you can run cmd:

```shell
cargo add udp-request
```

## Use

#### Receive Text

```rs
use udp_request::*;
let mut request_builder = RequestBuilder::new()
    .host("127.0.0.1")
    .port(80)
    .build();
request_builder
    .send("udp send".as_bytes())
    .and_then(|response| {
        println!("ResponseTrait => {:?}", response.text());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {:?}", e));
```

#### Receive Binary

```rs
use udp_request::*;
let mut request_builder = RequestBuilder::new()
    .host("127.0.0.1")
    .port(80)
    .build();
request_builder
    .send("udp send".as_bytes())
    .and_then(|response| {
        println!("ResponseTrait => {:?}", response.binary());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {:?}", e));
```

## Help

Ensure that CMake is installed on the system

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
