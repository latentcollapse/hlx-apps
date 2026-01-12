# HLX Dev Studio - Vision & Roadmap

## Core Philosophy
A native, self-hosted IDE built in HLX, for HLX. It serves as the primary interface for developing applications and the Helinux operating system.

## Key Features

### 1. AI Command Center (CLI Terminal)
- **Interface:** A built-in terminal window.
- **Capabilities:** 
  - Speak to models directly.
  - **Local:** Import and run local models (via GGUF/llama.cpp bindings).
  - **Cloud:** API key support for external models.
  - Context-aware assistance for coding.

### 2. Editor Workspace
- **Layout:** Horizontal tabs along the top (VS Code style).
- **Features:** Syntax highlighting (HLX), auto-formatting, project file tree.

### 3. Neural Forge (Training Center)
- **Goal:** Fine-tune models on HLX code.
- **Features:**
  - LoRA/QLoRA training configuration.
  - Set training cycles, learning rates, etc.
  - Real-time loss convergence graphs.
  - Benchmark runner.

### 4. Helinux Development (The "Axiom" Tab)
- **Goal:** Ring-0 OS development assisted by AI.
- **Concept:** An "IDE within an IDE".
- **Workflow:** 
  - Import kernel source files.
  - Agent-assisted sorting and architecture.
  - Cross-compilation to bootable ISOs/binaries.
  - **Target:** True bare-metal Helinux (not just a Linux skin).

### 5. Collaboration (The "Link" System)
- **Style:** "FromSoft" summoning/linking.
- **Mechanism:** 
  - P2P or Relay-based synchronization.
  - Server Hash/Session ID for connecting teams.
  - Auto-sync/Auto-save across all connected clients.
  - "Mini-Github" functionality without the web bloat.

## Implementation Strategy
- **Phase 1:** Core UI & File I/O (Current).
- **Phase 2:** Text Rendering & Editor Logic.
- **Phase 3:** CLI/Terminal Emulation.
- **Phase 4:** FFI Bindings for AI/Training (CUDA/Metal).
- **Phase 5:** Networking (P2P Sync).
