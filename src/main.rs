use helpers::command_line::CLIPrint;
use models::agents_coordinator::agent_coordinator::CoordinatorAgent;

mod api;
mod helpers;
mod models;

#[tokio::main]
async fn main() {
    CLIPrint::Default.out("System", "Starting Support Request");
    // Coordinator
    let context: &str = "This is a 5 star hotel.";
    let query: &str = "I am not happy with my room.";
    let mut coordinator_agent: CoordinatorAgent =
        CoordinatorAgent::new(context.to_string(), query.to_string());
    coordinator_agent.handle_support_request().await;
}
