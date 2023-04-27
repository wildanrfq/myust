#[tokio::test]
async fn create_pastes() {
    let client = myust::Client::new();
    let paste = client
        .create_multifile_paste(|p| {
            p.file(|f| {
                f.filename("myust1.txt")
                    .content("first file")
                    .password("myust")
            }); // set the password on the first file only, same for expiration date
            p.file(|f| f.filename("myust2.txt").content("second file"))
        })
        .await
        .unwrap();
    let url = format!("https://mystb.in/{}", paste.id);
    println!("Result: {}", url)
}
