#[cfg(feature = "sync")]
#[test]
fn get_paste_sync() {
    let client = myust::SyncClient::new();
    let paste = client
        .get_paste(|p| p.id("GarminDosageExists").password("myust"))
        .unwrap();
    println!("{paste:#?}");
}
