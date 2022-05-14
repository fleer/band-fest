#![warn(clippy::all, clippy::pedantic)]
use dotenv::dotenv;
use rspotify::{model::AlbumId, prelude::*, ClientCredsSpotify, Credentials};

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // Loads .env file
    dotenv().ok();

    // Read RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET in an .env file or
    let creds = Credentials::from_env().unwrap();
    println!("Credentials: {:#?}", creds);

    let mut spotify = ClientCredsSpotify::new(creds);

    // Obtaining the access token. Requires to be mutable because the internal
    // token will be modified. We don't need OAuth for this specific endpoint,
    // so `...` is used instead of `prompt_for_user_token`.
    spotify.request_token().await.unwrap();

    // Running the requests
    let birdy_uri = AlbumId::from_uri("spotify:album:0sNOF9WDwhWunNAHPD3Baj").unwrap();
    let albums = spotify.album(&birdy_uri).await;

    println!("Response: {:#?}", albums);
}
