//! Execution Timeline Panel
//!
//! Shows chronological execution of nodes with ability to inspect
//! any point in the execution history.

use eframe::egui;

/// Execution timeline state
#[derive(Default)]
pub struct Timeline {
    /// Currently selected timeline entry
    selected_entry: Option<usize>,
}

/// Entry in the execution timeline
#[derive(Debug, Clone)]
pub struct TimelineEntry {
    pub node_id: String,
    pub node_name: String,
    pub timestamp_ms: u64,
    pub duration_ms: u64,
    pub state: super::ExecutionState,
    pub output: Option<String>,
}

impl Timeline {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        timeline_entries: &[TimelineEntry],
        on_entry_clicked: &mut Option<usize>,
    ) {
        ui.heading("Execution Timeline");
        ui.separator();

        if timeline_entries.is_empty() {
            ui.label("No execution yet. Run a workflow to see the timeline.");
            return;
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (idx, entry) in timeline_entries.iter().enumerate() {
                let is_selected = self.selected_entry == Some(idx);

                let (icon, color) = match &entry.state {
                    super::ExecutionState::Pending => ("⏳", egui::Color32::GRAY),
                    super::ExecutionState::Executing => ("⚡", egui::Color32::YELLOW),
                    super::ExecutionState::Completed => ("✓", egui::Color32::GREEN),
                    super::ExecutionState::Error(_) => ("❌", egui::Color32::RED),
                };

                ui.horizontal(|ui| {
                    // Timeline connector
                    if idx > 0 {
                        ui.label("|");
                    }

                    // Entry button
                    let button_text = format!(
                        "{} {} ({}ms) - {}",
                        icon, entry.node_name, entry.duration_ms, entry.timestamp_ms
                    );

                    let button = egui::Button::new(button_text)
                        .fill(if is_selected {
                            egui::Color32::from_rgb(60, 60, 80)
                        } else {
                            egui::Color32::from_rgb(40, 40, 40)
                        })
                        .min_size(egui::Vec2::new(ui.available_width() - 20.0, 30.0));

                    let response = ui.add(button);

                    if response.clicked() {
                        self.selected_entry = Some(idx);
                        *on_entry_clicked = Some(idx);
                    }

                    response.on_hover_text(format!("Node: {}\nClick to inspect", entry.node_id));
                });

                // Show output if selected
                if is_selected {
                    ui.indent("timeline_detail", |ui| {
                        ui.colored_label(color, format!("State: {:?}", entry.state));

                        if let Some(output) = &entry.output {
                            ui.label("Output:");
                            ui.add(
                                egui::TextEdit::multiline(&mut output.as_str())
                                    .desired_width(ui.available_width())
                                    .desired_rows(5)
                                    .code_editor(),
                            );
                        } else {
                            ui.label("Output: (not captured)");
                        }

                        if ui.button("🔄 Replay from here").clicked() {
                            // TODO: Implement replay
                            ui.label("Replay coming soon!");
                        }
                    });
                }
            }
        });

        ui.separator();
        ui.label(format!("Total entries: {}", timeline_entries.len()));
    }
}
