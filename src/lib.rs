/*!
A rich and hybrid [mystb.in](https://mystb.in) API wrapper for Rust 🦀

## Introduction

myust is a rich and hybrid Rust wrapper for the mystb.in API that aims for user-flexibility.

myust supports hybrid clients:

- [`Client`] and [`AuthClient`] for asynchronous.
- [`SyncClient`] and [`SyncAuthClient`] for synchronous.

## Which one do I use?

It depends on your usecase,
if you're not doing anything with anything users-related
endpoints, use [`Client`]. It only wraps
non-users endpoints.

Otherwise, use [`AuthClient`]. It wraps both users
and non-users endpoints.
And the benefit of using [`AuthClient`] is mystb.in
gives you more ratelimit credits for authorized requests.

To use [`AuthClient`], you must have a mystb.in API
token to authenticate you to the API. Log into
[mystb.in](https://mystb.in) to get your own
API token.

## Installation

Add `myust = "0.1"` to your `Cargo.toml` file.

```toml
[dependencies]
myust = "0.1"
tokio = "1.27.0"
```

## Usage Examples

Asynchronously creating a paste with tomorrow expiration date, with error handling:
```rust
use chrono::{Days, Utc};
use myust::Client;

#[tokio::main]
async fn main() {
   let client = Client::new();
   let tomorrow = Utc::now().checked_add_days(Days::new(1)).unwrap().into();
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
           let url = format!("https://mystb.in/{}", paste.id);
           println!("Result: {}", url)
       },
       Err(_) => {
           println!("Error code: {}", result.unwrap_err().code)
       }
   }
}
```
Synchronously creating a multifile paste with a password:
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
   let client = AuthClient::new("YOUR_MYSTBIN_TOKEN").await;
   let result = client.delete_paste("EquipmentMovingExpensive").await; // The paste ID to delete
   match result {
       Ok(_) => println!("Successfully deleted the paste."),
       Err(_) => {
           println!("Error code: {}", result.unwrap_err().code)
       }
   }
}
```

You can check for another example snippets in [the test file](tests/test.rs).

## Help & Contributing

If you need any help regarding myust, feel free to open an issue about your problem, and feel free to make a pull request for bugfix, code improvements, etc.
*/

mod async_impl;
mod builders;
mod structs;
mod sync_impl;
mod traits;
mod utils;
pub use async_impl::{AuthClient, Client};
pub use structs::*;
pub use sync_impl::{SyncAuthClient, SyncClient};
