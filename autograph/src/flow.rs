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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub source: String, // Node ID
    pub target: String, // Node ID
    pub source_handle: Option<String>,
    pub target_handle: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
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
            match node.type_name.as_str() {
                "start" => {
                    // Entry point - maybe just alias 'input'
                    source.push_str(&format!("    let {}_out = input;\n", node.id));
                }
                "http_request" => {
                    // Extract config
                    let url = node.config["url"].as_str().unwrap_or("");
                    let method = node.config["method"].as_str().unwrap_or("GET");
                    
                    // Find input source (naively pick first incoming edge)
                    let input_var = self.find_input_var(&node.id).unwrap_or("null".to_string());
                    
                    source.push_str(&format!("    let {}_out = http_request(\"{}\", \"{}\", {}, {{}});\n", 
                        node.id, method, url, input_var));
                }
                "json_parse" => {
                    let input_var = self.find_input_var(&node.id).unwrap_or("null".to_string());
                    source.push_str(&format!("    let {}_out = json_parse({});\n", node.id, input_var));
                }
                "tensor_op" => {
                    let op = node.config["op"].as_str().unwrap_or("dot");
                    
                    // We need to find the two inputs.
                    // Heuristic: Find all edges pointing to this node.
                    let inputs: Vec<String> = self.edges.iter()
                        .filter(|e| e.target == node.id)
                        .map(|e| format!("{}_out", e.source))
                        .collect();
                    
                    if inputs.len() >= 2 {
                        let op_func = match op {
                            "dot" => "tensor_matmul", // Intrinsic
                            "add" => "tensor_add",    // Intrinsic
                            _ => "tensor_matmul"
                        };
                        source.push_str(&format!("    let {}_out = {}({}, {});\n", node.id, op_func, inputs[0], inputs[1]));
                    } else {
                        // Fallback or error
                        source.push_str(&format!("    let {}_out = null; // Error: Missing inputs for tensor op\n", node.id));
                    }
                }
                "tensor_create" => {
                    // Config: rows, cols, values (flat array)
                    let rows = node.config["rows"].as_u64().unwrap_or(2);
                    let cols = node.config["cols"].as_u64().unwrap_or(2);
                    let vals = node.config["values"].as_array();
                    
                    // 1. Allocate
                    source.push_str(&format!("    let {}_t = tensor_new_2d({}, {});\n", node.id, rows, cols));
                    
                    // 2. Populate (unrolling loop for simplicity in this MVP)
                    if let Some(values) = vals {
                        for (i, v) in values.iter().enumerate() {
                            let r = i as u64 / cols;
                            let c = i as u64 % cols;
                            let val = v.as_f64().unwrap_or(0.0);
                            // We need to set the value. 
                            // Direct memory access via array-like syntax for tensors isn't fully sugared in parser yet?
                            // Let's use the layout knowledge: [2] is data ptr.
                            // But wait, generated code shouldn't do raw pointer arithmetic if we can help it.
                            // We should really link the stdlib. 
                            // For now, I'll inline a helper or assume a 'tensor_set_val' intrinsic exists/wrapper.
                            // Actually, tensor.hlxa uses: data[idx] = val.
                            
                            // Let's just generate raw array access if the parser supports it on pointers.
                            // Or better: Assume the runtime supports `tensor_set_2d(t, r, c, val)` intrinsic.
                            // Checking tensor.hlxa again... it manually calculates idx.
                            
                            // Let's inject a helper function at the top of the file? No, too messy.
                            // Let's assume we use the 'tensor_new_with_data' if it existed.
                            // Backup plan: Generate the raw logic inline.
                            
                            source.push_str(&format!("    // Set ({}, {}) = {}\n", r, c, val));
                            // Access data ptr (index 2)
                            source.push_str(&format!("    let {}_data = {}_t[2];\n", node.id, node.id));
                            source.push_str(&format!("    {}_data[{}] = {};\n", node.id, i, val));
                        }
                    }
                    source.push_str(&format!("    let {}_out = {}_t;\n", node.id, node.id));
                }
                "print" => {
                    let input_var = self.find_input_var(&node.id).unwrap_or("null".to_string());
                    source.push_str(&format!("    print({});\n", input_var));
                    source.push_str(&format!("    let {}_out = {};\n", node.id, input_var)); // Pass through
                }
                _ => {
                    source.push_str(&format!("    // Unknown node type: {}\n", node.type_name));
                    source.push_str(&format!("    let {}_out = null;\n", node.id));
                }
            }
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
