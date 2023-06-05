use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::get;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    match get("http://httpbin.org/get").await {
        Ok(response) => {
            let result = response.json::<GetBody>().await;
            match result {
                Ok(json) => {
                    println!("{:#?}", json);
                }
                Err(err) => {
                    println!("{:#?}", err)
                }
            }
        }
        Err (err) => {
            println!("{:#?}", err)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GetBody {
    args: HashMap<String, String>,
    headers: HashMap<String, String>,
    origin: String,
    url: String,
}
