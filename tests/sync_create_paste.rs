#[cfg(feature = "sync")]
#[test]
fn create_paste_sync() {
    let client = myust::SyncClient::new().auth(std::env::var("MYSTBIN_TOKEN").unwrap());
    let paste = client
        .create_paste(|p| p.filename("myust.txt").content("hi from myust"))
        .unwrap();
    println!("{paste:#?}")
}
