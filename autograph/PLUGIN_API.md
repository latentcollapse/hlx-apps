# Autograph Plugin API

**Extend Autograph with custom nodes in minutes!**

Autograph's plugin system makes it easy to add new node types without modifying the core codebase. This guide shows you how to create, register, and test custom nodes.

---

## Table of Contents

- [Quick Start](#quick-start)
- [Node Definition Structure](#node-definition-structure)
- [Code Generation](#code-generation)
- [Configuration Schema](#configuration-schema)
- [Examples](#examples)
- [Best Practices](#best-practices)
- [Testing Your Node](#testing-your-node)
- [Advanced Topics](#advanced-topics)

---

## Quick Start

**5-Minute Example: Add a Custom Node**

```rust
// 1. Add to src/nodes.rs

use serde_json::json;

static MY_CUSTOM_NODE: NodeDef = NodeDef {
    name: "my_awesome_node",
    category: "Custom",
    description: "Does something amazing",
    default_config: || json!({
        "param1": "value",
        "param2": 42
    }),
    generate_code: |node_id, config, input_var| {
        let param1 = config["param1"].as_str().unwrap_or("default");
        let param2 = config["param2"].as_i64().unwrap_or(0);
        let input = input_var.unwrap_or("null");

        format!(
            "    let {}_out = custom_function(\"{}\", {}, {});\n",
            node_id, param1, param2, input
        )
    },
};

// 2. Register in all_nodes()
pub fn all_nodes() -> Vec<&'static NodeDef> {
    vec![
        // ... existing nodes ...
        &MY_CUSTOM_NODE,
    ]
}
```

**That's it!** Your node now appears in the palette and can be used in workflows.

---

## Node Definition Structure

### `NodeDef` Struct

```rust
pub struct NodeDef {
    /// Unique identifier (snake_case recommended)
    pub name: &'static str,

    /// Category in the palette (e.g., "HTTP", "Data", "Custom")
    pub category: &'static str,

    /// Short description shown in tooltips
    pub description: &'static str,

    /// Returns default JSON configuration
    pub default_config: fn() -> serde_json::Value,

    /// Generates HLX code for this node
    /// Parameters: (node_id, config, input_var)
    /// Returns: HLX code as String
    pub generate_code: fn(&str, &serde_json::Value, Option<&str>) -> String,
}
```

### Field Descriptions

#### `name`
- **Type**: `&'static str`
- **Purpose**: Unique identifier for the node type
- **Conventions**:
  - Use `snake_case`
  - Be descriptive: `json_parse`, `http_get`, `tensor_multiply`
  - Avoid generic names: ❌ `process`, `handle`, `do_thing`

#### `category`
- **Type**: `&'static str`
- **Purpose**: Groups nodes in the palette
- **Standard Categories**:
  - `"Control"` - Start, print, conditional
  - `"HTTP"` - API requests
  - `"Data"` - JSON, String, Array, Object ops
  - `"Files"` - File I/O operations
  - `"Math"` - Mathematical operations
  - `"ML/GPU"` - Tensor operations
  - `"System"` - OS-level operations
  - `"Convert"` - Type conversions
  - `"Custom"` - Your plugins

#### `description`
- **Type**: `&'static str`
- **Purpose**: Shown in tooltip when hovering over node
- **Best Practices**:
  - Keep it short (< 60 chars)
  - Action-oriented: "Parse JSON string" not "Parses JSON"
  - Avoid implementation details

#### `default_config`
- **Type**: `fn() -> serde_json::Value`
- **Purpose**: Returns default JSON configuration for new nodes
- **Examples**:
  ```rust
  // Simple config
  default_config: || json!({"url": "https://example.com"})

  // Complex config
  default_config: || json!({
      "method": "GET",
      "headers": {},
      "timeout_ms": 5000
  })

  // Empty config
  default_config: || json!({})
  ```

#### `generate_code`
- **Type**: `fn(&str, &serde_json::Value, Option<&str>) -> String`
- **Purpose**: Generates HLX source code for this node
- **Parameters**:
  - `node_id: &str` - Unique ID of this node instance (e.g., `"node_123"`)
  - `config: &serde_json::Value` - User's configuration JSON
  - `input_var: Option<&str>` - Variable name of input from previous node
- **Returns**: HLX code as a String
- **Rules**:
  - Must define a variable named `{node_id}_out` (output for next node)
  - Use `input_var.unwrap_or("null")` to handle missing inputs
  - Extract config values safely with `as_str()`, `as_i64()`, etc.

---

## Code Generation

### The `generate_code` Function

This is where the magic happens. Your function converts visual node config into executable HLX code.

### Pattern 1: No Input (Source Nodes)

```rust
generate_code: |node_id, config, _input_var| {
    let url = config["url"].as_str().unwrap_or("https://example.com");

    format!(
        "    let {}_out = http_get(\"{}\");\n",
        node_id, url
    )
}
```

**Output Example**:
```hlx
    let node_1_out = http_get("https://api.github.com");
```

### Pattern 2: Single Input (Transform Nodes)

```rust
generate_code: |node_id, config, input_var| {
    let input = input_var.unwrap_or("null");
    let key = config["key"].as_str().unwrap_or("data");

    format!(
        "    let {}_out = json_get({}, \"{}\");\n",
        node_id, input, key
    )
}
```

**Output Example**:
```hlx
    let node_2_out = json_get(node_1_out, "data");
```

### Pattern 3: Multiple Config Parameters

```rust
generate_code: |node_id, config, input_var| {
    let input = input_var.unwrap_or("null");
    let method = config["method"].as_str().unwrap_or("GET");
    let timeout = config["timeout_ms"].as_i64().unwrap_or(5000);
    let retry = config["retry"].as_bool().unwrap_or(false);

    format!(
        "    let {}_out = http_request(\"{}\", {}, {}, {});\n",
        node_id, method, input, timeout, retry
    )
}
```

### Pattern 4: Conditional Logic

```rust
generate_code: |node_id, config, input_var| {
    let input = input_var.unwrap_or("null");
    let mode = config["mode"].as_str().unwrap_or("sync");

    let code = match mode {
        "async" => format!("async_process({})", input),
        "batch" => format!("batch_process({})", input),
        _ => format!("sync_process({})", input),
    };

    format!("    let {}_out = {};\n", node_id, code)
}
```

---

## Configuration Schema

### JSON Schema Best Practices

**Good Config Design**:
```json
{
  "url": "https://api.example.com",
  "method": "GET",
  "timeout_ms": 5000,
  "retry_count": 3
}
```

**Bad Config Design**:
```json
{
  "options": "url=https://api.example.com;method=GET",
  "flags": 0x03
}
```

### Type Safety

Always use safe unwrapping:

```rust
// ✅ GOOD: Safe with defaults
let url = config["url"].as_str().unwrap_or("https://example.com");
let count = config["count"].as_i64().unwrap_or(10);
let enabled = config["enabled"].as_bool().unwrap_or(false);

// ❌ BAD: Can panic
let url = config["url"].as_str().unwrap();
```

### Validation

Add validation in `generate_code` if needed:

```rust
generate_code: |node_id, config, input_var| {
    let timeout = config["timeout_ms"].as_i64().unwrap_or(5000);

    // Validate range
    let timeout = timeout.clamp(0, 60000);

    // ... generate code ...
}
```

---

## Examples

### Example 1: Simple Transform Node

**Goal**: Convert string to uppercase

```rust
static STRING_UPPER: NodeDef = NodeDef {
    name: "string_upper",
    category: "Data",
    description: "Convert string to uppercase",
    default_config: || json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("\"\"");
        format!("    let {}_out = string_upper({});\n", node_id, input)
    },
};
```

### Example 2: HTTP Request Node

**Goal**: Make HTTP GET request

```rust
static HTTP_GET: NodeDef = NodeDef {
    name: "http_get",
    category: "HTTP",
    description: "HTTP GET request",
    default_config: || json!({"url": "https://example.com"}),
    generate_code: |node_id, config, _input_var| {
        let url = config["url"].as_str().unwrap_or("https://example.com");
        format!("    let {}_out = http_request(\"GET\", \"{}\", null, {{}});\n", node_id, url)
    },
};
```

### Example 3: Math Operation Node

**Goal**: Add a constant to input

```rust
static MATH_ADD: NodeDef = NodeDef {
    name: "math_add",
    category: "Math",
    description: "Add constant to value",
    default_config: || json!({"value": 10}),
    generate_code: |node_id, config, input_var| {
        let input = input_var.unwrap_or("0");
        let value = config["value"].as_i64().unwrap_or(10);
        format!("    let {}_out = {} + {};\n", node_id, input, value)
    },
};
```

### Example 4: File I/O Node

**Goal**: Read file contents

```rust
static FILE_READ: NodeDef = NodeDef {
    name: "file_read",
    category: "Files",
    description: "Read file contents",
    default_config: || json!({"path": "input.txt"}),
    generate_code: |node_id, config, _input_var| {
        let path = config["path"].as_str().unwrap_or("input.txt");
        format!("    let {}_out = file_read(\"{}\");\n", node_id, path)
    },
};
```

### Example 5: Conditional Node

**Goal**: Branch based on condition

```rust
static CONDITIONAL: NodeDef = NodeDef {
    name: "if_condition",
    category: "Control",
    description: "Execute if condition is true",
    default_config: || json!({
        "operator": "equals",
        "compare_value": "true"
    }),
    generate_code: |node_id, config, input_var| {
        let input = input_var.unwrap_or("false");
        let operator = config["operator"].as_str().unwrap_or("equals");
        let compare = config["compare_value"].as_str().unwrap_or("true");

        format!(
            "    let {}_out = if {} {} \"{}\" {{ \"true\" }} else {{ \"false\" }};\n",
            node_id, input, operator, compare
        )
    },
};
```

---

## Best Practices

### ✅ DO

1. **Use Descriptive Names**
   ```rust
   name: "json_parse"  // ✅ Clear
   name: "jp"          // ❌ Cryptic
   ```

2. **Provide Sensible Defaults**
   ```rust
   default_config: || json!({
       "timeout_ms": 5000,
       "retry_count": 3
   })
   ```

3. **Handle Missing Inputs**
   ```rust
   let input = input_var.unwrap_or("null");
   ```

4. **Validate Configuration**
   ```rust
   let timeout = config["timeout"].as_i64().unwrap_or(5000).clamp(0, 60000);
   ```

5. **Use Consistent Formatting**
   ```rust
   format!("    let {}_out = function({});\n", node_id, input)
   //      ^^^^                              ^^
   //      4 spaces                          newline
   ```

### ❌ DON'T

1. **Don't Panic**
   ```rust
   // ❌ BAD
   let url = config["url"].as_str().unwrap();

   // ✅ GOOD
   let url = config["url"].as_str().unwrap_or("default");
   ```

2. **Don't Mutate Global State**
   ```rust
   // ❌ BAD: Side effects in generate_code
   generate_code: |node_id, config, input_var| {
       GLOBAL_COUNTER += 1;  // Non-deterministic!
       // ...
   }
   ```

3. **Don't Forget Output Variable**
   ```rust
   // ❌ BAD: No output defined
   format!("    do_something({});\n", input)

   // ✅ GOOD: Output for next node
   format!("    let {}_out = do_something({});\n", node_id, input)
   ```

4. **Don't Use Complex Logic**
   Keep `generate_code` simple. For complex operations, define helper functions:
   ```rust
   fn build_http_headers(config: &JsonValue) -> String {
       // Complex logic here
   }

   generate_code: |node_id, config, input_var| {
       let headers = build_http_headers(config);
       format!("    let {}_out = http_request({});\n", node_id, headers)
   }
   ```

---

## Testing Your Node

### 1. Build Autograph

```bash
cargo build --release
```

### 2. Launch UI

```bash
./target/release/autograph
```

### 3. Create Test Workflow

1. Add your node from the palette
2. Configure it in the properties panel
3. Connect it to other nodes
4. Click "Compile" to see generated HLX code
5. Click "Run" to execute

### 4. Debug Generated Code

Click "Compile" and check `flows/{your_flow}.hlxa`:

```hlx
fn main() {
    let node_1_out = your_custom_function("param");
    let node_2_out = print(node_1_out);
}
```

Verify:
- ✅ Output variable is defined (`{node_id}_out`)
- ✅ Input variable is used correctly
- ✅ Config values are properly escaped
- ✅ Syntax is valid HLX

### 5. Test Edge Cases

- Empty config: `json!({})`
- Missing input: `input_var = None`
- Invalid config values: `config["missing_key"]`

---

## Advanced Topics

### Multi-Input Nodes

Some nodes need multiple inputs. Use special handling:

```rust
generate_code: |node_id, config, input_var| {
    // For multi-input, use config to specify input sources
    let input_a = config["input_a"].as_str().unwrap_or("null");
    let input_b = config["input_b"].as_str().unwrap_or("null");

    format!(
        "    let {}_out = combine({}, {});\n",
        node_id, input_a, input_b
    )
}
```

### GPU-Accelerated Nodes

For tensor operations, generate code that uses HLX's Vulkan backend:

```rust
static TENSOR_MATMUL: NodeDef = NodeDef {
    name: "tensor_matmul",
    category: "ML/GPU",
    description: "Matrix multiplication (GPU accelerated)",
    default_config: || json!({}),
    generate_code: |node_id, _config, input_var| {
        let input = input_var.unwrap_or("null");
        format!("    let {}_out = tensor_matmul({});\n", node_id, input)
    },
};
```

The HLX runtime automatically uses the selected backend (CPU/Vulkan).

### Async Operations

For long-running operations, generate async code:

```rust
generate_code: |node_id, config, input_var| {
    let url = config["url"].as_str().unwrap_or("https://example.com");
    format!(
        "    let {}_out = await http_get_async(\"{}\");\n",
        node_id, url
    )
}
```

### Error Handling

Generate code with explicit error handling:

```rust
generate_code: |node_id, config, input_var| {
    let path = config["path"].as_str().unwrap_or("input.txt");
    format!(
        "    let {}_out = match file_read(\"{}\") {{\n\
             Ok(content) => content,\n\
             Err(e) => error(\"File read failed: {{}}\", e),\n\
         }};\n",
        node_id, path
    )
}
```

---

## Plugin Distribution (Future)

Autograph v1.0 includes the plugin infrastructure. Future versions will support:

- **Hot-reload**: Update nodes without restarting
- **Remote plugins**: Load nodes from URLs
- **Plugin marketplace**: Share and discover community nodes
- **Versioning**: Manage plugin compatibility

For now, all plugins must be compiled into Autograph by modifying `src/nodes.rs`.

---

## Example: Complete Custom Node

Here's a complete example showing all best practices:

```rust
use serde_json::json;

/// Custom node: Retry HTTP request with exponential backoff
static HTTP_RETRY: NodeDef = NodeDef {
    name: "http_retry",
    category: "HTTP",
    description: "HTTP request with retry and exponential backoff",

    default_config: || json!({
        "url": "https://api.example.com",
        "method": "GET",
        "max_retries": 3,
        "initial_delay_ms": 1000,
        "backoff_multiplier": 2.0
    }),

    generate_code: |node_id, config, input_var| {
        // Extract config with safe defaults
        let url = config["url"].as_str().unwrap_or("https://example.com");
        let method = config["method"].as_str().unwrap_or("GET");
        let max_retries = config["max_retries"].as_i64().unwrap_or(3).clamp(1, 10);
        let initial_delay = config["initial_delay_ms"].as_i64().unwrap_or(1000).clamp(100, 60000);
        let backoff = config["backoff_multiplier"].as_f64().unwrap_or(2.0).clamp(1.0, 10.0);

        let input_body = input_var.unwrap_or("null");

        // Generate HLX code
        format!(
            "    let {}_out = http_request_with_retry(\
                \"{}\", \"{}\", {}, {}, {}, {});\n",
            node_id, method, url, input_body, max_retries, initial_delay, backoff
        )
    },
};

// Register it
pub fn all_nodes() -> Vec<&'static NodeDef> {
    vec![
        // ... existing nodes ...
        &HTTP_RETRY,
    ]
}
```

---

## Questions?

**Issues**: https://github.com/latentcollapse/hlx-apps/issues
**Discussions**: https://github.com/latentcollapse/hlx-apps/discussions

---

**Happy Building!** 🚀
