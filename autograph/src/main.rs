mod flow;
mod ui;

use axum::{
    extract::{Path, State},
    routing::post,
    Json, Router,
};
use clap::{Parser, Subcommand};
use serde_json::{Value as JsonValue};
use std::sync::Arc;
use std::path::PathBuf;
use tracing::{info, error};
use hlx_core::Value;
use hlx_compiler::{HlxaParser, parser::Parser as ParseTrait, lower};
use hlx_runtime::{execute_with_config, RuntimeConfig};
use flow::Flow;

#[derive(Parser)]
#[command(name = "autograph")]
#[command(about = "Visual workflow automation powered by HLX", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch the visual editor (default)
    Ui,
    /// Start the REST API server
    Server {
        /// Port to listen on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
}

struct AppState {
    flows_dir: PathBuf,
}

fn main() {
    tracing_subscriber::fmt::init();

    // Ensure flows dir exists
    std::fs::create_dir_all("flows").ok();

    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Server { port }) => {
            // Run REST API server
            run_server(port).map_err(|e| eprintln!("Server error: {}", e))
        }
        Some(Commands::Ui) | None => {
            // Run native UI (default)
            ui::run().map_err(|e| eprintln!("UI error: {}", e))
        }
    };

    if let Err(_) = result {
        std::process::exit(1);
    }
}

#[tokio::main]
async fn run_server(port: u16) -> anyhow::Result<()> {
    let state = Arc::new(AppState {
        flows_dir: PathBuf::from("flows"),
    });

    let app = Router::new()
        .route("/run/:flow_name", post(run_flow))
        .route("/deploy/:flow_name", post(deploy_flow))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Autograph server listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn deploy_flow(
    Path(flow_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(flow): Json<Flow>,
) -> Json<JsonValue> {
    info!("Deploying flow: {}", flow_name);

    let source = flow.compile_to_hlx();
    let flow_path = state.flows_dir.join(format!("{}.hlxa", flow_name));

    match std::fs::write(&flow_path, &source) {
        Ok(_) => {
            info!("Flow saved to {}", flow_path.display());
            Json(serde_json::json!({
                "status": "success",
                "message": "Flow compiled and deployed",
                "source": source
            }))
        },
        Err(e) => {
            error!("Failed to save flow: {}", e);
            Json(serde_json::json!({"error": format!("Failed to save flow: {}", e)}))
        }
    }
}


async fn run_flow(
    Path(flow_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<JsonValue>,
) -> Json<JsonValue> {
    info!("Running flow: {}", flow_name);

    let flow_path = state.flows_dir.join(format!("{}.hlxa", flow_name));
    if !flow_path.exists() {
        error!("Flow not found: {}", flow_path.display());
        return Json(serde_json::json!({"error": "Flow not found"}));
    }

    let source = match std::fs::read_to_string(&flow_path) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to read flow: {}", e);
            return Json(serde_json::json!({"error": format!("Failed to read flow: {}", e)}));
        }
    } ;

    // Compile and run
    let result = match compile_and_run(&source, payload) {
        Ok(res) => res,
        Err(e) => {
            error!("Flow execution failed: {}", e);
            return Json(serde_json::json!({"error": format!("Execution failed: {}", e)}));
        }
    };

    // Convert result back to JSON
    match result.to_json() {
        Ok(j) => Json(j),
        Err(e) => Json(serde_json::json!({"error": format!("Serialization failed: {}", e)})),
    }
}

fn compile_and_run(source: &str, input_json: JsonValue) -> anyhow::Result<Value> {
    // Parse
    let parser = HlxaParser::new();
    let ast = parser.parse(source).map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))?;

    // Lower
    let krate = lower::lower_to_crate(&ast).map_err(|e| anyhow::anyhow!("Lowering error: {:?}", e))?;

    // Setup config with main input
    let mut config = RuntimeConfig::default();
    let hlx_input = Value::from_json(input_json).map_err(|e| anyhow::anyhow!("Input conversion error: {:?}", e))?;
    
    // We pass the input as a string to main(input) for now, or we could modify the runtime to take a Value
    // The current runtime.main_input is a Option<String>
    config.main_input = Some(serde_json::to_string(&hlx_input.to_json()?)?);

    // Execute
    let result = execute_with_config(&krate, &config).map_err(|e| anyhow::anyhow!("Runtime error: {:?}", e))?;

    Ok(result)
}