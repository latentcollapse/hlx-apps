//! Node Type Registry for Autograph
//!
//! Centralized definitions for all node types, their metadata,
//! config schemas, and HLX code generation logic.

use serde_json::Value as JsonValue;

/// Node type metadata and code generation
pub struct NodeDef {
    pub name: &'static str,
    pub category: &'static str,
    pub description: &'static str,
    pub default_config: fn() -> JsonValue,
    pub generate_code: fn(&str, &JsonValue, Option<&str>) -> String,
}

/// Get all registered node types
pub fn all_nodes() -> Vec<&'static NodeDef> {
    vec![
        // Control
        &START,
        &PRINT,

        // HTTP
        &HTTP_GET,
        &HTTP_POST,
        &HTTP_PUT,
        &HTTP_DELETE,
        &HTTP_REQUEST,

        // Data - JSON
        &JSON_PARSE,
        &JSON_STRINGIFY,
        &JSON_GET,
        &JSON_SET,

        // Data - String
        &STRING_CONCAT,
        &STRING_UPPER,
        &STRING_LOWER,
        &STRING_TRIM,
        &STRING_SPLIT,
        &STRING_REPLACE,
        &STRING_LENGTH,

        // Data - Array
        &ARRAY_MAP,
        &ARRAY_FILTER,
        &ARRAY_REDUCE,
        &ARRAY_SLICE,
        &ARRAY_CONCAT,
        &ARRAY_SORT,
        &ARRAY_LENGTH,

        // Data - Object
        &OBJECT_GET,
        &OBJECT_SET,
        &OBJECT_KEYS,
        &OBJECT_VALUES,
        &OBJECT_HAS_KEY,

        // Files
        &FILE_READ,
        &FILE_WRITE,
        &FILE_EXISTS,
        &FILE_DELETE,
        &FILE_LIST,
        &DIR_CREATE,
        &JSON_READ,
        &JSON_WRITE,

        // Math
        &MATH_ADD,
        &MATH_SUBTRACT,
        &MATH_MULTIPLY,
        &MATH_DIVIDE,
        &MATH_FLOOR,
        &MATH_CEIL,
        &MATH_ROUND,
        &MATH_SQRT,
        &MATH_RANDOM,

        // Type Conversion
        &TO_STRING,
        &TO_INT,
        &TO_FLOAT,

        // ML/GPU
        &TENSOR_CREATE,
        &TENSOR_MATMUL,
        &TENSOR_ADD,

        // System
        &SLEEP,
        &CAPTURE_SCREEN,
    ]
}

// Helper to get input variable from edges
fn input_var(node_id: &str, default: &str) -> String {
    format!("{{ let input_var = edges_to_{}; if input_var then input_var else {} }}", node_id, default)
}

// ====================
// CONTROL NODES
// ====================

static START: NodeDef = NodeDef {
    name: "start",
    category: "Control",
    description: "Entry point for workflow",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, _input_var| {
        format!("    let {}_out = input;\n", node_id)
    },
};

static PRINT: NodeDef = NodeDef {
    name: "print",
    category: "Debug",
    description: "Print value to console",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("null");
        format!("    print({});\n    let {}_out = {};\n", input, node_id, input)
    },
};

// ====================
// HTTP NODES
// ====================

static HTTP_GET: NodeDef = NodeDef {
    name: "http_get",
    category: "HTTP",
    description: "HTTP GET request",
    default_config: || serde_json::json!({"url": "https://example.com"}),
    generate_code: |node_id, config, _input_var| {
        let url = config["url"].as_str().unwrap_or("https://example.com");
        format!("    let {}_out = http_request(\"GET\", \"{}\", null, {{}});\n", node_id, url)
    },
};

static HTTP_POST: NodeDef = NodeDef {
    name: "http_post",
    category: "HTTP",
    description: "HTTP POST request",
    default_config: || serde_json::json!({"url": "https://example.com"}),
    generate_code: |node_id, config, input_var| {
        let url = config["url"].as_str().unwrap_or("https://example.com");
        let body = input_var.unwrap_or("null");
        format!("    let {}_out = http_request(\"POST\", \"{}\", {}, {{}});\n", node_id, url, body)
    },
};

static HTTP_PUT: NodeDef = NodeDef {
    name: "http_put",
    category: "HTTP",
    description: "HTTP PUT request",
    default_config: || serde_json::json!({"url": "https://example.com"}),
    generate_code: |node_id, config, input_var| {
        let url = config["url"].as_str().unwrap_or("https://example.com");
        let body = input_var.unwrap_or("null");
        format!("    let {}_out = http_request(\"PUT\", \"{}\", {}, {{}});\n", node_id, url, body)
    },
};

static HTTP_DELETE: NodeDef = NodeDef {
    name: "http_delete",
    category: "HTTP",
    description: "HTTP DELETE request",
    default_config: || serde_json::json!({"url": "https://example.com"}),
    generate_code: |node_id, config, _input_var| {
        let url = config["url"].as_str().unwrap_or("https://example.com");
        format!("    let {}_out = http_request(\"DELETE\", \"{}\", null, {{}});\n", node_id, url)
    },
};

static HTTP_REQUEST: NodeDef = NodeDef {
    name: "http_request",
    category: "HTTP",
    description: "Custom HTTP request",
    default_config: || serde_json::json!({"method": "GET", "url": "https://example.com"}),
    generate_code: |node_id, config, input_var| {
        let url = config["url"].as_str().unwrap_or("https://example.com");
        let method = config["method"].as_str().unwrap_or("GET");
        let body = input_var.unwrap_or("null");
        format!("    let {}_out = http_request(\"{}\", \"{}\", {}, {{}});\n", node_id, method, url, body)
    },
};

// ====================
// DATA - JSON NODES
// ====================

static JSON_PARSE: NodeDef = NodeDef {
    name: "json_parse",
    category: "Data",
    description: "Parse JSON string",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("null");
        format!("    let {}_out = json_parse({});\n", node_id, input)
    },
};

static JSON_STRINGIFY: NodeDef = NodeDef {
    name: "json_stringify",
    category: "Data",
    description: "Convert value to JSON string",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("null");
        format!("    let {}_out = json_stringify({});\n", node_id, input)
    },
};

static JSON_GET: NodeDef = NodeDef {
    name: "json_get",
    category: "Data",
    description: "Get value from JSON object",
    default_config: || serde_json::json!({"key": "field"}),
    generate_code: |node_id, config, input_var| {
        let input = input_var.unwrap_or("null");
        let key = config["key"].as_str().unwrap_or("field");
        format!("    let {}_out = get({}, \"{}\");\n", node_id, input, key)
    },
};

static JSON_SET: NodeDef = NodeDef {
    name: "json_set",
    category: "Data",
    description: "Set value in JSON object",
    default_config: || serde_json::json!({"key": "field", "value": ""}),
    generate_code: |node_id, config, input_var| {
        let input = input_var.unwrap_or("{}");
        let key = config["key"].as_str().unwrap_or("field");
        let value = config["value"].as_str().unwrap_or("");
        format!("    let {}_out = set({}, \"{}\", \"{}\");\n", node_id, input, key, value)
    },
};

// ====================
// DATA - STRING NODES
// ====================

static STRING_CONCAT: NodeDef = NodeDef {
    name: "string_concat",
    category: "Data",
    description: "Concatenate strings",
    default_config: || serde_json::json!({"separator": ""}),
    generate_code: |node_id, config, input_var| {
        let input = input_var.unwrap_or("\"\"");
        let sep = config["separator"].as_str().unwrap_or("");
        format!("    let {}_out = concat({}, \"{}\");\n", node_id, input, sep)
    },
};

static STRING_UPPER: NodeDef = NodeDef {
    name: "string_upper",
    category: "Data",
    description: "Convert to uppercase",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("\"\"");
        format!("    let {}_out = to_upper({});\n", node_id, input)
    },
};

static STRING_LOWER: NodeDef = NodeDef {
    name: "string_lower",
    category: "Data",
    description: "Convert to lowercase",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("\"\"");
        format!("    let {}_out = to_lower({});\n", node_id, input)
    },
};

static STRING_TRIM: NodeDef = NodeDef {
    name: "string_trim",
    category: "Data",
    description: "Trim whitespace",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("\"\"");
        format!("    let {}_out = trim({});\n", node_id, input)
    },
};

static STRING_SPLIT: NodeDef = NodeDef {
    name: "string_split",
    category: "Data",
    description: "Split string into array",
    default_config: || serde_json::json!({"delimiter": ","}),
    generate_code: |node_id, config, input_var| {
        let input = input_var.unwrap_or("\"\"");
        let delim = config["delimiter"].as_str().unwrap_or(",");
        // Note: HLX doesn't have built-in split, this would need implementation
        format!("    // TODO: Implement string_split\n    let {}_out = [];\n", node_id)
    },
};

static STRING_REPLACE: NodeDef = NodeDef {
    name: "string_replace",
    category: "Data",
    description: "Replace substring",
    default_config: || serde_json::json!({"find": "", "replace": ""}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("\"\"");
        // Note: HLX doesn't have built-in replace
        format!("    // TODO: Implement string_replace\n    let {}_out = {};\n", node_id, input)
    },
};

static STRING_LENGTH: NodeDef = NodeDef {
    name: "string_length",
    category: "Data",
    description: "Get string length",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("\"\"");
        format!("    let {}_out = strlen({});\n", node_id, input)
    },
};

// ====================
// DATA - ARRAY NODES
// ====================

static ARRAY_MAP: NodeDef = NodeDef {
    name: "array_map",
    category: "Data",
    description: "Map function over array",
    default_config: || serde_json::json!({"function": ""}),
    generate_code: |node_id, _config, _input_var| {
        // TODO: Requires lambda support
        format!("    // TODO: Implement array_map\n    let {}_out = [];\n", node_id)
    },
};

static ARRAY_FILTER: NodeDef = NodeDef {
    name: "array_filter",
    category: "Data",
    description: "Filter array elements",
    default_config: || serde_json::json!({"condition": ""}),
    generate_code: |node_id, _config, _input_var| {
        // TODO: Requires lambda support
        format!("    // TODO: Implement array_filter\n    let {}_out = [];\n", node_id)
    },
};

static ARRAY_REDUCE: NodeDef = NodeDef {
    name: "array_reduce",
    category: "Data",
    description: "Reduce array to single value",
    default_config: || serde_json::json!({"initial": 0}),
    generate_code: |node_id, _config, _input_var| {
        // TODO: Requires lambda support
        format!("    // TODO: Implement array_reduce\n    let {}_out = null;\n", node_id)
    },
};

static ARRAY_SLICE: NodeDef = NodeDef {
    name: "array_slice",
    category: "Data",
    description: "Slice array",
    default_config: || serde_json::json!({"start": 0, "end": 10}),
    generate_code: |node_id, config, input_var| {
        let input = input_var.unwrap_or("[]");
        let start = config["start"].as_i64().unwrap_or(0);
        let end = config["end"].as_i64().unwrap_or(10);
        format!("    let {}_out = arr_slice({}, {}, {});\n", node_id, input, start, end)
    },
};

static ARRAY_CONCAT: NodeDef = NodeDef {
    name: "array_concat",
    category: "Data",
    description: "Concatenate arrays",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("[]");
        format!("    let {}_out = arr_concat({}, []);\n", node_id, input)
    },
};

static ARRAY_SORT: NodeDef = NodeDef {
    name: "array_sort",
    category: "Data",
    description: "Sort array",
    default_config: || serde_json::json!({"order": "asc"}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("[]");
        // TODO: Implement sort
        format!("    // TODO: Implement array_sort\n    let {}_out = {};\n", node_id, input)
    },
};

static ARRAY_LENGTH: NodeDef = NodeDef {
    name: "array_length",
    category: "Data",
    description: "Get array length",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("[]");
        format!("    let {}_out = len({});\n", node_id, input)
    },
};

// ====================
// DATA - OBJECT NODES
// ====================

static OBJECT_GET: NodeDef = NodeDef {
    name: "object_get",
    category: "Data",
    description: "Get object property",
    default_config: || serde_json::json!({"key": "field"}),
    generate_code: |node_id, config, input_var| {
        let input = input_var.unwrap_or("{}");
        let key = config["key"].as_str().unwrap_or("field");
        format!("    let {}_out = get({}, \"{}\");\n", node_id, input, key)
    },
};

static OBJECT_SET: NodeDef = NodeDef {
    name: "object_set",
    category: "Data",
    description: "Set object property",
    default_config: || serde_json::json!({"key": "field", "value": ""}),
    generate_code: |node_id, config, input_var| {
        let input = input_var.unwrap_or("{}");
        let key = config["key"].as_str().unwrap_or("field");
        let value = config["value"].as_str().unwrap_or("");
        format!("    let {}_out = set({}, \"{}\", \"{}\");\n", node_id, input, key, value)
    },
};

static OBJECT_KEYS: NodeDef = NodeDef {
    name: "object_keys",
    category: "Data",
    description: "Get object keys",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("{}");
        format!("    let {}_out = keys({});\n", node_id, input)
    },
};

static OBJECT_VALUES: NodeDef = NodeDef {
    name: "object_values",
    category: "Data",
    description: "Get object values",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("{}");
        format!("    let {}_out = values({});\n", node_id, input)
    },
};

static OBJECT_HAS_KEY: NodeDef = NodeDef {
    name: "object_has_key",
    category: "Data",
    description: "Check if object has key",
    default_config: || serde_json::json!({"key": "field"}),
    generate_code: |node_id, config, input_var| {
        let input = input_var.unwrap_or("{}");
        let key = config["key"].as_str().unwrap_or("field");
        format!("    let {}_out = has_key({}, \"{}\");\n", node_id, input, key)
    },
};

// ====================
// FILE NODES
// ====================

static FILE_READ: NodeDef = NodeDef {
    name: "file_read",
    category: "Files",
    description: "Read file contents",
    default_config: || serde_json::json!({"path": "file.txt"}),
    generate_code: |node_id, config, _input_var| {
        let path = config["path"].as_str().unwrap_or("file.txt");
        format!("    let {}_out = read_file(\"{}\");\n", node_id, path)
    },
};

static FILE_WRITE: NodeDef = NodeDef {
    name: "file_write",
    category: "Files",
    description: "Write file contents",
    default_config: || serde_json::json!({"path": "file.txt"}),
    generate_code: |node_id, config, input_var| {
        let path = config["path"].as_str().unwrap_or("file.txt");
        let content = input_var.unwrap_or("\"\"");
        format!("    let {}_out = write_file(\"{}\", {});\n", node_id, path, content)
    },
};

static FILE_EXISTS: NodeDef = NodeDef {
    name: "file_exists",
    category: "Files",
    description: "Check if file exists",
    default_config: || serde_json::json!({"path": "file.txt"}),
    generate_code: |node_id, config, _input_var| {
        let path = config["path"].as_str().unwrap_or("file.txt");
        format!("    let {}_out = file_exists(\"{}\");\n", node_id, path)
    },
};

static FILE_DELETE: NodeDef = NodeDef {
    name: "file_delete",
    category: "Files",
    description: "Delete file",
    default_config: || serde_json::json!({"path": "file.txt"}),
    generate_code: |node_id, config, _input_var| {
        let path = config["path"].as_str().unwrap_or("file.txt");
        format!("    let {}_out = delete_file(\"{}\");\n", node_id, path)
    },
};

static FILE_LIST: NodeDef = NodeDef {
    name: "file_list",
    category: "Files",
    description: "List files in directory",
    default_config: || serde_json::json!({"path": "."}),
    generate_code: |node_id, config, _input_var| {
        let path = config["path"].as_str().unwrap_or(".");
        format!("    let {}_out = list_files(\"{}\");\n", node_id, path)
    },
};

static DIR_CREATE: NodeDef = NodeDef {
    name: "dir_create",
    category: "Files",
    description: "Create directory",
    default_config: || serde_json::json!({"path": "new_dir"}),
    generate_code: |node_id, config, _input_var| {
        let path = config["path"].as_str().unwrap_or("new_dir");
        format!("    let {}_out = create_dir(\"{}\");\n", node_id, path)
    },
};

static JSON_READ: NodeDef = NodeDef {
    name: "json_read",
    category: "Files",
    description: "Read JSON file",
    default_config: || serde_json::json!({"path": "data.json"}),
    generate_code: |node_id, config, _input_var| {
        let path = config["path"].as_str().unwrap_or("data.json");
        format!("    let {}_out = read_json(\"{}\");\n", node_id, path)
    },
};

static JSON_WRITE: NodeDef = NodeDef {
    name: "json_write",
    category: "Files",
    description: "Write JSON file",
    default_config: || serde_json::json!({"path": "data.json"}),
    generate_code: |node_id, config, input_var| {
        let path = config["path"].as_str().unwrap_or("data.json");
        let content = input_var.unwrap_or("null");
        format!("    let {}_out = write_json(\"{}\", {});\n", node_id, path, content)
    },
};

// ====================
// MATH NODES
// ====================

static MATH_ADD: NodeDef = NodeDef {
    name: "math_add",
    category: "Math",
    description: "Add two numbers",
    default_config: || serde_json::json!({"value": 0}),
    generate_code: |node_id, config, input_var| {
        let input = input_var.unwrap_or("0");
        let value = config["value"].as_i64().unwrap_or(0);
        format!("    let {}_out = {} + {};\n", node_id, input, value)
    },
};

static MATH_SUBTRACT: NodeDef = NodeDef {
    name: "math_subtract",
    category: "Math",
    description: "Subtract two numbers",
    default_config: || serde_json::json!({"value": 0}),
    generate_code: |node_id, config, input_var| {
        let input = input_var.unwrap_or("0");
        let value = config["value"].as_i64().unwrap_or(0);
        format!("    let {}_out = {} - {};\n", node_id, input, value)
    },
};

static MATH_MULTIPLY: NodeDef = NodeDef {
    name: "math_multiply",
    category: "Math",
    description: "Multiply two numbers",
    default_config: || serde_json::json!({"value": 1}),
    generate_code: |node_id, config, input_var| {
        let input = input_var.unwrap_or("1");
        let value = config["value"].as_i64().unwrap_or(1);
        format!("    let {}_out = {} * {};\n", node_id, input, value)
    },
};

static MATH_DIVIDE: NodeDef = NodeDef {
    name: "math_divide",
    category: "Math",
    description: "Divide two numbers",
    default_config: || serde_json::json!({"value": 1}),
    generate_code: |node_id, config, input_var| {
        let input = input_var.unwrap_or("1");
        let value = config["value"].as_i64().unwrap_or(1);
        format!("    let {}_out = {} / {};\n", node_id, input, value)
    },
};

static MATH_FLOOR: NodeDef = NodeDef {
    name: "math_floor",
    category: "Math",
    description: "Floor of number",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("0");
        format!("    let {}_out = floor({});\n", node_id, input)
    },
};

static MATH_CEIL: NodeDef = NodeDef {
    name: "math_ceil",
    category: "Math",
    description: "Ceiling of number",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("0");
        format!("    let {}_out = ceil({});\n", node_id, input)
    },
};

static MATH_ROUND: NodeDef = NodeDef {
    name: "math_round",
    category: "Math",
    description: "Round number",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("0");
        format!("    let {}_out = round({});\n", node_id, input)
    },
};

static MATH_SQRT: NodeDef = NodeDef {
    name: "math_sqrt",
    category: "Math",
    description: "Square root",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("0");
        format!("    let {}_out = sqrt({});\n", node_id, input)
    },
};

static MATH_RANDOM: NodeDef = NodeDef {
    name: "math_random",
    category: "Math",
    description: "Random number (0-1)",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, _input_var| {
        format!("    let {}_out = random();\n", node_id)
    },
};

// ====================
// TYPE CONVERSION NODES
// ====================

static TO_STRING: NodeDef = NodeDef {
    name: "to_string",
    category: "Convert",
    description: "Convert to string",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("null");
        format!("    let {}_out = to_string({});\n", node_id, input)
    },
};

static TO_INT: NodeDef = NodeDef {
    name: "to_int",
    category: "Convert",
    description: "Convert to integer",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("0");
        format!("    let {}_out = to_int({});\n", node_id, input)
    },
};

static TO_FLOAT: NodeDef = NodeDef {
    name: "to_float",
    category: "Convert",
    description: "Convert to float",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("0");
        format!("    let {}_out = to_float({});\n", node_id, input)
    },
};

// ====================
// ML/GPU NODES
// ====================

static TENSOR_CREATE: NodeDef = NodeDef {
    name: "tensor_create",
    category: "ML/GPU",
    description: "Create 2D tensor",
    default_config: || serde_json::json!({"rows": 2, "cols": 2, "values": [1.0, 0.0, 0.0, 1.0]}),
    generate_code: |node_id, config, _input_var| {
        let rows = config["rows"].as_u64().unwrap_or(2);
        let cols = config["cols"].as_u64().unwrap_or(2);
        let vals = config["values"].as_array();

        let mut code = format!("    let {}_t = tensor_new_2d({}, {});\n", node_id, rows, cols);

        if let Some(values) = vals {
            for (i, v) in values.iter().enumerate() {
                let val = v.as_f64().unwrap_or(0.0);
                code.push_str(&format!("    let {}_data = {}_t[2];\n", node_id, node_id));
                code.push_str(&format!("    {}_data[{}] = {};\n", node_id, i, val));
            }
        }
        code.push_str(&format!("    let {}_out = {}_t;\n", node_id, node_id));
        code
    },
};

static TENSOR_MATMUL: NodeDef = NodeDef {
    name: "tensor_matmul",
    category: "ML/GPU",
    description: "Matrix multiplication",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, _input_var| {
        // Needs two inputs from edges
        format!("    // TODO: Get two tensor inputs from edges\n    let {}_out = null;\n", node_id)
    },
};

static TENSOR_ADD: NodeDef = NodeDef {
    name: "tensor_add",
    category: "ML/GPU",
    description: "Element-wise tensor addition",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, _input_var| {
        // Needs two inputs from edges
        format!("    // TODO: Get two tensor inputs from edges\n    let {}_out = null;\n", node_id)
    },
};

// ====================
// SYSTEM NODES
// ====================

static SLEEP: NodeDef = NodeDef {
    name: "sleep",
    category: "System",
    description: "Sleep for milliseconds",
    default_config: || serde_json::json!({"ms": 1000}),
    generate_code: |node_id, config, input_var| {
        let ms = config["ms"].as_i64().unwrap_or(1000);
        let input = input_var.unwrap_or("null");
        format!("    sleep({});\n    let {}_out = {};\n", ms, node_id, input)
    },
};

static CAPTURE_SCREEN: NodeDef = NodeDef {
    name: "capture_screen",
    category: "System",
    description: "Capture screenshot",
    default_config: || serde_json::json!({}),
    generate_code: |node_id, _config, _input_var| {
        format!("    let {}_out = capture_screen();\n", node_id)
    },
};
