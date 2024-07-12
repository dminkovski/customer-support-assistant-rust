use std::fs;

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::models::ai::chatgpt::Message;

pub const CODE_TEMPLATE_PATH: &str = "template/code_template.rs";
pub const EXEC_MAIN_PATH: &str = "template/main.rs";
pub const API_SCHEMA_PATH: &str = "template/api_schema.json";
pub const WEB_SERVER_PROJECT_PATH: &str = "template";

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    // Extend the string to encourage only printing the output
    let msg: String = format!(
        "FUNCTION: {}
    INSTRUCTION: You are a function printer. You ONLY print the results of functions.
    Nothing else. No commentary. Here is the input to the function: {}.
    Print out what the function will return",
        ai_function_str, func_input
    );

    Message {
        role: "system".to_string(),
        content: msg,
    }
}



// Check whether request URL is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// Get Code Template
pub fn read_code_template_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path.clone())
        .expect(format!("Failed to read code template from path: {}", path).as_str())
}

// Read Executable Main
pub fn read_executable_main_contents() -> String {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::read_to_string(path.clone())
        .expect(format!("Failed to read code template from path: {}", path).as_str())
}

// Save new Backend Code
pub fn save_backend_code(contents: &String) {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::write(path.clone(), contents).expect(format!("Failed to write to path: {}", path).as_str());
}

// Save JSON API Endpoint Schema
pub fn save_api_endpoints(api_endpoints: &String) {
    let path: String = String::from(API_SCHEMA_PATH);
    fs::write(path.clone(), api_endpoints)
        .expect(format!("Failed to write to path: {}", path).as_str());
}