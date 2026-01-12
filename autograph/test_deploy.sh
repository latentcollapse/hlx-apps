#!/bin/bash

# Start server in background
cargo run --bin autograph &
SERVER_PID=$!

echo "Waiting for server to start..."
sleep 5

# Define the flow JSON
FLOW_JSON='{
  "nodes": [
    {
      "id": "start",
      "type_name": "start",
      "config": {},
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
    {
      "source": "start",
      "target": "printer",
      "source_handle": null,
      "target_handle": null
    }
  ]
}'

echo "Deploying flow..."
curl -X POST http://localhost:3000/deploy/test_flow \
  -H "Content-Type: application/json" \
  -d "$FLOW_JSON"

echo -e "\n\nRunning flow..."
curl -X POST http://localhost:3000/run/test_flow \
  -H "Content-Type: application/json" \
  -d '"Hello from Autograph!"'

# Cleanup
kill $SERVER_PID
