//! Node palette panel for dragging new nodes onto canvas

use eframe::egui;
use crate::flow::Position;

/// Node palette state
#[derive(Default)]
pub struct NodePalette {
    /// Available node types
    node_types: Vec<NodeType>,
}

struct NodeType {
    name: String,
    description: String,
    category: String,
}

impl NodePalette {
    pub fn show(&mut self, ui: &mut egui::Ui, flow: &mut crate::flow::Flow, selected_node: &mut Option<String>) {
        // Initialize node types if empty
        if self.node_types.is_empty() {
            self.node_types = vec![
                NodeType {
                    name: "start".to_string(),
                    description: "Entry point for the workflow".to_string(),
                    category: "Control".to_string(),
                },
                NodeType {
                    name: "print".to_string(),
                    description: "Print value to console".to_string(),
                    category: "Debug".to_string(),
                },
                NodeType {
                    name: "http_request".to_string(),
                    description: "Make HTTP request".to_string(),
                    category: "HTTP".to_string(),
                },
                NodeType {
                    name: "json_parse".to_string(),
                    description: "Parse JSON string".to_string(),
                    category: "Data".to_string(),
                },
                NodeType {
                    name: "tensor_create".to_string(),
                    description: "Create a new tensor".to_string(),
                    category: "ML/GPU".to_string(),
                },
                NodeType {
                    name: "tensor_op".to_string(),
                    description: "Tensor operations (matmul, add)".to_string(),
                    category: "ML/GPU".to_string(),
                },
            ];
        }

        ui.heading("Node Palette");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Group by category
            let mut categories = std::collections::HashMap::new();
            for node_type in &self.node_types {
                categories
                    .entry(&node_type.category)
                    .or_insert_with(Vec::new)
                    .push(node_type);
            }

            for (category, nodes) in categories {
                ui.collapsing(category, |ui| {
                    for node_type in nodes {
                        let button = egui::Button::new(&node_type.name)
                            .min_size(egui::Vec2::new(ui.available_width(), 30.0));

                        let response = ui.add(button);

                        if response.clicked() {
                            // Add node to center of canvas
                            use crate::flow::Node;

                            let node_count = flow.nodes.len();
                            let id = format!("node_{}", node_count);
                            let config = match node_type.name.as_str() {
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

                            flow.nodes.push(Node {
                                id: id.clone(),
                                type_name: node_type.name.clone(),
                                config,
                                position: Some(Position {
                                    x: 300.0 + (node_count as f32 * 20.0),
                                    y: 200.0 + (node_count as f32 * 20.0),
                                }),
                            });

                            *selected_node = Some(id);
                        }

                        response.on_hover_text(&node_type.description);
                    }
                });
            }
        });

        ui.separator();
        ui.label("Click a node to add it to the canvas");
    }
}
