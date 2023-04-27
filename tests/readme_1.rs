use myust::{Client, Expiry};

#[tokio::test]
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
