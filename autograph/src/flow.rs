use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub type_name: String,
    pub config: serde_json::Value,
    pub position: Option<Position>, // For UI only
    #[serde(default)]
    pub breakpoint: bool, // For debugging
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub source: String, // Node ID
    pub target: String, // Node ID
    pub source_handle: Option<String>,
    pub target_handle: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Flow {
    pub fn compile_to_hlx(&self) -> String {
        let mut source = String::new();
        
        // Header
        source.push_str("program workflow {\n\n");
        
        // We need to topologically sort nodes to determine execution order.
        // For this MVP, we'll assume a simple linear chain or manual ordering isn't strictly enforced 
        // by the compiler yet (HLX handles variable dependencies).
        
        source.push_str("fn main(input) {\n");
        
        // 1. Generate variable declarations for each node output
        for node in &self.nodes {
            // Get input variable from first incoming edge
            let input_var = self.find_input_var(&node.id);

            // Find node definition in registry
            let node_code = if let Some(node_def) = crate::nodes::all_nodes()
                .into_iter()
                .find(|def| def.name == node.type_name)
            {
                // Generate code using registry
                (node_def.generate_code)(&node.id, &node.config, input_var.as_deref())
            } else {
                // Fallback for unknown nodes
                format!("    // Unknown node type: {}\n    let {}_out = null;\n",
                    node.type_name, node.id)
            };

            source.push_str(&node_code);
        }
        
        // Return the output of the last node (heuristic: node with no outgoing edges)
        if let Some(last_node) = self.find_leaf_node() {
            source.push_str(&format!("    return {}_out;\n", last_node.id));
        } else {
            source.push_str("    return null;\n");
        }

        source.push_str("}\n\n");
        source.push_str("}\n");
        
        source
    }
    
    fn find_input_var(&self, node_id: &str) -> Option<String> {
        self.edges.iter()
            .find(|e| e.target == node_id)
            .map(|e| format!("{}_out", e.source))
    }
    
    fn find_leaf_node(&self) -> Option<&Node> {
        // Find a node that is not a source for any edge
        self.nodes.iter().find(|n| !self.edges.iter().any(|e| e.source == n.id))
    }
}
