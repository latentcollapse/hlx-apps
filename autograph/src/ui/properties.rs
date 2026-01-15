//! Properties panel for editing selected nodes

use eframe::egui;

/// Properties panel state
#[derive(Default)]
pub struct PropertiesPanel {
    /// JSON editor buffer
    config_json: String,

    /// Whether JSON is being edited
    editing: bool,
}

impl PropertiesPanel {
    pub fn show(&mut self, ui: &mut egui::Ui, flow: &mut crate::flow::Flow, selected_node: &mut Option<String>) -> bool {
        let mut delete_requested = false;

        ui.heading("Properties");
        ui.separator();

        if let Some(node_id) = selected_node.clone() {
            if let Some(node) = flow.nodes.iter_mut().find(|n| n.id == node_id) {
                ui.label(format!("Node: {}", node.id));
                ui.label(format!("Type: {}", node.type_name));
                ui.separator();

                // Position
                if let Some(pos) = &mut node.position {
                    ui.label("Position:");
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(egui::DragValue::new(&mut pos.x).speed(1.0));
                        ui.label("Y:");
                        ui.add(egui::DragValue::new(&mut pos.y).speed(1.0));
                    });
                }

                ui.separator();
                ui.label("Configuration:");

                // Initialize JSON buffer if not editing
                if !self.editing {
                    self.config_json = serde_json::to_string_pretty(&node.config).unwrap();
                }

                // JSON editor
                let response = ui.add(
                    egui::TextEdit::multiline(&mut self.config_json)
                        .desired_width(ui.available_width())
                        .desired_rows(15)
                        .code_editor(),
                );

                if response.changed() {
                    self.editing = true;
                }

                if response.lost_focus() || ui.button("Apply Changes").clicked() {
                    // Try to parse and update config
                    match serde_json::from_str(&self.config_json) {
                        Ok(new_config) => {
                            node.config = new_config;
                            self.editing = false;
                        }
                        Err(e) => {
                            ui.colored_label(egui::Color32::RED, format!("Invalid JSON: {}", e));
                        }
                    }
                }

                ui.separator();

                if ui.button("Delete Node").clicked() {
                    delete_requested = true;
                }

                // Node-specific help
                ui.separator();
                ui.label("Help:");
                match node.type_name.as_str() {
                    "http_request" => {
                        ui.label("Config: { \"method\": \"GET\", \"url\": \"...\" }");
                    }
                    "tensor_create" => {
                        ui.label("Config: { \"rows\": N, \"cols\": M, \"values\": [...] }");
                    }
                    "tensor_op" => {
                        ui.label("Config: { \"op\": \"dot\" | \"add\" }");
                    }
                    "json_parse" | "print" | "start" => {
                        ui.label("Config: {}");
                    }
                    _ => {}
                }
            }
        } else {
            ui.label("No node selected");
            ui.label("\nClick a node to view its properties");
        }

        delete_requested
    }
}
