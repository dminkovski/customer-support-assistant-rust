use crate::models::ai::{
    assistant::{AssistantRequest, AssistantResponse, Thread, ThreadMessageResponse, ThreadRunRequest, ThreadRunResponse},
    chatgpt::Message,
};
use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::{error::Error, future::IntoFuture};

pub fn create_client() -> Client {
    dotenv().ok();

    let key: String =
        dotenv::var("AZURE_OPEN_AI_KEY").expect("AZURE_OPEN_AI_KEY missing from .env");

    let mut headers: HeaderMap = HeaderMap::new();
    headers.append(
        "Content-Type",
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers.append("api-key", HeaderValue::from_str(&key).unwrap());

    Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn Error + Send> { Box::new(e) })
        .expect("should have created client")
}

pub async fn create_assistant(
    assistant: AssistantRequest,
) -> Result<AssistantResponse, Box<dyn Error + Send>> {
    let client: Client = create_client();
    let endpoint: String =
        dotenv::var("AZURE_OPEN_AI_ENDPOINT").expect("AZURE_OPEN_AI_ENDPOINT missing from .env");
    let api_version: String = dotenv::var("AZURE_OPEN_AI_API_VERSION")
        .expect("AZURE_OPEN_AI_API_VERSION missing from .env");

    let url = &format!("{}/openai/assistants?api-version={}", endpoint, api_version);

    let api_response: AssistantResponse = client
        .post(url)
        .json(&assistant)
        .send()
        .await
        .map_err(|e| -> Box<dyn Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn Error + Send> { Box::new(e) })?;

    Ok(api_response.clone())
}

pub async fn create_and_run_thread(message: Message, assistant_id: &str) -> Result<ThreadRunResponse, Box<dyn Error + Send>> {
    let client: Client = create_client();
    let endpoint: String =
        dotenv::var("AZURE_OPEN_AI_ENDPOINT").expect("AZURE_OPEN_AI_ENDPOINT missing from .env");
    let api_version: String = dotenv::var("AZURE_OPEN_AI_API_VERSION")
        .expect("AZURE_OPEN_AI_API_VERSION missing from .env");

    let url = &format!("{}/openai/threads/runs?api-version={}", endpoint, api_version);

    let thread_request: ThreadRunRequest = ThreadRunRequest {
        thread: Thread {
            messages: vec![message],
        },
        assistant_id:String::from(assistant_id)
    };

    let api_response: ThreadRunResponse = client
        .post(url)
        .json(&thread_request)
        .send()
        .await
        .map_err(|e| -> Box<dyn Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn Error + Send> { Box::new(e) })?;

    Ok(api_response.clone())
}

pub async fn create_thread_message(message: Message, thread_id: &str) -> Result<ThreadMessageResponse, Box<dyn Error + Send>> {
    let client: Client = create_client();
    let endpoint: String =
        dotenv::var("AZURE_OPEN_AI_ENDPOINT").expect("AZURE_OPEN_AI_ENDPOINT missing from .env");
    let api_version: String = dotenv::var("AZURE_OPEN_AI_API_VERSION")
        .expect("AZURE_OPEN_AI_API_VERSION missing from .env");

    let url = &format!("{}/openai/threads/{}/messages?api-version={}", endpoint, thread_id, api_version);

    let api_response: ThreadMessageResponse = client
        .post(url)
        .json(&message)
        .send()
        .await
        .map_err(|e| -> Box<dyn Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn Error + Send> { Box::new(e) })?;

    Ok(api_response.clone())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_assistant() {
        let assistant: AssistantRequest = AssistantRequest {
          instructions: String::from("You are a personal concierge inside a hotel. You want the customer to feel heard and understood."),
          name: String::from("Concierge"),
          model: String::from("gpt-4o"),
          description: String::from(""),
          temperature: 1.0,
          top_p: 1.0,
          response_format:String::from("auto"),
        };

        let assistant_result = create_assistant(assistant).await;
        let thread_result = create_and_run_thread(Message {
            role: String::from("user"),
            content: String::from("I am not happy with my room."),
        }, &assistant_result.as_ref().unwrap().id)
        .await;
        if let Ok(assistant_response) = assistant_result {
            println!("{:?}", assistant_response);
            assert!(true)
        } else {
            assert!(false)
        }

        if let Ok(thread_response) = thread_result {
            println!("{:?}", thread_response);
            assert!(true)
        } else {
            assert!(false)
        }
    }
}
