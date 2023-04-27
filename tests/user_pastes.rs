#[tokio::test]
async fn user_pastes() {
    let client = myust::Client::new()
        .auth(std::env::var("MYSTBIN_TOKEN").unwrap())
        .await;
    let pastes = client.get_user_pastes(|p| p).await.unwrap();
    println!("{pastes:#?}")
}
