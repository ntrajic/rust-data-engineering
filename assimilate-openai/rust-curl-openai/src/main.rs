/*
 */

use reqwest::{header, Client};
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

    //use env variable OPENAI_API_KEY
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let url = "https://api.openai.com/v1/completions";

    let json = json!({
        "model": "text-davinci-003",
        "prompt": "Summarize this for a second-grade student:\n\nJupiter is the fifth planet from the Sun and the largest in the Solar System. It is a gas giant with a mass one-thousandth that of the Sun, but two-and-a-half times that of all the other planets in the Solar System combined. Jupiter is one of the brightest objects visible to the naked eye in the night sky, and has been known to ancient civilizations since before recorded history. It is named after the Roman god Jupiter.[19] When viewed from Earth, Jupiter can be bright enough for its reflected light to cast visible shadows,[20] and is on average the third-brightest natural object in the night sky after the Moon and Venus.",
        "temperature": 0.7,
        "max_tokens": 256,
        "top_p": 1,
        "frequency_penalty": 0,
        "presence_penalty": 0
    });

    let response = client
        .post(url)
        .header(header::AUTHORIZATION, format!("Bearer {api_key}"))
        .header(header::CONTENT_TYPE, "application/json")
        .json(&json)
        .send()
        .await?;

    let body = response.text().await?;
    println!("{body}");

    Ok(())
}
