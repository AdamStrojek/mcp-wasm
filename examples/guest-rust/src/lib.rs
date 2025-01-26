use wit_bindgen::generate;

generate!("server" in "../../wit");

struct MyServer;

impl Guest for MyServer {
    fn initialize(client_info: VersionInfo, capabilities: Vec<Capability>) -> (VersionInfo, Vec<(String, bool)>) {
        eprintln!("WASM: Server initialized by: {:?} with capabilities: {:?}", client_info, capabilities);
        (VersionInfo {
            name: "My Tool".to_string(),
            version: "0.1.0".to_string(),
        }, vec![])
    }

    fn notify(name: String, _params: Option<String>) {
        eprintln!("WASM: Received notification: {name}");
    }
}

use mcp_wasm::server::errors;

// region Resources
use exports::mcp_wasm::server::resources;

impl resources::Guest for MyServer {
    fn list() -> Result<Vec<resources::ResourceInfo>, errors::Error> {
        eprintln!("WASM: List all resources");
        Ok(vec![
            resources::ResourceInfo {
                uri: "file:///logs/app.log".to_string(),
                name: "Application Logs".to_string(),
                description: None,
                mime_type: Some("text/plain".to_string()),
            }
        ])
    }

    fn read(uri: String) -> Result<Vec<resources::Resource>, errors::Error> {
        eprintln!("WASM: Read resource {uri}");
        todo!("Implement read for Resource");
    }
}
// endregion

// region Prompts
use exports::mcp_wasm::server::prompts;

impl prompts::Guest for MyServer {
    fn get_list() -> Vec<prompts::Prompt> {
        vec![prompts::Prompt {
            name: "List".to_string(),
            description: "Lists all prompts".to_string(),
            arguments: vec![prompts::PromptArgument {
                name: "prompt".to_string(),
                description: "The prompt message".to_string(),
                required: false,
            }],
        }]
    }
}
// endregion

// region Tools
use exports::mcp_wasm::server::tools;

impl tools::Guest for MyServer {
    fn get_list() -> Vec<tools::Tool> {
        vec![tools::Tool {
            name: "".to_string(),
            description: "".to_string(),
            input_schema: "".to_string(),
        }]
    }

    fn call(_:String, _: String) -> tools::ToolResponse {
        todo!("call function")
    }
}
// endregion

export!(MyServer);
