use std::{
    io::{stdin, stdout},
    process::{Command, Stdio},
    thread::{self, sleep},
    time::{self, Duration},
};

use async_trait::async_trait;
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use reqwest::Client;

use crate::{helpers::{backend::{check_status_code, read_code_template_contents, read_executable_main_contents, save_api_endpoints, save_backend_code, WEB_SERVER_PROJECT_PATH}, command_line::{confirm_safe_code, CLIPrint}, request::{ai_request, prepare_message}}, models::{agents_common::{common_agent::{AgentState, CommonAgent}, common_traits::AgentFunctionTrait}, ai::chatgpt::Message, general::{route::Route, support_case::{self, SupportCase}}}};

const PROMPT_GENERATE_BACKEND:&str = r#"INPUT: Takes in a ACTIONS_DESCRIPTION and CODE_TEMPLATE for a website backend build
    IMPORTANT: The backend code is ONLY an example. If the Actions described requires it, make as many changes as you like.
    IMPORTANT: You do not need to follow the backend code exactly. Write functions that make sense for the users request if required.
    FUNCTION: Takes an existing set of code marked as CODE_TEMPLATE and updates or re-writes it to work for the purpose in the PROJECT_DESCRIPTION
    IMPORTANT: The following libraries are already installed
    reqwest, serde, serde_json, tokio, actix-web, async-trait, actix_cors
    No other external libraries should be used. Write functions that fit with the description from the ACTIONS_DESCRIPTION
    OUTPUT: Print ONLY the code, nothing else. This function ONLY prints code. Do not print '''rust at the beginning and ''' at the end of the code."#;

const PROMPT_FIX_CODE: &str = r#"INPUT: Takes in Rust BROKEN_CODE and the ERROR_BUGS found
    FUNCTION: Removes bugs from code
    IMPORTANT: Only prints out the new and improved code. No commentary or anything else"#;

const PROMPT_IMPROVE_CODE: &str = r#"INPUT: Takes in a ACTIONS_DESCRIPTION and CODE_TEMPLATE for a website backend build
    FUNCTION: Performs the following tasks:
    1. Removes any bugs in the code and adds minor additional functionality
    2. Makes sure everything requested in the spec from a backend standpoint was followed. If not, add the feature. No code should be implemented later. Everything should be written now.
    3. ONLY writes the code. No commentary.
    IMPORTANT: The following libraries are already installed. Does not use ANY libraries other than what was provided in the template
    reqwest, serde, serde_json, tokio, actix-web, async-trait"#;

const PROMPT_EXTRACT_API: &str = r#"INPUT: Takes in Rust webserver CODE_INPUT based on actix-web
FUNCTION: Prints out the JSON schema in JSON format only for url endpoints and their respective types
LOGIC: Script analyses all code and can categorize into the following object keys:
"route": This represents the url path of the endpoint
"is_route_dynamic": if a route has curly braces in it such as {symbol} or {id} as an example, then this will be set to true
"method": This represents the method being called
"request_body": This represents the body of a post method request
"response": This represents the output based upon the structs in the code and understanding the functions
SUPER IMPORTANT: Only prints out the JSON schema. No commentary or anything else.
 MUST READ: All keys are strings. Even bool should be wrapped in double quotes as "bool"
 IMPORTANT: Only prints in correct JSON. No spaces, tabs or newlines or similar additional formatting code. No '```json' prefix is allowed.
 EXAMPLE:
 INPUT_CODE:
 ...
 pub struct Item {
   pub id: u64,
   pub name: String,
   pub completed: bool,
 }
 pub struct User {
   pub id: u64,
   pub username: String,
   pub password: String,
 }
 ...
 HttpServer::new(move || {
   App::new()
       .app_data(data.clone())
       .route("/item", web::post().to(create_item))
       .route("/item/{id}", web::get().to(read_item))
       .route("/item/{id}", web::put().to(update_item))
       .route("/item/{id}", web::delete().to(delete_item))
       .route("/signup", web::post().to(signup))
       .route("/crypto", web::get().to(crypto))
 PRINTS JSON FORMATTED OUTPUT:
 [
   {
     "route": "/item/{id}",
     "is_route_dynamic": "true",
     "method": "get"
     "request_body": "None",
     "response": {
       "id": "number",
       "name": "string",
       "completed": "bool",
     }
   },
   {
     "route": "/item",
     "is_route_dynamic": "false",
     "method": "post",
     "request_body": {
       "id": "number",
       "name": "string",
       "completed": "bool",
     },
     "response": "None"
   },
   {
     "route": "/item/{id}",
     "is_route_dynamic": "true",
     "method": "delete",
     "request_body": "None",
     "response": "None"
   },
   {
     "route": "/crypto",
     "is_route_dynamic": "false",
     "method": "get",
     "request_body": "None",
     "response": "not_provided"
   },
   ... // etc
 ]"#;

#[derive(Debug)]
pub struct AgentBackendDeveloper {
    common: CommonAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let common = CommonAgent {
            objective: "Develops backend code for webserver and JSON database.".to_string(),
            role: "Backend Engineer".to_string(),
            state: AgentState::Waiting,
            memory: Vec::from([]),
        };
        Self {
            common,
            bug_errors: None,
            bug_count: 0,
        }
    }
    async fn call_initial_backend_code(&mut self, support_case: &mut SupportCase) {
        let code_template_str: String = read_code_template_contents();

        let msg_context: String = format!(
            "{} CODE_TEMPLATE: {} \n ACTIONS_DESCRIPTION: {} \n",
            PROMPT_GENERATE_BACKEND, code_template_str, support_case.supported_actions.join(",")
        );
        let msg: Message = prepare_message(&self.common.objective, &support_case.support_context, &msg_context);

        let ai_response = ai_request(
            msg,
        )
        .await.expect("should have returned ai response");
        save_backend_code(&ai_response);
    }

    async fn call_improved_backend_code(&mut self, support_case: &mut SupportCase) {
        let code_template_str: String = read_code_template_contents();

        let msg_context: String = format!(
            "{} CODE_TEMPLATE: {} \n ACTIONS_DESCRIPTION: {} \n",
            PROMPT_IMPROVE_CODE, code_template_str, support_case.supported_actions.join(",")
        );
        let msg: Message = prepare_message(&self.common.objective, &support_case.support_context, &msg_context);

        let ai_response = ai_request(
            msg,
        )
        .await.expect("should have returned ai response");
        save_backend_code(&ai_response);
    }

    async fn call_fix_code_bugs(&mut self, support_case: &mut SupportCase) {
        let backend_code: String = read_executable_main_contents();

        let msg_context: String = format!(
        "{} BACKEND_CODE: {:?} \n ERROR_BUGS: {:?} \n THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.", PROMPT_FIX_CODE, backend_code, self.bug_errors
      );
      let msg: Message = prepare_message(&self.common.objective, &support_case.support_context, &msg_context);

        let ai_response: String = ai_request(
            msg
        )
        .await.expect("should have returned ai response");
        save_backend_code(&ai_response);
    }

    async fn call_extract_rest_api_endpoints(&self, support_case: &mut SupportCase) -> String {
        let backend_code = read_executable_main_contents();

        let msg_context: String = format!(
            "{} CODE INPUT: {}",PROMPT_EXTRACT_API, backend_code
          );
          let msg: Message = prepare_message(&self.common.objective, &support_case.support_context, &msg_context);
    
            let ai_response: String = ai_request(
                msg
            )
            .await.expect("should have returned ai response");
        ai_response
    }
}

#[async_trait]
impl AgentFunctionTrait for AgentBackendDeveloper {
    fn get_common_from_agent(&self) -> &CommonAgent {
        &self.common
    }
    async fn execute(
        &mut self,
        support_case: &mut SupportCase,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.common.state != AgentState::Finished {
            match &self.common.state {
                AgentState::Waiting => {
                    CLIPrint::Info.out(
                        &self.common.role.as_str(),
                        "Starting to generate backend code...",
                    );
                    self.call_initial_backend_code(support_case).await;
                    self.common.state = AgentState::Working;
                    continue;
                }
                AgentState::Working => {
                    // TODO: Potentially loop a certain amount of times
                    if self.bug_count == 0 {
                        self.call_improved_backend_code(support_case).await;
                    } else {
                        self.call_fix_code_bugs(support_case).await;
                    }
                    self.common.state = AgentState::Testing;
                    continue;
                }
                AgentState::Testing => {
                    // Making sure there is a guard for safety
                    CLIPrint::Info.out(
                        &self.common.role.as_str(),
                        "Backend Code Unit Testing: Requesting user input.",
                    );

                    let is_safe_code: bool = confirm_safe_code();
                    if !is_safe_code {
                        panic!("Better go work on some AI alignment then.");
                    }

                    // Build and test code
                    CLIPrint::Info.out(
                        &self.common.role.as_str(),
                        "Backend Code Unit Testing: Building Project...",
                    );

                    // Cargo build
                    let build_backend_server: std::process::Output = Command::new("cargo")
                        .arg("build")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                        .expect("Failed to build backend application");

                    // Determine if build has errors
                    if build_backend_server.status.success() {
                        self.bug_errors = None;
                        self.bug_count = 0;
                        CLIPrint::Info.out(
                            &self.common.role.as_str(),
                            "Backend Code Unit Testing: Test server build successful.",
                        );
                    } else {
                        let error_array: Vec<u8> = build_backend_server.stderr;
                        let error_str: String = String::from_utf8(error_array).unwrap();
                        // Update error stats
                        self.bug_errors = Some(error_str);
                        self.bug_count += 1;

                        // Exits if too many bugs
                        if self.bug_count > 2 {
                            CLIPrint::Error.out(
                                &self.common.role.as_str(),
                                "Backend Code Unit Testing: Too many bugs found in code.",
                            );
                            panic!("Error: Too many bugs");
                        }

                        // Pass back for rework
                        self.common.state = AgentState::Working;
                        continue;
                    }

                    // Extract and Test API Endpoints
                    let api_endpoints_str: String = self.call_extract_rest_api_endpoints(support_case).await;
                    save_api_endpoints(&api_endpoints_str);
                    // Convert endpoints into values
                    let api_endpoints: Vec<Route> =
                        serde_json::from_str(api_endpoints_str.as_str())
                            .expect("Failed to decode API Endpoints.");

                    // Define endpoints to check - non dynamic (id etc.) | Just simple get requests
                    let check_endpoints: Vec<Route> = api_endpoints
                        .iter()
                        .filter(|&route| route.method == "get" && route.is_route_dynamic == "false")
                        .cloned()
                        .collect();

                    // Run backend application
                    CLIPrint::Info.out(
                        &self.common.role.as_str(),
                        "Backend Code Unit Testing: Starting Web Server...",
                    );

                    // Execute running server
                    let mut run_backend_server: std::process::Child = Command::new("cargo")
                        .arg("run")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("Failed to run backend application");

              
                    CLIPrint::Info.out(
                        &self.common.role.as_str(),
                        "Backend Code Unit Testing: Launching tests on server in couple seconds...",
                    );

                    let seconds_sleep: Duration = Duration::from_secs(5);
                    thread::sleep(seconds_sleep);

                    // Check status code
                    for endpoint in check_endpoints {
                        let testing_msg: String =
                            format!("Testing endpoint '{}'...", endpoint.route);
                        
                        CLIPrint::Info.out(
                            &self.common.role.as_str(),
                            &testing_msg,
                        );

                        // Create client with timout
                        let client = Client::builder()
                            .timeout(Duration::from_secs(5))
                            .build()
                            .unwrap();

                        let url: String = format!("http://localhost:8080{}", endpoint.route);
                        match check_status_code(&client, url.as_str()).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    let err_msg: String = format!(
                                        "WARNING: Failed to call endpoint: {}",
                                        endpoint.route
                                    );
                        
                                    CLIPrint::Warning.out(
                                        &self.common.role.as_str(),
                                        err_msg.as_str(),
                                    );
                                }
                            }
                            Err(e) => {
                                
                                CLIPrint::Error.out(
                                    &self.common.role.as_str(),
                                    format!("{:?}", e).as_str(),
                                );
                                run_backend_server
                                    .kill()
                                    .expect("Failed to kill backend server");
                            }
                        }
                    }
                    CLIPrint::Info.out(
                        &self.common.role.as_str(),
                        "Backend testing complete.",
                    );

                    run_backend_server.kill().expect("Failed to kill backend server after completion.");

                    self.common.state = AgentState::Finished;
                }
                _ => {
                    &AgentState::Finished;
                }
            }
        }
        Ok(())
    }
}
