//! Workflow Templates
//!
//! Pre-built workflow templates for common automation tasks

use crate::flow::{Flow, Node, Edge, Position};
use serde_json::json;

pub struct WorkflowTemplate {
    pub name: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub create: fn() -> Flow,
}

pub fn all_templates() -> Vec<&'static WorkflowTemplate> {
    vec![
        &HTTP_TO_JSON_TO_PRINT,
        &FILE_READ_TRANSFORM_WRITE,
        &JSON_API_PIPELINE,
        &DATA_PROCESSING,
        &MATH_CALCULATOR,
    ]
}

static HTTP_TO_JSON_TO_PRINT: WorkflowTemplate = WorkflowTemplate {
    name: "HTTP → JSON → Print",
    description: "Fetch JSON from API and print result",
    category: "API",
    create: || {
        Flow {
            nodes: vec![
                Node {
                    id: "http1".to_string(),
                    type_name: "http_get".to_string(),
                    config: json!({"url": "https://api.github.com/users/octocat"}),
                    position: Some(Position { x: 100.0, y: 200.0 }),
                    breakpoint: false,
                },
                Node {
                    id: "json1".to_string(),
                    type_name: "json_parse".to_string(),
                    config: json!({}),
                    position: Some(Position { x: 300.0, y: 200.0 }),
                    breakpoint: false,
                },
                Node {
                    id: "print1".to_string(),
                    type_name: "print".to_string(),
                    config: json!({}),
                    position: Some(Position { x: 500.0, y: 200.0 }),
                    breakpoint: false,
                },
            ],
            edges: vec![
                Edge {
                    source: "http1".to_string(),
                    target: "json1".to_string(),
                    source_handle: None,
                    target_handle: None,
                },
                Edge {
                    source: "json1".to_string(),
                    target: "print1".to_string(),
                    source_handle: None,
                    target_handle: None,
                },
            ],
        }
    },
};

static FILE_READ_TRANSFORM_WRITE: WorkflowTemplate = WorkflowTemplate {
    name: "File Processing",
    description: "Read file, transform, write back",
    category: "Files",
    create: || {
        Flow {
            nodes: vec![
                Node {
                    id: "read1".to_string(),
                    type_name: "file_read".to_string(),
                    config: json!({"path": "input.txt"}),
                    position: Some(Position { x: 100.0, y: 200.0 }),
                    breakpoint: false,
                },
                Node {
                    id: "upper1".to_string(),
                    type_name: "string_upper".to_string(),
                    config: json!({}),
                    position: Some(Position { x: 300.0, y: 200.0 }),
                    breakpoint: false,
                },
                Node {
                    id: "write1".to_string(),
                    type_name: "file_write".to_string(),
                    config: json!({"path": "output.txt"}),
                    position: Some(Position { x: 500.0, y: 200.0 }),
                    breakpoint: false,
                },
            ],
            edges: vec![
                Edge {
                    source: "read1".to_string(),
                    target: "upper1".to_string(),
                    source_handle: None,
                    target_handle: None,
                },
                Edge {
                    source: "upper1".to_string(),
                    target: "write1".to_string(),
                    source_handle: None,
                    target_handle: None,
                },
            ],
        }
    },
};

static JSON_API_PIPELINE: WorkflowTemplate = WorkflowTemplate {
    name: "JSON API Pipeline",
    description: "Fetch, parse, extract, save to file",
    category: "API",
    create: || {
        Flow {
            nodes: vec![
                Node {
                    id: "http1".to_string(),
                    type_name: "http_get".to_string(),
                    config: json!({"url": "https://api.example.com/data"}),
                    position: Some(Position { x: 100.0, y: 150.0 }),
                    breakpoint: false,
                },
                Node {
                    id: "json1".to_string(),
                    type_name: "json_parse".to_string(),
                    config: json!({}),
                    position: Some(Position { x: 300.0, y: 150.0 }),
                    breakpoint: false,
                },
                Node {
                    id: "get1".to_string(),
                    type_name: "json_get".to_string(),
                    config: json!({"key": "results"}),
                    position: Some(Position { x: 500.0, y: 150.0 }),
                    breakpoint: false,
                },
                Node {
                    id: "write1".to_string(),
                    type_name: "json_write".to_string(),
                    config: json!({"path": "results.json"}),
                    position: Some(Position { x: 700.0, y: 150.0 }),
                    breakpoint: false,
                },
            ],
            edges: vec![
                Edge {
                    source: "http1".to_string(),
                    target: "json1".to_string(),
                    source_handle: None,
                    target_handle: None,
                },
                Edge {
                    source: "json1".to_string(),
                    target: "get1".to_string(),
                    source_handle: None,
                    target_handle: None,
                },
                Edge {
                    source: "get1".to_string(),
                    target: "write1".to_string(),
                    source_handle: None,
                    target_handle: None,
                },
            ],
        }
    },
};

static DATA_PROCESSING: WorkflowTemplate = WorkflowTemplate {
    name: "Data Processing",
    description: "Load JSON, transform, filter, save",
    category: "Data",
    create: || {
        Flow {
            nodes: vec![
                Node {
                    id: "read1".to_string(),
                    type_name: "json_read".to_string(),
                    config: json!({"path": "data.json"}),
                    position: Some(Position { x: 100.0, y: 200.0 }),
                    breakpoint: false,
                },
                Node {
                    id: "get1".to_string(),
                    type_name: "object_get".to_string(),
                    config: json!({"key": "items"}),
                    position: Some(Position { x: 300.0, y: 200.0 }),
                    breakpoint: false,
                },
                Node {
                    id: "len1".to_string(),
                    type_name: "array_length".to_string(),
                    config: json!({}),
                    position: Some(Position { x: 500.0, y: 200.0 }),
                    breakpoint: false,
                },
                Node {
                    id: "print1".to_string(),
                    type_name: "print".to_string(),
                    config: json!({}),
                    position: Some(Position { x: 700.0, y: 200.0 }),
                    breakpoint: false,
                },
            ],
            edges: vec![
                Edge {
                    source: "read1".to_string(),
                    target: "get1".to_string(),
                    source_handle: None,
                    target_handle: None,
                },
                Edge {
                    source: "get1".to_string(),
                    target: "len1".to_string(),
                    source_handle: None,
                    target_handle: None,
                },
                Edge {
                    source: "len1".to_string(),
                    target: "print1".to_string(),
                    source_handle: None,
                    target_handle: None,
                },
            ],
        }
    },
};

static MATH_CALCULATOR: WorkflowTemplate = WorkflowTemplate {
    name: "Math Calculator",
    description: "Chain math operations",
    category: "Math",
    create: || {
        Flow {
            nodes: vec![
                Node {
                    id: "add1".to_string(),
                    type_name: "math_add".to_string(),
                    config: json!({"value": 10}),
                    position: Some(Position { x: 100.0, y: 200.0 }),
                    breakpoint: false,
                },
                Node {
                    id: "mult1".to_string(),
                    type_name: "math_multiply".to_string(),
                    config: json!({"value": 2}),
                    position: Some(Position { x: 300.0, y: 200.0 }),
                    breakpoint: false,
                },
                Node {
                    id: "sqrt1".to_string(),
                    type_name: "math_sqrt".to_string(),
                    config: json!({}),
                    position: Some(Position { x: 500.0, y: 200.0 }),
                    breakpoint: false,
                },
                Node {
                    id: "print1".to_string(),
                    type_name: "print".to_string(),
                    config: json!({}),
                    position: Some(Position { x: 700.0, y: 200.0 }),
                    breakpoint: false,
                },
            ],
            edges: vec![
                Edge {
                    source: "add1".to_string(),
                    target: "mult1".to_string(),
                    source_handle: None,
                    target_handle: None,
                },
                Edge {
                    source: "mult1".to_string(),
                    target: "sqrt1".to_string(),
                    source_handle: None,
                    target_handle: None,
                },
                Edge {
                    source: "sqrt1".to_string(),
                    target: "print1".to_string(),
                    source_handle: None,
                    target_handle: None,
                },
            ],
        }
    },
};
