use std::sync::Arc;
use async_trait::async_trait;
use serde_json::{json, Value};
use crate::AgentTool;

#[derive(Clone, Debug)]
struct ClosureTool<T>
where
    T: (AsyncFn(Value) -> anyhow::Result<String>) + Sync + Send,
{
    c_name: String,
    c_description: String,

    c_tool: Arc<T>,
}

#[async_trait]
impl<CTX, T> AgentTool<CTX> for ClosureTool<T>
where
    T: (AsyncFn(Value) -> anyhow::Result<String>) + Sync + Send,
{
    fn name(&self) -> String {
        self.c_name.clone()
    }

    fn description(&self) -> String {
        self.c_description.clone()
    }

    fn schema(&self) -> Value {
        json!({})
    }

    async fn call(&self, _ctx: &CTX, params: Value) -> anyhow::Result<String> {
        let closure = self.c_tool.clone();
        let result = (closure)(params).await?;
        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_closure() {
        let c_tool = async |_params: Value| -> anyhow::Result<String> { Ok("result".to_string()) };
        let c = ClosureTool {
            c_name: "Example".to_string(),
            c_description: "Example description".to_string(),
            c_tool: Arc::new(c_tool),
        };

        let result = (c.c_tool)(json!({})).await;
        assert_eq!(result.unwrap(), "result");

        // let tool: Arc<dyn AgentTool<()>> = Arc::new(c);
        // assert_eq!(tool.name(), "Example".to_string());

        // let result = c.call(&(), json!({})).await;
        //
        // assert_eq!(c.name(), "Example".to_string());
        // assert_eq!(c.description(), "Example description".to_string());
        //
        // assert_eq!(result.unwrap(), "result".to_string());
    }
}
