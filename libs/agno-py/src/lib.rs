use pyo3::prelude::*;
use std::sync::Arc;
use agno_rust::{Agent as RustAgent, OpenAIClient, LanguageModel, ToolRegistry};

// ─────────────────────────────────────────────────────────────────────────────
// Models
// ─────────────────────────────────────────────────────────────────────────────

/// Wrapper for OpenAI model configuration
#[pyclass]
#[derive(Clone)]
struct OpenAIChat {
    model_id: String,
    api_key: Option<String>,
}

#[pymethods]
impl OpenAIChat {
    #[new]
    #[pyo3(signature = (id="gpt-4o", api_key=None))]
    fn new(id: String, api_key: Option<String>) -> Self {
        Self { model_id: id, api_key }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Agent
// ─────────────────────────────────────────────────────────────────────────────

/// The main Agent class 
#[pyclass]
struct Agent {
    // We store the Rust agent wrapped in a runtime-agnostic container or using generic erasure.
    // Ideally we'd store RustAgent<dyn LanguageModel>, but Rust generics + PyO3 is tricky.
    // For this MVP, we'll store specific agent types or an enum, or use dynamic dispatch if possible.
    // Agno-rust uses generics: Agent<M: LanguageModel>.
    // To expose this to Python, we likely need a trait object or an enum wrapper.
    // Let's use dynamic dispatch via Arc<dyn LanguageModel> if we can refactor agno_rust::Agent
    // OR create an enum of supported agents.
    
    // For MVP, we'll implement it wrapping a specific instance (OpenAI) and expand later.
    inner: Arc<tokio::sync::Mutex<RustAgent<OpenAIClient>>>,
    rt: tokio::runtime::Runtime,
}

#[pymethods]
impl Agent {
    #[new]
    #[pyo3(signature = (model=None, description=None, markdown=true))]
    fn new(model: Option<OpenAIChat>, description: Option<String>, markdown: bool) -> PyResult<Self> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
            
        let model_config = model.unwrap_or(OpenAIChat { 
            model_id: "gpt-4".to_string(), 
            api_key: None 
        });

        let client = if let Some(key) = model_config.api_key {
            OpenAIClient::new(key)
        } else {
            OpenAIClient::from_env().map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
        };
        
        let client = client.with_model(model_config.model_id);
        
        let mut agent = RustAgent::new(std::sync::Arc::new(client));
        if let Some(desc) = description {
            agent = agent.with_system_prompt(desc);
        }

        Ok(Agent {
            inner: Arc::new(tokio::sync::Mutex::new(agent)),
            rt,
        })
    }

    /// Get the response from the model and print it
    fn print_response(&self, message: String) -> PyResult<()> {
        let agent = self.inner.clone();
        
        // Block on async execution
        self.rt.block_on(async move {
            let mut agent_lock = agent.lock().await;
            match agent_lock.respond(&message).await {
                Ok(response) => {
                    println!("{}", response);
                    Ok(())
                },
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
            }
        })
    }
    
    /// Get the response as a string
    fn run(&self, message: String) -> PyResult<String> {
        let agent = self.inner.clone();
        self.rt.block_on(async move {
            let mut agent_lock = agent.lock().await;
            match agent_lock.respond(&message).await {
                Ok(response) => Ok(response),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
            }
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Module Definition
// ─────────────────────────────────────────────────────────────────────────────

#[pymodule]
fn agno(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Agent>()?;
    m.add_class::<OpenAIChat>()?;
    Ok(())
}
