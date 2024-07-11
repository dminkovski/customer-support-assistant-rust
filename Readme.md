# Multi-Agent Customer Support System

This project demonstrates a multi-agent architecture using Azure OpenAI and Rust to enhance customer support operations. The system automates responses to common queries, escalates complex issues to humans, and analyzes customer sentiment to navigate.

## Features

1. **Automated Query Handling**: AI agent handles common customer queries automatically.
2. **Issue Escalation**: Detects complex or sensitive issues and escalates them by deciding for action items or upper management escalation.
3. **Sentiment Analysis**: Analyzes customer sentiment to prioritize support tickets.

## Components
- **Customer Query Agent**: Handles incoming customer queries and provides immediate responses using Azure OpenAI.
- **Escalation Agent**: Monitors interactions and escalates complex issues to humans or proposes action items.
- **Sentiment Analysis Agent**: Analyzes the sentiment of customer messages to identify unhappy customers.

## Prerequisites

- Rust (latest version)
- An Azure OpenAI API key

## Installation

1. **Clone the repository**
    ```bash
    git clone https://github.com/dminkovski/customer-support-assistant-rust.git
    cd customer-support-assistant-rust
    ```

2. **Set up environment variables**
    Create an `.env` file in the root directory and add your Azure OpenAI API key or copy the `.env.sample`.
    ```env
    AZURE_OPEN_AI_ENDPOINT=https://XXXXXXXXX.openai.azure.com/
    AZURE_OPEN_AI_KEY=XXXXXXXXXXXXX
    AZURE_OPEN_AI_MODEL_DEPLOYMENT_NAME=gpt-4o
    AZURE_OPEN_AI_API_VERSION=2024-02-15-preview
    ```

3. **Build the project**
    ```bash
    cargo build
    ```

## Usage

1. **Run the project**
    ```bash
    cargo run
    ```

## SupportCase Struct

The `SupportCase` struct is used to log information and maintain a complete history of the interaction.

```rust
struct SupportCase {
    pub case_id: Uuid,
    pub support_context: String,
    pub customer_query: String,
    pub support_response: Option<String>,
    pub sentiment: Option<String>,
    pub should_escalate: bool,
    pub escalated: bool,
    pub needs_upper_management_attention: bool,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub trace: Vec<Message>
}
```