//! Autograph Native UI
//!
//! egui-based visual flow editor for HLX workflows

use eframe::egui;
use crate::flow::{Flow, Node, Edge, Position};

mod canvas;
mod palette;
mod properties;

use canvas::Canvas;
use palette::NodePalette;
use properties::PropertiesPanel;

/// Main Autograph application
pub struct AutographApp {
    /// Current flow being edited
    flow: Flow,

    /// Currently selected node ID
    selected_node: Option<String>,

    /// Canvas state
    canvas: Canvas,

    /// Node palette
    palette: NodePalette,

    /// Properties panel
    properties: PropertiesPanel,

    /// Flow name for save/load
    flow_name: String,

    /// Execution result (JSON string)
    execution_result: Option<String>,

    /// Error messages
    error_message: Option<String>,
}

impl Default for AutographApp {
    fn default() -> Self {
        Self {
            flow: Flow {
                nodes: Vec::new(),
                edges: Vec::new(),
            },
            selected_node: None,
            canvas: Canvas::default(),
            palette: NodePalette::default(),
            properties: PropertiesPanel::default(),
            flow_name: "untitled".to_string(),
            execution_result: None,
            error_message: None,
        }
    }
}

impl AutographApp {
    /// Create a new Autograph app
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    /// Add a new node to the flow
    pub fn add_node(&mut self, type_name: String, position: Position) {
        let id = format!("node_{}", self.flow.nodes.len());
        let config = match type_name.as_str() {
            "http_request" => serde_json::json!({
                "method": "GET",
                "url": "https://example.com"
            }),
            "tensor_create" => serde_json::json!({
                "rows": 2,
                "cols": 2,
                "values": [1.0, 0.0, 0.0, 1.0]
            }),
            "tensor_op" => serde_json::json!({
                "op": "dot"
            }),
            _ => serde_json::json!({}),
        };

        self.flow.nodes.push(Node {
            id: id.clone(),
            type_name,
            config,
            position: Some(position),
        });

        self.selected_node = Some(id);
    }

    /// Delete selected node
    pub fn delete_selected_node(&mut self) {
        if let Some(node_id) = &self.selected_node {
            // Remove node
            self.flow.nodes.retain(|n| &n.id != node_id);

            // Remove connected edges
            self.flow.edges.retain(|e| {
                e.source != *node_id && e.target != *node_id
            });

            self.selected_node = None;
        }
    }

    /// Add edge between two nodes
    pub fn add_edge(&mut self, source: String, target: String) {
        // Check if edge already exists
        let exists = self.flow.edges.iter().any(|e| {
            e.source == source && e.target == target
        });

        if !exists {
            self.flow.edges.push(Edge {
                source,
                target,
                source_handle: None,
                target_handle: None,
            });
        }
    }

    /// Compile flow to HLX
    pub fn compile_flow(&mut self) {
        let source = self.flow.compile_to_hlx();

        // Save to file
        let path = format!("flows/{}.hlxa", self.flow_name);
        if let Err(e) = std::fs::write(&path, &source) {
            self.error_message = Some(format!("Failed to save: {}", e));
        } else {
            self.error_message = None;
            self.execution_result = Some(format!("Compiled successfully to {}", path));
        }
    }

    /// Execute flow with input
    pub fn run_flow(&mut self, input: serde_json::Value) {
        use hlx_compiler::hlxa::HlxaParser;
        use hlx_compiler::parser::Parser;
        use hlx_compiler::lower::lower_to_crate;
        use hlx_runtime::config::RuntimeConfig;
        use hlx_runtime::execute_with_config;

        // First compile
        self.compile_flow();

        if self.error_message.is_some() {
            return;
        }

        // Load and execute
        let path = format!("flows/{}.hlxa", self.flow_name);
        match std::fs::read_to_string(&path) {
            Ok(source) => {
                let parser = HlxaParser;
                match parser.parse(&source) {
                    Ok(program) => {
                        match lower_to_crate(&program) {
                            Ok(krate) => {
                                let mut config = RuntimeConfig::default();
                                config.main_input = Some(input.to_string());

                                match execute_with_config(&krate, &config) {
                                    Ok(result) => {
                                        match result.to_json() {
                                            Ok(json) => {
                                                self.execution_result = Some(serde_json::to_string_pretty(&json).unwrap());
                                                self.error_message = None;
                                            }
                                            Err(e) => {
                                                self.error_message = Some(format!("JSON conversion error: {}", e));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        self.error_message = Some(format!("Runtime error: {}", e));
                                    }
                                }
                            }
                            Err(e) => {
                                self.error_message = Some(format!("Lowering error: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Parse error: {}", e));
                    }
                }
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to read compiled flow: {}", e));
            }
        }
    }

    /// Save flow to JSON
    pub fn save_flow(&mut self) {
        let json = serde_json::to_string_pretty(&self.flow).unwrap();
        let path = format!("flows/{}.json", self.flow_name);
        if let Err(e) = std::fs::write(&path, json) {
            self.error_message = Some(format!("Failed to save: {}", e));
        } else {
            self.error_message = None;
            self.execution_result = Some(format!("Saved to {}", path));
        }
    }

    /// Load flow from JSON
    pub fn load_flow(&mut self, name: String) {
        let path = format!("flows/{}.json", name);
        match std::fs::read_to_string(&path) {
            Ok(json) => {
                match serde_json::from_str(&json) {
                    Ok(flow) => {
                        self.flow = flow;
                        self.flow_name = name;
                        self.error_message = None;
                        self.execution_result = Some(format!("Loaded from {}", path));
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed to parse flow: {}", e));
                    }
                }
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to load: {}", e));
            }
        }
    }
}

impl eframe::App for AutographApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Autograph");
                ui.separator();

                ui.label("Flow:");
                ui.text_edit_singleline(&mut self.flow_name);

                if ui.button("Save").clicked() {
                    self.save_flow();
                }

                if ui.button("Compile").clicked() {
                    self.compile_flow();
                }

                if ui.button("Run").clicked() {
                    self.run_flow(serde_json::json!(null));
                }

                if ui.button("New").clicked() {
                    self.flow = Flow { nodes: Vec::new(), edges: Vec::new() };
                    self.selected_node = None;
                }
            });
        });

        // Node palette (left side)
        egui::SidePanel::left("palette").min_width(200.0).show(ctx, |ui| {
            self.palette.show(ui, &mut self.flow, &mut self.selected_node);
        });

        // Properties panel (right side)
        let mut delete_requested = false;
        egui::SidePanel::right("properties").min_width(250.0).show(ctx, |ui| {
            delete_requested = self.properties.show(ui, &mut self.flow, &mut self.selected_node);
        });

        if delete_requested {
            self.delete_selected_node();
        }

        // Bottom panel for results/errors
        egui::TopBottomPanel::bottom("output").show(ctx, |ui| {
            ui.heading("Output");

            if let Some(error) = &self.error_message {
                ui.colored_label(egui::Color32::RED, error);
            }

            if let Some(result) = &self.execution_result {
                ui.monospace(result);
            }
        });

        // Central canvas
        egui::CentralPanel::default().show(ctx, |ui| {
            self.canvas.show(ui, &mut self.flow, &mut self.selected_node);
        });
    }
}

/// Launch the Autograph UI
pub fn run() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_title("Autograph - Visual Workflow Automation"),
        ..Default::default()
    };

    eframe::run_native(
        "Autograph",
        options,
        Box::new(|cc| Ok(Box::new(AutographApp::new(cc)))),
    )
}
