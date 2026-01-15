//! Node palette panel for dragging new nodes onto canvas

use eframe::egui;
use crate::flow::Position;

/// Node palette state
#[derive(Default)]
pub struct NodePalette {}

impl NodePalette {
    fn get_node_defs(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        // Get all nodes from registry
        crate::nodes::all_nodes()
            .into_iter()
            .map(|def| (def.name, def.category, def.description))
            .collect()
    }

    pub fn show(&mut self, ui: &mut egui::Ui, flow: &mut crate::flow::Flow, selected_node: &mut Option<String>) {
        ui.heading("Node Palette");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Group nodes by category with stable ordering
            use std::collections::BTreeMap;
            let mut categories = BTreeMap::new();
            for (name, category, description) in self.get_node_defs() {
                categories
                    .entry(category)
                    .or_insert_with(Vec::new)
                    .push((name, description));
            }

            for (category, nodes) in categories {
                ui.collapsing(category, |ui| {
                    for (name, description) in nodes {
                        let button = egui::Button::new(name)
                            .min_size(egui::Vec2::new(ui.available_width(), 30.0));

                        let response = ui.add(button);

                        if response.clicked() {
                            // Add node to canvas with default config
                            use crate::flow::Node;

                            let node_count = flow.nodes.len();
                            let id = format!("node_{}", node_count);

                            // Get default config from node registry
                            let config = crate::nodes::all_nodes()
                                .into_iter()
                                .find(|def| def.name == name)
                                .map(|def| (def.default_config)())
                                .unwrap_or(serde_json::json!({}));

                            flow.nodes.push(Node {
                                id: id.clone(),
                                type_name: name.to_string(),
                                config,
                                position: Some(Position {
                                    x: 300.0 + (node_count as f32 * 20.0),
                                    y: 200.0 + (node_count as f32 * 20.0),
                                }),
                            });

                            *selected_node = Some(id);
                        }

                        response.on_hover_text(description);
                    }
                });
            }
        });

        ui.separator();
        ui.label("Click a node to add it to the canvas");
    }
}
