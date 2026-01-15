//! Autograph Native UI
//!
//! egui-based visual flow editor for HLX workflows

use eframe::egui;
use crate::flow::{Flow, Node, Edge, Position};
use std::collections::HashMap;

mod canvas;
mod palette;
mod properties;
mod timeline;

use canvas::Canvas;
use palette::NodePalette;
use properties::PropertiesPanel;
use timeline::{Timeline, TimelineEntry};

/// Execution state for a node
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionState {
    Pending,
    Executing,
    Completed,
    Error(String),
}

/// Execution result for a node
#[derive(Debug, Clone)]
pub struct NodeExecution {
    pub state: ExecutionState,
    pub output: Option<String>,
    pub duration_ms: Option<u64>,
}

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

    /// Execution state for each node
    node_executions: HashMap<String, NodeExecution>,

    /// Execution log entries
    execution_log: Vec<String>,

    /// Whether execution is in progress
    executing: bool,

    /// Node being inspected (for data view)
    inspected_node: Option<String>,

    /// Timeline state
    timeline: Timeline,

    /// Timeline entries
    timeline_entries: Vec<TimelineEntry>,

    /// Selected backend for execution
    backend_selection: BackendType,

    /// Dark mode enabled
    dark_mode: bool,

    /// Show mini-map
    show_minimap: bool,
}

/// Backend type for execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendType {
    Auto,
    Cpu,
    Vulkan,
}

impl BackendType {
    fn as_str(&self) -> &'static str {
        match self {
            BackendType::Auto => "Auto",
            BackendType::Cpu => "CPU",
            BackendType::Vulkan => "GPU (Vulkan)",
        }
    }

    fn to_runtime_backend(&self) -> hlx_runtime::config::BackendType {
        match self {
            BackendType::Auto => hlx_runtime::config::BackendType::Auto,
            BackendType::Cpu => hlx_runtime::config::BackendType::Cpu,
            BackendType::Vulkan => hlx_runtime::config::BackendType::Vulkan,
        }
    }
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
            node_executions: HashMap::new(),
            execution_log: Vec::new(),
            executing: false,
            inspected_node: None,
            timeline: Timeline::default(),
            timeline_entries: Vec::new(),
            backend_selection: BackendType::Auto,
            dark_mode: true,  // Default to dark mode
            show_minimap: true,  // Show minimap by default
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
            breakpoint: false,
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

    /// Clear execution state
    pub fn clear_execution(&mut self) {
        self.node_executions.clear();
        self.execution_log.clear();
        self.timeline_entries.clear();
        self.executing = false;
        self.execution_result = None;
        self.error_message = None;
    }

    /// Mark all nodes as pending
    fn mark_nodes_pending(&mut self) {
        for node in &self.flow.nodes {
            self.node_executions.insert(
                node.id.clone(),
                NodeExecution {
                    state: ExecutionState::Pending,
                    output: None,
                    duration_ms: None,
                },
            );
        }
    }

    /// Execute flow with input
    pub fn run_flow(&mut self, input: serde_json::Value) {
        use hlx_compiler::hlxa::HlxaParser;
        use hlx_compiler::parser::Parser;
        use hlx_compiler::lower::lower_to_crate;
        use hlx_runtime::config::RuntimeConfig;
        use hlx_runtime::execute_with_config;
        use std::time::Instant;

        // Clear previous execution
        self.clear_execution();

        // Mark all nodes as pending
        self.mark_nodes_pending();

        self.execution_log.push(format!("=== Starting execution of '{}' ===", self.flow_name));
        self.execution_log.push(format!("Backend: {}", self.backend_selection.as_str()));
        self.execution_log.push(format!("Input: {}", serde_json::to_string(&input).unwrap_or("null".to_string())));

        // First compile
        self.compile_flow();

        if self.error_message.is_some() {
            self.execution_log.push("❌ Compilation failed".to_string());
            return;
        }

        self.execution_log.push("✓ Compilation successful".to_string());
        self.executing = true;

        // Load and execute
        let path = format!("flows/{}.hlxa", self.flow_name);
        match std::fs::read_to_string(&path) {
            Ok(source) => {
                let parser = HlxaParser;
                match parser.parse(&source) {
                    Ok(program) => {
                        self.execution_log.push("✓ Parsed HLX source".to_string());

                        match lower_to_crate(&program) {
                            Ok(krate) => {
                                self.execution_log.push("✓ Lowered to IR".to_string());
                                self.execution_log.push("⚡ Executing workflow...".to_string());

                                let mut config = RuntimeConfig::default();
                                config.main_input = Some(input.to_string());
                                config.backend = self.backend_selection.to_runtime_backend();

                                let start = Instant::now();
                                match execute_with_config(&krate, &config) {
                                    Ok(result) => {
                                        let duration = start.elapsed();
                                        self.execution_log.push(format!("✓ Execution completed in {}ms", duration.as_millis()));

                                        // Mark all nodes as completed and create timeline entries
                                        let mut timeline_offset_ms = 0u64;
                                        for node in &self.flow.nodes {
                                            // Simulate per-node timing (in reality, all execute together)
                                            // In Phase 4 Part 2, we'll get real per-node timing from runtime
                                            let node_duration = duration.as_millis() as u64 / self.flow.nodes.len() as u64;

                                            if let Some(exec) = self.node_executions.get_mut(&node.id) {
                                                exec.state = ExecutionState::Completed;
                                                exec.duration_ms = Some(node_duration);
                                            }

                                            // Add timeline entry
                                            self.timeline_entries.push(TimelineEntry {
                                                node_id: node.id.clone(),
                                                node_name: node.type_name.clone(),
                                                timestamp_ms: timeline_offset_ms,
                                                duration_ms: node_duration,
                                                state: ExecutionState::Completed,
                                                output: None, // TODO: Capture from runtime
                                            });

                                            timeline_offset_ms += node_duration;
                                        }

                                        match result.to_json() {
                                            Ok(json) => {
                                                let result_str = serde_json::to_string_pretty(&json).unwrap();
                                                self.execution_result = Some(result_str.clone());
                                                self.execution_log.push(format!("Result: {}", result_str));
                                                self.error_message = None;
                                            }
                                            Err(e) => {
                                                self.error_message = Some(format!("JSON conversion error: {}", e));
                                                self.execution_log.push(format!("❌ JSON conversion failed: {}", e));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        self.error_message = Some(format!("Runtime error: {}", e));
                                        self.execution_log.push(format!("❌ Runtime error: {}", e));

                                        // Mark all nodes as error
                                        for node in &self.flow.nodes {
                                            if let Some(exec) = self.node_executions.get_mut(&node.id) {
                                                exec.state = ExecutionState::Error(e.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                self.error_message = Some(format!("Lowering error: {}", e));
                                self.execution_log.push(format!("❌ Lowering error: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Parse error: {}", e));
                        self.execution_log.push(format!("❌ Parse error: {}", e));
                    }
                }
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to read compiled flow: {}", e));
                self.execution_log.push(format!("❌ Failed to read: {}", e));
            }
        }

        self.executing = false;
        self.execution_log.push("=== Execution finished ===".to_string());
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
        // Handle keyboard shortcuts
        ctx.input(|i| {
            // Ctrl+S: Save
            if i.modifiers.ctrl && i.key_pressed(egui::Key::S) {
                self.save_flow();
            }

            // Ctrl+R: Run
            if i.modifiers.ctrl && i.key_pressed(egui::Key::R) {
                self.run_flow(serde_json::json!(null));
            }

            // Ctrl+B: Compile
            if i.modifiers.ctrl && i.key_pressed(egui::Key::B) {
                self.compile_flow();
            }

            // Ctrl+N: New
            if i.modifiers.ctrl && i.key_pressed(egui::Key::N) {
                self.flow = Flow { nodes: Vec::new(), edges: Vec::new() };
                self.selected_node = None;
                self.clear_execution();
            }

            // Ctrl+K: Clear execution
            if i.modifiers.ctrl && i.key_pressed(egui::Key::K) {
                self.clear_execution();
            }

            // F5: Run (alternative)
            if i.key_pressed(egui::Key::F5) {
                self.run_flow(serde_json::json!(null));
            }
        });

        // Apply theme
        ctx.set_visuals(if self.dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });

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
                    self.clear_execution();
                }

                ui.separator();

                if ui.button("Clear Execution").clicked() {
                    self.clear_execution();
                }

                ui.separator();

                // Templates menu
                ui.menu_button("Templates ▼", |ui| {
                    use std::collections::BTreeMap;
                    let mut categories = BTreeMap::new();

                    for template in crate::templates::all_templates() {
                        categories
                            .entry(template.category)
                            .or_insert_with(Vec::new)
                            .push(template);
                    }

                    for (category, templates) in categories {
                        ui.menu_button(category, |ui| {
                            for template in templates {
                                if ui.button(template.name).on_hover_text(template.description).clicked() {
                                    self.flow = (template.create)();
                                    self.clear_execution();
                                    ui.close_menu();
                                }
                            }
                        });
                    }
                });

                ui.separator();

                // Backend selection
                ui.label("Backend:");
                egui::ComboBox::from_id_source("backend_selector")
                    .selected_text(self.backend_selection.as_str())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.backend_selection, BackendType::Auto, "Auto (Prefer GPU)");
                        ui.selectable_value(&mut self.backend_selection, BackendType::Cpu, "CPU Only");
                        ui.selectable_value(&mut self.backend_selection, BackendType::Vulkan, "GPU (Vulkan)");
                    });

                ui.separator();

                // Theme toggle
                if ui.button(if self.dark_mode { "☀ Light Mode" } else { "🌙 Dark Mode" }).clicked() {
                    self.dark_mode = !self.dark_mode;
                }

                // Mini-map toggle
                if ui.button(if self.show_minimap { "🗺 Hide Map" } else { "🗺 Show Map" }).clicked() {
                    self.show_minimap = !self.show_minimap;
                }
            });
        });

        // Node palette (left side) - top half
        egui::SidePanel::left("palette").min_width(200.0).max_width(250.0).show(ctx, |ui| {
            // Split into two sections: palette and timeline
            let total_height = ui.available_height();

            // Palette section (scrollable)
            ui.push_id("palette_section", |ui| {
                ui.set_max_height(total_height * 0.5);
                self.palette.show(ui, &mut self.flow, &mut self.selected_node);
            });

            ui.separator();

            // Timeline section (scrollable)
            ui.push_id("timeline_section", |ui| {
                ui.set_max_height(total_height * 0.5);
                let mut clicked_entry = None;
                self.timeline.show(ui, &self.timeline_entries, &mut clicked_entry);

                if let Some(idx) = clicked_entry {
                    if let Some(entry) = self.timeline_entries.get(idx) {
                        // Select the node on canvas
                        self.selected_node = Some(entry.node_id.clone());
                    }
                }
            });
        });

        // Properties panel (right side)
        let mut delete_requested = false;
        egui::SidePanel::right("properties").min_width(300.0).show(ctx, |ui| {
            delete_requested = self.properties.show(
                ui,
                &mut self.flow,
                &mut self.selected_node,
                &self.node_executions,
            );
        });

        if delete_requested {
            self.delete_selected_node();
        }

        // Bottom panel for results/errors
        egui::TopBottomPanel::bottom("output").min_height(200.0).show(ctx, |ui| {
            ui.heading("Output");
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Execution log section
                ui.collapsing("Execution Log", |ui| {
                    if self.execution_log.is_empty() {
                        ui.label("No execution yet. Click 'Run' to execute the workflow.");
                    } else {
                        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                        for log_entry in &self.execution_log {
                            if log_entry.starts_with("❌") {
                                ui.colored_label(egui::Color32::RED, log_entry);
                            } else if log_entry.starts_with("✓") {
                                ui.colored_label(egui::Color32::GREEN, log_entry);
                            } else if log_entry.starts_with("⚡") {
                                ui.colored_label(egui::Color32::YELLOW, log_entry);
                            } else {
                                ui.label(log_entry);
                            }
                        }
                    }
                });

                ui.separator();

                // Error section
                if let Some(error) = &self.error_message {
                    ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
                    ui.separator();
                }

                // Result section
                if let Some(result) = &self.execution_result {
                    ui.label("Result:");
                    ui.monospace(result);
                }
            });
        });

        // Central canvas
        egui::CentralPanel::default().show(ctx, |ui| {
            self.canvas.show(ui, &mut self.flow, &mut self.selected_node, &self.node_executions);
        });

        // Mini-map overlay
        if self.show_minimap && !self.flow.nodes.is_empty() {
            egui::Window::new("🗺 Map")
                .default_pos([ctx.screen_rect().width() - 220.0, ctx.screen_rect().height() - 220.0])
                .default_size([200.0, 200.0])
                .resizable(false)
                .collapsible(true)
                .show(ctx, |ui| {
                    // Calculate bounds of all nodes
                    let mut min_x = f32::MAX;
                    let mut min_y = f32::MAX;
                    let mut max_x = f32::MIN;
                    let mut max_y = f32::MIN;

                    for node in &self.flow.nodes {
                        if let Some(pos) = &node.position {
                            min_x = min_x.min(pos.x);
                            min_y = min_y.min(pos.y);
                            max_x = max_x.max(pos.x + 120.0);  // Approximate node width
                            max_y = max_y.max(pos.y + 60.0);   // Approximate node height
                        }
                    }

                    let world_width = max_x - min_x;
                    let world_height = max_y - min_y;
                    let world_size = world_width.max(world_height).max(1.0);

                    // Scale factor to fit in minimap
                    let minimap_size = 180.0;
                    let scale = minimap_size / world_size;

                    let (response, painter) = ui.allocate_painter(
                        egui::Vec2::new(minimap_size, minimap_size),
                        egui::Sense::hover()
                    );

                    let minimap_rect = response.rect;

                    // Draw background
                    painter.rect_filled(
                        minimap_rect,
                        0.0,
                        egui::Color32::from_rgb(30, 30, 30)
                    );

                    // Draw nodes as small rectangles
                    for node in &self.flow.nodes {
                        if let Some(pos) = &node.position {
                            let minimap_x = minimap_rect.min.x + (pos.x - min_x) * scale;
                            let minimap_y = minimap_rect.min.y + (pos.y - min_y) * scale;
                            let minimap_w = 120.0 * scale;
                            let minimap_h = 60.0 * scale;

                            let node_rect = egui::Rect::from_min_size(
                                egui::pos2(minimap_x, minimap_y),
                                egui::vec2(minimap_w.max(3.0), minimap_h.max(3.0))
                            );

                            // Color based on execution state
                            let color = if let Some(exec) = self.node_executions.get(&node.id) {
                                match &exec.state {
                                    ExecutionState::Completed => egui::Color32::from_rgb(0, 150, 0),
                                    ExecutionState::Error(_) => egui::Color32::from_rgb(200, 0, 0),
                                    ExecutionState::Executing => egui::Color32::from_rgb(200, 200, 0),
                                    ExecutionState::Pending => egui::Color32::from_rgb(80, 80, 80),
                                }
                            } else {
                                egui::Color32::from_rgb(100, 100, 100)
                            };

                            painter.rect_filled(node_rect, 1.0, color);
                        }
                    }

                    // Draw viewport indicator (current view)
                    let canvas_offset = self.canvas.offset();
                    let viewport_scale = 1.0 / self.canvas.zoom();
                    let viewport_width = 800.0 * viewport_scale;  // Approximate canvas width
                    let viewport_height = 600.0 * viewport_scale;

                    let viewport_x = minimap_rect.min.x + (canvas_offset.x - min_x) * scale;
                    let viewport_y = minimap_rect.min.y + (canvas_offset.y - min_y) * scale;
                    let viewport_w = viewport_width * scale;
                    let viewport_h = viewport_height * scale;

                    let viewport_rect = egui::Rect::from_min_size(
                        egui::pos2(viewport_x, viewport_y),
                        egui::vec2(viewport_w, viewport_h)
                    );

                    painter.rect_stroke(
                        viewport_rect,
                        0.0,
                        egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 150, 255))
                    );
                });
        }
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
