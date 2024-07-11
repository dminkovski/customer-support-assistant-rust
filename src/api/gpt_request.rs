use crate::models::ai::chatgpt::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::error::Error;

pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn Error + Send>> {
    dotenv().ok();

    let endpoint: String =
        dotenv::var("AZURE_OPEN_AI_ENDPOINT").expect("AZURE_OPEN_AI_ENDPOINT missing from .env");
    let key: String =
        dotenv::var("AZURE_OPEN_AI_KEY").expect("AZURE_OPEN_AI_KEY missing from .env");
    let model: String = dotenv::var("AZURE_OPEN_AI_MODEL_DEPLOYMENT_NAME")
        .expect("AZURE_OPEN_AI_MODEL_DEPLOYMENT_NAME missing from .env");
    let api_version: String = dotenv::var("AZURE_OPEN_AI_API_VERSION")
        .expect("AZURE_OPEN_AI_API_VERSION missing from .env");

    let url = &format!(
        "{}/openai/deployments/{}/chat/completions?api-version={}",
        endpoint, model, api_version
    );

    let mut headers: HeaderMap = HeaderMap::new();
    headers.append(
        "Content-Type",
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers.append("api-key", HeaderValue::from_str(&key).unwrap());

    let client: Client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn Error + Send> { Box::new(e) })?;

    let chat_completion = ChatCompletion {
        model,
        messages,
        temperature: 0.1,
    };

    let api_response: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn Error + Send> { Box::new(e) })?;

    Ok(api_response.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_gpt() {
        let msg = Message {
            content: "Hi there".to_string(),
            role: "user".to_string(),
        };
        let messages: Vec<Message> = Vec::from([msg]);
        let result = call_gpt(messages).await;
        if let Ok(response) = result {
            println!("{}", response);
            assert!(true)
        } else {
            assert!(false)
        }
    }
}
