use myust::Client;

#[tokio::test]
async fn main() {
    let client = Client::new()
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
