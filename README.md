 # **myust**: 📋=>🌐=>🦀
 
[![crates.io][crates-io-badge]][crates-io]
[![Documentation][docs-badge]][docs]
[![MIT][license-badge]](./LICENSE)

 ## Introduction

 myust is a rich and hybrid Rust wrapper for the mystb.in API that aims for user-flexibility.
 
 myust supports hybrid clients:

- [Client] for asynchronous, and [SyncClient] for synchronous.

**⚠️ Synchronous clients are only available on the [sync] feature.**

## Authentication

You can authenticate with the API using the `auth` method with your
[mystb.in], example:

```rust
use myust::{Client, SyncClient};

let client = Client::new().auth("YOUR_MYSTBIN_TOKEN").await;
// or using synchronous client,
let client = SyncClient::new().auth("YOUR_MYSTBIN_TOKEN");
```

It will panic if the provided token is invalid.

 ## Installation

 Add `myust = "1.0"` to your `Cargo.toml` file.

 ```toml
 [dependencies]
 myust = "1.0"
 tokio = "1.0"
 ```

 If you want to use synchronous clients, add the [`sync`][sync] feature.

 ```toml
 [dependencies]
 myust = { version = "1.0", features = ["sync"] }
 ```

 ## Usage Examples

 Asynchronously creating a paste with tomorrow expiration date, with error handling:
 ```rust
 use myust::{Client, Expiry};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let tomorrow = Expiry {
        days: 1,
        ..Default::default()
    }; // other fields default to 0
    let result = client
        .create_paste(|p| {
            p.filename("myust.txt")
                .content("Hello from myust!")
                .expires(tomorrow)
        })
        .await;
    match result {
        Ok(_) => {
            let paste = result.unwrap();
            println!("{paste:#?}");
            let url = format!("https://mystb.in/{}", paste.id);
            println!("Result: {}", url)
        }
        Err(_) => {
            println!("Error code: {}", result.unwrap_err().code)
        }
    }
}
```
Synchronously creating a multifile paste with a password (you must have the [`sync`][sync] feature enabled):
```rust
use myust::SyncClient;

fn main() {
    let client = SyncClient::new();
    let paste = client
        .create_multifile_paste(|p| {
            p.file(|f| {
                f.filename("myust1.txt")
                    .content("first file")
                    .password("myust")
            }); // set the password on the first file only, same for expiration date
            p.file(|f| f.filename("myust2.txt").content("second file"))
        })
        .unwrap();
    let url = format!("https://mystb.in/{}", paste.id);
    println!("Result: {}", url)
}
```

Asynchronously deleting a paste (you must own the paste):
```rust
use myust::AuthClient;

#[tokio::main]
async fn main() {
    let client = AuthClient::new()
        .auth(std::env::var("MYSTBIN_TOKEN").unwrap())
        .await;
    let result = client.delete_paste("EquipmentMovingExpensive").await; // The paste ID to delete
    match result {
        Ok(_) => println!("Successfully deleted the paste."),
        Err(_) => {
            println!("Error code: {}", result.unwrap_err().code)
        }
    }
}
```

You can check for another example snippets in [the test folder](tests/).

## Help & Contributing

If you need any help regarding myust, feel free to open an issue about your problem, and feel free to make a pull request for code improvements, bugfixing, etc.

[Client]: https://docs.rs/myust/latest/myust/struct.Client.html
[SyncClient]: https://docs.rs/myust/latest/myust/sync/struct.SyncClient.html
[sync]: https://docs.rs/myust/latest/myust/sync/index.html
[crates-io-badge]: https://img.shields.io/crates/v/myust.svg
[crates-io]: https://crates.io/crates/myust
[docs-badge]: https://docs.rs/myust/badge.svg
[docs]: https://docs.rs/myust
[license-badge]: https://img.shields.io/crates/l/myust.svg
[mystb.in]: https://mystb.in