# csgo-gsi

[![builds.sr.ht status](https://builds.sr.ht/~boringcactus/csgo-gsi.svg)](https://builds.sr.ht/~boringcactus/csgo-gsi?)
[![Crates.io version](https://img.shields.io/crates/v/csgo-gsi)](https://crates.io/crates/csgo-gsi)
[![Crates.io downloads](https://img.shields.io/crates/d/csgo-gsi)](https://crates.io/crates/csgo-gsi)
![Crates.io license](https://img.shields.io/crates/l/csgo-gsi)

Helper library for the [CS:GO Game State Integration (GSI) API][gsi].

Best used with the [tokio](https://tokio.rs/) async ecosystem.

[gsi]: https://developer.valvesoftware.com/wiki/Counter-Strike:_Global_Offensive_Game_State_Integration

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
csgo-gsi = "0.3.0"
```

# Simple Example

```rust
use csgo_gsi::{GSIConfigBuilder, GSIServer, Subscription};

#[tokio::main]
async fn main() {
    let config = GSIConfigBuilder::new("csgo-gsi Example")
        .subscribe_multiple(Subscription::UNRESTRICTED)
        .build();

    let mut server = GSIServer::new(config, 31337);
    server.add_listener(|update| println!("Got an update {:#?}", update));

    server
        .run()
        .await
        .expect("server didn't start");
}
```

## License

Licensed under the [Anti-Capitalist Software License](https://anticapitalist.software/) version 1.4.

## Contribution

Unless you explicitly state otherwise, any contribution you submit shall be
also under the Anti-Capitalist Software License version 1.4, without any additional terms or conditions.

## History

v0.3.0 - 2020-09-20
- add optional export into the Rhai scripting language, with the `rhai` feature

v0.2.0 - 2020-09-20
- make `Update` be `Clone`

v0.1.0 - 2020-09-15
- Initial release
