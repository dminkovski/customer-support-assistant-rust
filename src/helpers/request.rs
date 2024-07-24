use std::error::Error;

use crate::{
    api::gpt_request::call_gpt,
    models::{ai::chatgpt::Message, general::support_case::SupportCase},
};

pub fn prepare_message(objective: &str, context: &str, query: &str) -> Message {
    let msg_str: String = format!(
        "
  YOUR INSTRUCTIONS: {:?}
  CONTEXT: {:?}
  QUERY: {:?}
  ",
        objective, context, query
    );

    Message {
        role: "system".to_string(),
        content: msg_str,
    }
}

// Performs call to ChatGPT
pub async fn ai_request(msg: Message) -> Result<String, Box<dyn Error + Send>> {
    // Get LLM response
    let chatgpt_response: Result<String, Box<dyn Error + Send>> = call_gpt(vec![msg.clone()]).await;

    // Return successful response or try again
    match chatgpt_response {
        Ok(response) => Ok(response),
        Err(_) => call_gpt(vec![msg.clone()]).await,
    }
}
