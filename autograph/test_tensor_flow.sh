#!/bin/bash

# Define the tensor flow JSON
# A = [[1, 0], [0, 1]] (Identity)
# B = [[2, 2], [2, 2]]
# Result = A @ B = [[2, 2], [2, 2]]
FLOW_JSON='{
  "nodes": [
    {
      "id": "matA",
      "type_name": "tensor_create",
      "config": { "rows": 2, "cols": 2, "values": [1.0, 0.0, 0.0, 1.0] },
      "position": null
    },
    {
      "id": "matB",
      "type_name": "tensor_create",
      "config": { "rows": 2, "cols": 2, "values": [2.0, 2.0, 2.0, 2.0] },
      "position": null
    },
    {
      "id": "matmul",
      "type_name": "tensor_op",
      "config": { "op": "dot" },
      "position": null
    },
    {
      "id": "printer",
      "type_name": "print",
      "config": {},
      "position": null
    }
  ],
  "edges": [
    { "source": "matA", "target": "matmul", "source_handle": null, "target_handle": null },
    { "source": "matB", "target": "matmul", "source_handle": null, "target_handle": null },
    { "source": "matmul", "target": "printer", "source_handle": null, "target_handle": null }
  ]
}'

echo "Deploying tensor flow..."
curl -X POST http://localhost:3000/deploy/tensor_test \
  -H "Content-Type: application/json" \
  -d "$FLOW_JSON"

echo -e "\n\nRunning tensor flow..."
curl -X POST http://localhost:3000/run/tensor_test \
  -H "Content-Type: application/json" \
  -d 'null'
