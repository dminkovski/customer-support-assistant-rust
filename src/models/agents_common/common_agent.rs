use crate::models::ai::chatgpt::Message;

use super::common_traits::CommonTrait;

#[derive(Debug, PartialEq)]
pub enum AgentState {
    Waiting,
    Working,
    Error,
    Finished,
}

#[derive(Debug)]
pub struct CommonAgent {
    pub role: String,
    pub objective: String,
    pub state: AgentState,
    pub memory: Vec<Message>,
}

impl CommonTrait for CommonAgent {
    fn new(role: String, objective: String) -> Self {
        Self {
            role,
            objective,
            state: AgentState::Waiting,
            memory: Vec::from([]),
        }
    }
    fn get_memory(&self) -> &Vec<Message> {
        &self.memory
    }
    fn get_objective(&self) -> &String {
        &self.objective
    }
    fn get_role(&self) -> &String {
        &self.role
    }
    fn get_state(&self) -> &AgentState {
        &self.state
    }
    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }
}
