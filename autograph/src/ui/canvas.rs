//! Canvas for drawing and manipulating flow nodes and edges

use eframe::egui;

/// Canvas state and rendering
#[derive(Default)]
pub struct Canvas {
    /// Pan offset
    pan_offset: egui::Vec2,

    /// Zoom level
    zoom: f32,

    /// Node being dragged
    dragging_node: Option<String>,

    /// Edge being drawn (from node_id)
    drawing_edge: Option<String>,

    /// Mouse position for edge preview
    mouse_pos: egui::Pos2,
}

impl Canvas {
    const NODE_WIDTH: f32 = 150.0;
    const NODE_HEIGHT: f32 = 60.0;
    const NODE_ROUNDING: f32 = 5.0;
    const EDGE_THICKNESS: f32 = 2.0;

    pub fn show(&mut self, ui: &mut egui::Ui, flow: &mut crate::flow::Flow, selected_node: &mut Option<String>) {
        // Initialize zoom if needed
        if self.zoom == 0.0 {
            self.zoom = 1.0;
        }

        let (response, painter) = ui.allocate_painter(
            ui.available_size(),
            egui::Sense::click_and_drag(),
        );

        // Store mouse position
        if let Some(pos) = response.hover_pos() {
            self.mouse_pos = pos;
        }

        // Handle pan with middle mouse or right drag
        if response.dragged_by(egui::PointerButton::Middle)
            || (response.dragged() && ui.input(|i| i.modifiers.shift)) {
            self.pan_offset += response.drag_delta();
        }

        // Handle zoom with scroll
        if response.hovered() {
            ui.input(|i| {
                if i.smooth_scroll_delta.y != 0.0 {
                    self.zoom *= 1.0 + i.smooth_scroll_delta.y * 0.001;
                    self.zoom = self.zoom.clamp(0.1, 3.0);
                }
            });
        }

        // Transform from screen space to canvas space
        let to_canvas = |screen_pos: egui::Pos2| -> egui::Pos2 {
            egui::Pos2::new(
                (screen_pos.x - self.pan_offset.x) / self.zoom,
                (screen_pos.y - self.pan_offset.y) / self.zoom,
            )
        };

        let to_screen = |canvas_pos: egui::Pos2| -> egui::Pos2 {
            egui::Pos2::new(
                canvas_pos.x * self.zoom + self.pan_offset.x,
                canvas_pos.y * self.zoom + self.pan_offset.y,
            )
        };

        // Draw grid
        self.draw_grid(&painter, response.rect, self.zoom, self.pan_offset);

        // Draw edges
        for edge in &flow.edges {
            if let (Some(source_node), Some(target_node)) = (
                flow.nodes.iter().find(|n| n.id == edge.source),
                flow.nodes.iter().find(|n| n.id == edge.target),
            ) {
                if let (Some(source_pos), Some(target_pos)) = (&source_node.position, &target_node.position) {
                    let start = to_screen(egui::Pos2::new(
                        source_pos.x + Self::NODE_WIDTH / 2.0,
                        source_pos.y + Self::NODE_HEIGHT,
                    ));
                    let end = to_screen(egui::Pos2::new(
                        target_pos.x + Self::NODE_WIDTH / 2.0,
                        target_pos.y,
                    ));

                    // Draw bezier curve for edge
                    self.draw_edge(&painter, start, end, egui::Color32::GRAY);
                }
            }
        }

        // Draw edge preview if currently drawing
        if let Some(source_id) = &self.drawing_edge {
            if let Some(source_node) = flow.nodes.iter().find(|n| &n.id == source_id) {
                if let Some(source_pos) = &source_node.position {
                    let start = to_screen(egui::Pos2::new(
                        source_pos.x + Self::NODE_WIDTH / 2.0,
                        source_pos.y + Self::NODE_HEIGHT,
                    ));
                    self.draw_edge(&painter, start, self.mouse_pos, egui::Color32::LIGHT_GRAY);
                }
            }
        }

        // Draw nodes
        let mut nodes_to_draw = Vec::new();
        for node in &flow.nodes {
            if let Some(pos) = &node.position {
                nodes_to_draw.push((node.id.clone(), node.type_name.clone(), *pos));
            }
        }

        for (node_id, type_name, pos) in nodes_to_draw {
            let is_selected = selected_node.as_ref() == Some(&node_id);
            let screen_pos = to_screen(egui::Pos2::new(pos.x, pos.y));

            let node_rect = egui::Rect::from_min_size(
                screen_pos,
                egui::Vec2::new(Self::NODE_WIDTH * self.zoom, Self::NODE_HEIGHT * self.zoom),
            );

            // Node interaction
            let node_response = ui.interact(
                node_rect,
                egui::Id::new(&node_id),
                egui::Sense::click_and_drag(),
            );

            // Handle dragging
            if node_response.dragged() {
                if self.dragging_node.is_none() {
                    self.dragging_node = Some(node_id.clone());
                }

                if self.dragging_node.as_ref() == Some(&node_id) {
                    // Update node position
                    if let Some(node) = flow.nodes.iter_mut().find(|n| n.id == node_id) {
                        if let Some(node_pos) = &mut node.position {
                            node_pos.x += node_response.drag_delta().x / self.zoom;
                            node_pos.y += node_response.drag_delta().y / self.zoom;
                        }
                    }
                }
            }

            // Stop dragging
            if node_response.drag_stopped() {
                if self.dragging_node.as_ref() == Some(&node_id) {
                    self.dragging_node = None;
                }
            }

            // Handle selection
            if node_response.clicked() {
                *selected_node = Some(node_id.clone());
            }

            // Handle edge creation (ctrl+click)
            if node_response.clicked() && ui.input(|i| i.modifiers.ctrl) {
                if let Some(source) = &self.drawing_edge {
                    use crate::flow::Edge;

                    // Complete edge - check if edge already exists
                    let exists = flow.edges.iter().any(|e| {
                        e.source == *source && e.target == node_id
                    });

                    if !exists {
                        flow.edges.push(Edge {
                            source: source.clone(),
                            target: node_id.clone(),
                            source_handle: None,
                            target_handle: None,
                        });
                    }
                    self.drawing_edge = None;
                } else {
                    // Start edge
                    self.drawing_edge = Some(node_id.clone());
                }
            }

            // Draw node
            self.draw_node(&painter, node_rect, &type_name, is_selected);
        }

        // Cancel edge drawing on escape
        ui.input(|i| {
            if i.key_pressed(egui::Key::Escape) {
                self.drawing_edge = None;
            }
        });

        // Delete selected node on Delete key
        ui.input(|i| {
            if i.key_pressed(egui::Key::Delete) {
                if let Some(node_id) = selected_node {
                    // Remove node
                    flow.nodes.retain(|n| &n.id != node_id);

                    // Remove connected edges
                    flow.edges.retain(|e| {
                        e.source != *node_id && e.target != *node_id
                    });

                    *selected_node = None;
                }
            }
        });

        // Click on empty canvas to deselect
        if response.clicked() && !ui.input(|i| i.modifiers.ctrl) {
            *selected_node = None;
        }

        // Instructions
        ui.label("Drag nodes to move | Ctrl+Click to connect | Delete key to remove | Shift+Drag to pan");
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: egui::Rect, zoom: f32, offset: egui::Vec2) {
        let grid_spacing = 50.0 * zoom;
        let color = egui::Color32::from_gray(30);

        // Vertical lines
        let start_x = (rect.min.x - offset.x) - ((rect.min.x - offset.x) % grid_spacing);
        let mut x = start_x;
        while x < rect.max.x {
            painter.line_segment(
                [egui::Pos2::new(x, rect.min.y), egui::Pos2::new(x, rect.max.y)],
                egui::Stroke::new(1.0, color),
            );
            x += grid_spacing;
        }

        // Horizontal lines
        let start_y = (rect.min.y - offset.y) - ((rect.min.y - offset.y) % grid_spacing);
        let mut y = start_y;
        while y < rect.max.y {
            painter.line_segment(
                [egui::Pos2::new(rect.min.x, y), egui::Pos2::new(rect.max.x, y)],
                egui::Stroke::new(1.0, color),
            );
            y += grid_spacing;
        }
    }

    fn draw_node(&self, painter: &egui::Painter, rect: egui::Rect, type_name: &str, is_selected: bool) {
        // Node colors by type
        let (bg_color, text_color) = match type_name {
            "start" => (egui::Color32::from_rgb(50, 150, 50), egui::Color32::WHITE),
            "http_request" => (egui::Color32::from_rgb(70, 130, 180), egui::Color32::WHITE),
            "json_parse" => (egui::Color32::from_rgb(200, 120, 50), egui::Color32::WHITE),
            "tensor_create" => (egui::Color32::from_rgb(150, 50, 150), egui::Color32::WHITE),
            "tensor_op" => (egui::Color32::from_rgb(180, 50, 100), egui::Color32::WHITE),
            "print" => (egui::Color32::from_rgb(100, 100, 100), egui::Color32::WHITE),
            _ => (egui::Color32::DARK_GRAY, egui::Color32::WHITE),
        };

        // Draw selection highlight
        if is_selected {
            painter.rect(
                rect.expand(3.0),
                Self::NODE_ROUNDING + 1.0,
                egui::Color32::TRANSPARENT,
                egui::Stroke::new(2.0, egui::Color32::YELLOW),
            );
        }

        // Draw node body
        painter.rect_filled(rect, Self::NODE_ROUNDING, bg_color);
        painter.rect_stroke(rect, Self::NODE_ROUNDING, egui::Stroke::new(1.0, egui::Color32::BLACK));

        // Draw type name
        let text_pos = rect.center() - egui::Vec2::new(0.0, 0.0);
        painter.text(
            text_pos,
            egui::Align2::CENTER_CENTER,
            type_name,
            egui::FontId::proportional(14.0),
            text_color,
        );
    }

    fn draw_edge(&self, painter: &egui::Painter, start: egui::Pos2, end: egui::Pos2, color: egui::Color32) {
        // Simple bezier curve for edges
        let ctrl_offset = (end.y - start.y).abs() * 0.5;
        let ctrl1 = egui::Pos2::new(start.x, start.y + ctrl_offset);
        let ctrl2 = egui::Pos2::new(end.x, end.y - ctrl_offset);

        // Draw bezier with line segments
        let segments = 20;
        for i in 0..segments {
            let t1 = i as f32 / segments as f32;
            let t2 = (i + 1) as f32 / segments as f32;

            let p1 = self.bezier_point(start, ctrl1, ctrl2, end, t1);
            let p2 = self.bezier_point(start, ctrl1, ctrl2, end, t2);

            painter.line_segment(
                [p1, p2],
                egui::Stroke::new(Self::EDGE_THICKNESS, color),
            );
        }
    }

    fn bezier_point(&self, p0: egui::Pos2, p1: egui::Pos2, p2: egui::Pos2, p3: egui::Pos2, t: f32) -> egui::Pos2 {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;

        egui::Pos2::new(
            mt3 * p0.x + 3.0 * mt2 * t * p1.x + 3.0 * mt * t2 * p2.x + t3 * p3.x,
            mt3 * p0.y + 3.0 * mt2 * t * p1.y + 3.0 * mt * t2 * p2.y + t3 * p3.y,
        )
    }
}
