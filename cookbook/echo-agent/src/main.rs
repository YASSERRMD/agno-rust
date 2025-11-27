use agno_core::{Agent, StubModel, Tool, ToolRegistry};
use async_trait::async_trait;
use serde_json::Value;

#[derive(Debug)]
struct Echo;

#[async_trait]
impl Tool for Echo {
    fn name(&self) -> &str {
        "echo"
    }

    fn description(&self) -> &str {
        "Echoes the inbound JSON back to the caller"
    }

    async fn call(&self, input: Value) -> agno_core::Result<Value> {
        Ok(input)
    }
}

#[tokio::main]
async fn main() -> agno_core::Result<()> {
    // The stub model produces two directives:
    // 1. Ask to call the `echo` tool with a JSON payload.
    // 2. Respond with the final assistant message.
    let scripted = vec![
        r#"{"action":"call_tool","name":"echo","arguments":{"text":"ping"}}"#.into(),
        r#"{"action":"respond","content":"Echo complete."}"#.into(),
    ];
    let model = StubModel::new(scripted);

    let mut tools = ToolRegistry::new();
    tools.register(Echo);

    let mut agent = Agent::new(model).with_tools(tools);
    let reply = agent.respond("say ping").await?;

    println!("Agent reply: {reply}");
    Ok(())
}
