#[tokio::test]
async fn delete_bookmark() {
    let client = myust::Client::new()
        .auth(std::env::var("MYSTBIN_TOKEN").unwrap())
        .await;
    client.delete_bookmark("InfraredYukonEmpty").await.unwrap();
    let bm = client.get_user_bookmarks().await.unwrap();
    println!("{bm:#?}")
}
