use chrono::{Days, Utc};
use myust::AuthClient;

#[test]
#[cfg(feature = "sync")]
fn createsync() {
    let client = myust::SyncClient::new();
    let paste = client
        .create_paste(|p| p.filename("syncmyust.txt").content("lol"))
        .unwrap();
    println!("{paste:#?}")
}

#[test]
#[cfg(feature = "sync")]
fn getsync() {
    let client = myust::SyncClient::new();
    let paste = client
        .get_paste(|p| p.id("SpecificsBillionComponent").password("x"))
        .unwrap();
    println!("{paste:#?}");
}

#[tokio::test]
async fn readme() {
    let client = myust::Client::new();
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
        }
        Err(_) => {
            println!("Error code: {}", result.unwrap_err().code)
        }
    }
}

#[tokio::test]
async fn readme_del() {
    let client = AuthClient::new("eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJpZCI6OTQ5MjI3MTg3NTN9.pmoHaFT3BBPAWUgQ5OUpYfx8fD3BkiO0cHfFHrk3dbpjyuHhIqfpwA3Bh5PXXickNAjmrb_fsBxkltYuy_EuVg").await;
    let result = client.delete_paste("RotationSchedulesProperly").await; // The paste ID to delete
    match result {
        Ok(_) => println!("Successfully deleted the paste."),
        Err(_) => {
            println!("Error code: {}", result.unwrap_err().code)
        }
    }
}
#[tokio::test]
async fn create_paste() {
    let client = myust::AuthClient::new("eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJpZCI6OTQ5MjI3MTg3NTN9.pmoHaFT3BBPAWUgQ5OUpYfx8fD3BkiO0cHfFHrk3dbpjyuHhIqfpwA3Bh5PXXickNAjmrb_fsBxkltYuy_EuVg").await;
    let paste = client
        .create_paste(|p| {
            p.filename("myust.txt")
                .content("hi from myust")
                .expires(Utc::now().checked_add_days(Days::new(1)).unwrap().into())
        })
        .await
        .unwrap();
    println!("{paste:#?}")
}

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

#[tokio::test]
async fn get_paste() {
    let client = myust::Client::new();
    let paste = client
        .get_paste(|p| p.id("CalendarNightsCovering"))
        .await
        .unwrap();
    println!("{paste:#?}")
}

#[tokio::test]
async fn user_paste() {
    let client = myust::AuthClient::new("eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJpZCI6OTQ5MjI3MTg3NTN9.pmoHaFT3BBPAWUgQ5OUpYfx8fD3BkiO0cHfFHrk3dbpjyuHhIqfpwA3Bh5PXXickNAjmrb_fsBxkltYuy_EuVg").await;
    let pastes = client.get_user_pastes(|p| p).await.unwrap();
    println!("{pastes:#?}")
}

#[tokio::test]
async fn user_bm() {
    let client =  myust::AuthClient::new("eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJpZCI6OTQ5MjI3MTg3NTN9.pmoHaFT3BBPAWUgQ5OUpYfx8fD3BkiO0cHfFHrk3dbpjyuHhIqfpwA3Bh5PXXickNAjmrb_fsBxkltYuy_EuVg").await;
    let bm = client.get_user_bookmarks().await.unwrap();
    println!("{bm:#?}")
}

#[tokio::test]
async fn add_bm() {
    let client =  myust::AuthClient::new("eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJpZCI6OTQ5MjI3MTg3NTN9.pmoHaFT3BBPAWUgQ5OUpYfx8fD3BkiO0cHfFHrk3dbpjyuHhIqfpwA3Bh5PXXickNAjmrb_fsBxkltYuy_EuVg").await;
    client.create_bookmark("InfraredYukonEmpty").await.unwrap();
    let bm = client.get_user_bookmarks().await.unwrap();
    println!("{bm:#?}")
}

#[tokio::test]
#[ignore]
async fn del_bm() {
    let client =  myust::AuthClient::new("eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJpZCI6OTQ5MjI3MTg3NTN9.pmoHaFT3BBPAWUgQ5OUpYfx8fD3BkiO0cHfFHrk3dbpjyuHhIqfpwA3Bh5PXXickNAjmrb_fsBxkltYuy_EuVg").await;
    client.delete_bookmark("InfraredYukonEmpty").await.unwrap();
    let bm = client.get_user_bookmarks().await.unwrap();
    println!("{bm:#?}")
}
