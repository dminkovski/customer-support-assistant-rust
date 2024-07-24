use helpers::command_line::{get_user_response, CLIPrint};
use models::agents_coordinator::agent_coordinator::CoordinatorAgent;

mod api;
mod helpers;
mod models;

#[tokio::main]
async fn main() {
    let context: String = String::from("Customer Support");
    let query: String = get_user_response("What is your ask for the customer support?");
    CLIPrint::Default.out("System", "Thank you. \nStarting Support Request...");
    // Coordinator
    let mut coordinator_agent: CoordinatorAgent = CoordinatorAgent::new(context, query);
    coordinator_agent.handle_support_request().await;
}
