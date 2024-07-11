use helpers::command_line::{get_user_response, CLIPrint};
use models::agents_coordinator::agent_coordinator::CoordinatorAgent;

mod api;
mod helpers;
mod models;

#[tokio::main]
async fn main() {
    CLIPrint::Default.out("System", "Let's set the scene (f.e '5 Star Hotel').");
    let context: String = get_user_response("Where are we?");
    let query: String = get_user_response("What is your ask for the customer support?");
    CLIPrint::Default.out("System", "Thank you. \nStarting Support Request...");
    // Coordinator
    let mut coordinator_agent: CoordinatorAgent =
        CoordinatorAgent::new(context, query);
    coordinator_agent.handle_support_request().await;
}
