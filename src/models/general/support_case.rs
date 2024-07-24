use chrono::{DateTime, Local};
use uuid::Uuid;

use crate::models::ai::chatgpt::Message;

#[derive(Debug, Clone)]
pub struct SupportCase {
    pub case_id: Uuid,
    pub support_context: String,
    pub customer_query: String,
    pub intent_category: Option<String>,
    pub support_response: Option<String>,
    pub sentiment: Option<String>,
    pub should_escalate: bool,
    pub escalated: bool,
    pub needs_upper_management_attention: bool,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub trace: Vec<Message>,
    pub supported_actions: Vec<String>,
}

impl SupportCase {
    pub fn new(context: String, query: String) -> Self {
        Self {
            case_id: Uuid::new_v4(),
            support_context: context,
            customer_query: query,
            intent_category: None,
            support_response: None,
            sentiment: None,
            should_escalate: false,
            escalated: false,
            needs_upper_management_attention: false,
            created_at: Local::now(),
            updated_at: Local::now(),
            trace: Vec::from([]),
            supported_actions: Vec::from([]),
        }
    }
    pub fn get_history(&self) -> String {
        let mut history: String = String::new();
        for message in self.trace.iter() {
            if message.role != "system" {
                history.push_str(format!("{}\n", message.content.as_str()).as_str());
            }
        }
        history
    }
    pub fn updated(&mut self) {
        self.updated_at = Local::now()
    }
}
