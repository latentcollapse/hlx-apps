# HLX Applications

This repository contains applications built on top of the HLX compiler. These demonstrate HLX's capabilities for deterministic systems programming, GPU compute, and verifiable execution.

## Applications

### 1. Autograph
**Location:** `autograph/`
**Type:** Web service (Rust + Axum)
**Purpose:** HTTP server for running HLX flows on-demand

Autograph serves HLX programs via REST API, allowing external systems to trigger deterministic computations.

**Build:**
```bash
cd autograph
cargo build --release
```

**Run:**
```bash
cargo run --release
# Server starts on http://localhost:3000
```

**Dependencies:**
- hlx_core, hlx_compiler, hlx_runtime (from `../hlx-compiler`)
- Axum, Tokio for async web serving

---

### 2. Runegraph
**Location:** `flows/runegraph.hlxa`
**Type:** HLX script
**Purpose:** Deterministic execution visualizer (flamegraph replacement)

Runegraph captures the exact execution trace of an HLX program and renders it as an SVG. Because HLX is deterministic, the visualization is identical across all runs.

**Features:**
- Records every instruction (PC + Op)
- Color-coded by operation type (math, control, memory, etc.)
- No statistical sampling - exact execution path
- Future: diff mode for comparing runs

**Usage:**
```bash
# Via Autograph server
curl -X POST http://localhost:3000/run/runegraph

# Direct compilation
hlx compile flows/runegraph.hlxa -o runegraph.lcc
hlx run runegraph.lcc
```

**See also:** `docs/RUNEGRAPH.md`

---

### 3. Flight Recorder
**Location:** `flows/recorder.hlxa`
**Type:** HLX script
**Purpose:** Screen capture and video recording

Captures screen frames and pipes them to ffmpeg for encoding. Demonstrates HLX's ability to interface with system tools via pipes.

**Usage:**
```bash
# Via Autograph
curl -X POST http://localhost:3000/run/recorder

# Outputs: output.mp4
```

**Requirements:**
- ffmpeg installed
- Display server (X11/Wayland)

---

### 4. The Construct
**Location:** `the_construct/`
**Type:** Native binary (HLX → LLVM → ELF)
**Purpose:** Procedural 3D reality engine

A Vulkan + SDL2 application demonstrating HLX's GPU compute and graphics capabilities. Generates procedural content deterministically on the GPU.

**Architecture:**
- HLX source → LC-B bytecode → LLVM IR → native object file
- Linked with SDL2 for windowing/input
- Vulkan compute shaders for procedural generation

**Build:**
```bash
cd the_construct

# Compile HLX to object file
../../hlx-compiler/hlx/target/release/hlx compile-native src/main.hlxa -o construct.o --target x86_64-unknown-linux-gnu

# Link with SDL2 and math library
gcc -no-pie construct.o -o construct -lSDL2 -lm

# Run
./construct
```

**Requirements:**
- gcc
- libsdl2-dev
- GUI environment (X11/Wayland)

**Status:**
- ✅ Phase 1: SDL2 window + main loop
- 🚧 Phase 2: Vulkan context
- 🔮 Phase 3: Procedural mesh generation
- 🔮 Phase 4: Physics & animation

---

### 5. Other Flows
**Location:** `flows/`

- **echo.hlxa** - Simple echo server demonstration
- **kobayashi.hlxa** - Named flow (purpose TBD)
- **weather.hlxa** - Weather data fetching example

---

## Project Structure

```
hlx-apps/
├── autograph/           # Web service for running HLX flows
│   ├── Cargo.toml       # Points to ../../hlx-compiler/hlx/*
│   └── src/main.rs
├── flows/               # HLX scripts served by Autograph
│   ├── runegraph.hlxa   # Execution visualizer
│   ├── recorder.hlxa    # Screen recorder
│   ├── weather.hlxa
│   └── ...
├── the_construct/       # 3D engine (SDL2 + Vulkan)
│   ├── src/main.hlxa
│   ├── construct        # Compiled binary
│   └── README.md
└── docs/                # Documentation
    └── RUNEGRAPH.md
```

---

## Dependency Model

All apps depend on the HLX compiler libraries:

```
hlx-apps/
└── autograph/Cargo.toml
    └── depends on: ../../hlx-compiler/hlx/{hlx_core, hlx_compiler, hlx_runtime}
```

This keeps the compiler separate from the applications it enables.

**To rebuild compiler and apps:**
```bash
# 1. Build compiler
cd hlx-compiler/hlx
cargo build --release

# 2. Build Autograph (picks up latest compiler libs)
cd ../../hlx-apps/autograph
cargo build --release

# 3. Recompile The Construct if needed
cd ../the_construct
../../hlx-compiler/hlx/target/release/hlx compile-native src/main.hlxa -o construct.o --target x86_64-unknown-linux-gnu
gcc -no-pie construct.o -o construct -lSDL2 -lm
```

---

## Why These Apps Matter

### Runegraph
Demonstrates **A1 (Determinism)**: Same input → same execution trace → identical visualization, always.

### Flight Recorder
Demonstrates **A2 (Reversibility)** + **A4 (Universal Value)**: Capture runtime state, serialize, decode for debugging.

### The Construct
Demonstrates **cross-platform determinism**: GPU compute produces identical results on NVIDIA, AMD, Intel.

### Autograph
Demonstrates **content-addressable code**: Hash the bytecode, serve it, verify integrity.

---

## Contributing

To add a new HLX application:

1. **HLX scripts:** Add to `flows/` directory
2. **Native apps:** Create subdirectory with README
3. **Web services:** Extend Autograph or create new Rust project

All apps should demonstrate one or more of HLX's four axioms (A1-A4).

---

## License

Apache 2.0 (same as HLX compiler)
