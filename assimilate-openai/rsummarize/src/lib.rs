/*
Summarize content by using openAI GPT-3 API
*/

/*accepts the prompt and returns the completion
pub async fn complete_prompt(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_token = std::env::var("OPENAI_API_KEY")?;
    let client = openai_api::Client::new(&api_token);
    let prompt = String::from(prompt);
    let result = client.complete_prompt(prompt.as_str()).await?;
    Ok(result.choices[0].text.clone())
}
*/
pub async fn summarize(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_token = std::env::var("OPENAI_API_KEY")?;
    let client = openai_api::Client::new(&api_token);
    let prompt = String::from(prompt);
    let result = client.summarize(prompt.as_str()).await?;
    println!("{:?}", result);
}
