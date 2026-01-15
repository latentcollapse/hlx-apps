# Autograph: The n8n Killer

**Status:** MVP → Production-Ready Workflow Automation
**Owner:** Claude (Pet Project)
**Vision:** Attack n8n's pain points with HLX-powered determinism, native performance, and incredible UX

---

## What Is Autograph?

A **visual workflow automation tool** powered by HLX. Users drag-and-drop nodes to create workflows, Autograph compiles them to HLX, executes with determinism guarantees, and provides real-time debugging.

**Current State:**
- ✅ Flow model (nodes + edges)
- ✅ HLX code generation from flows
- ✅ REST API for deploying/running flows
- ✅ Node types: http_request, json_parse, tensor_op, tensor_create, print
- ⚠️ Web-based server (needs native UI)

**Target State:**
- 🎯 Native desktop app (Tauri/egui/iced)
- 🎯 Visual flow editor with drag-and-drop
- 🎯 100+ node types (HTTP, DB, Files, ML, etc.)
- 🎯 Real-time execution visualization
- 🎯 Deterministic replay & debugging
- 🎯 Version control friendly (flows = .hlxa files)
- 🎯 GPU-accelerated for data pipelines
- 🎯 "Piss easy" for humans to use

---

## n8n's Pain Points → Autograph Solutions

| n8n Problem | Autograph Solution |
|-------------|-------------------|
| **Slow (Node.js)** | Native code via LLVM, GPU via Vulkan |
| **Non-deterministic** | HLX A1 axiom - same input always gives same output |
| **Complex self-hosting** | Single binary, no Docker, runs offline |
| **Black box debugging** | Runegraph visualization, step-through execution |
| **No type safety** | HLX type system + LSP hardening |
| **Version control issues** | Flows are readable .hlxa files, git-friendly |
| **Memory issues** | Efficient memory model, GPU offloading |
| **Testing is hard** | Compile-time checks, mock inputs, replay |
| **JS-only extensibility** | Native nodes in Rust/HLX, plugin system |
| **Cloud-dependent** | Offline-first, no telemetry |

---

## Architecture

### Current (MVP)
```
┌─────────────┐
│   REST API  │  (axum web server)
│  (Port 3000)│
└──────┬──────┘
       │
       ▼
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│    Flow     │────>│ HLX Compiler │────>│  HLX        │
│  (JSON)     │     │              │     │  Runtime    │
└─────────────┘     └──────────────┘     └─────────────┘
```

### Target (Production)
```
┌──────────────────────────────────────────┐
│      Native Desktop App (Tauri/egui)     │
│  ┌────────────┐      ┌────────────────┐ │
│  │   Canvas   │      │ Node Palette   │ │
│  │  (Visual   │      │ - HTTP         │ │
│  │   Editor)  │      │ - Database     │ │
│  │            │      │ - Files        │ │
│  │  Drag &    │      │ - ML/GPU       │ │
│  │  Drop      │      │ - Custom       │ │
│  └─────┬──────┘      └────────────────┘ │
└────────┼─────────────────────────────────┘
         │
         ▼
┌─────────────────┐
│  Flow Compiler  │
│  (flow.rs)      │
│                 │
│  Flow → HLX     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐     ┌──────────────┐
│  HLX Compiler   │────>│ HLX Runtime  │
│  + Optimizer    │     │ + GPU Backend│
└─────────────────┘     └──────┬───────┘
                                │
                                ▼
                        ┌──────────────┐
                        │   Result +   │
                        │  Trace Data  │
                        └──────────────┘
```

---

## Roadmap

### Phase 1: Native UI Foundation (CURRENT)
**Goal:** Replace web server with native desktop app

**Tasks:**
- [ ] Choose UI framework (Tauri? egui? iced?)
- [ ] Basic window with canvas area
- [ ] Drag-and-drop node creation
- [ ] Visual edge connections
- [ ] Node property editor
- [ ] Save/Load flows as JSON
- [ ] "Compile & Run" button

**Deliverable:** Visual flow editor that compiles to HLX and executes

---

### Phase 2: Core Node Library
**Goal:** 50+ nodes covering common automation tasks

**Categories:**
1. **HTTP** (10 nodes)
   - GET, POST, PUT, DELETE
   - GraphQL query
   - OAuth2 authentication
   - Rate limiting
   - Webhook receiver

2. **Data** (15 nodes)
   - JSON parse/stringify
   - XML parse
   - CSV parse/generate
   - Filter, Map, Reduce
   - Merge, Split
   - Sort, Dedupe

3. **Files** (10 nodes)
   - Read, Write
   - Watch directory
   - S3 upload/download
   - ZIP, Unzip

4. **Database** (10 nodes)
   - PostgreSQL
   - SQLite
   - MongoDB
   - Redis

5. **ML/GPU** (10 nodes)
   - Tensor create/ops
   - Matrix multiply
   - Image processing
   - Model inference

6. **Control Flow** (5 nodes)
   - If/Else
   - Loop
   - Switch
   - Delay
   - Parallel

**Deliverable:** Rich node palette that covers 80% of automation use cases

---

### Phase 3: Execution & Debugging
**Goal:** Real-time execution visualization + replay

**Features:**
- [ ] Live execution view (highlight active nodes)
- [ ] Step-through debugger
- [ ] Breakpoints on nodes
- [ ] Inspect data at each step
- [ ] Runegraph integration (execution trace)
- [ ] Error handling (try/catch nodes)
- [ ] Rollback on failure

**Deliverable:** Developer-friendly debugging experience

---

### Phase 4: Deterministic Superpowers
**Goal:** Leverage HLX axioms for features n8n can't do

**Features:**
- [ ] Time-travel debugging (replay from any point)
- [ ] Diff two flow executions (A2 reversibility)
- [ ] Cryptographic verification of outputs
- [ ] "Undo" button that actually works
- [ ] Snapshot workflow state at any node
- [ ] Resume from snapshot

**Deliverable:** Unique features impossible in non-deterministic systems

---

### Phase 5: Performance & Scale
**Goal:** Handle data pipelines that break n8n

**Features:**
- [ ] GPU acceleration for tensor ops
- [ ] Vulkan compute for parallel loops
- [ ] Streaming data (don't load entire dataset)
- [ ] Incremental execution (only re-run changed nodes)
- [ ] @scale pragma for parallel node execution
- [ ] Distributed execution (future)

**Deliverable:** Process millions of rows without OOM

---

### Phase 6: Polish & UX
**Goal:** "Piss easy for humans to use"

**Features:**
- [ ] Templates for common workflows
- [ ] AI-assisted flow generation (describe in text → generate flow)
- [ ] Smart autocomplete for node connections
- [ ] Visual themes (dark mode, light mode, custom)
- [ ] Keyboard shortcuts for power users
- [ ] Tutorial flows (interactive onboarding)
- [ ] Export to standalone binary
- [ ] Share flows as URLs (base64-encoded .hlxa)

**Deliverable:** Best-in-class UX that makes n8n feel clunky

---

### Phase 7: Extensibility
**Goal:** Plugin ecosystem for custom nodes

**Features:**
- [ ] Plugin API (Rust traits)
- [ ] Node definition DSL
- [ ] Auto-generate node wrappers from OpenAPI specs
- [ ] Community node repository
- [ ] One-click node installation
- [ ] Sandboxed execution for untrusted nodes

**Deliverable:** Let users build their own integrations

---

## Technical Decisions

### UI Framework
**Options:**
1. **Tauri** (HTML/CSS/JS frontend + Rust backend)
   - ✅ Familiar web tech for UI
   - ✅ Great ecosystem
   - ❌ Still using web rendering

2. **egui** (Immediate mode GUI, pure Rust)
   - ✅ Native performance
   - ✅ GPU-rendered
   - ✅ Easy to integrate with HLX
   - ❌ Less mature than web tech

3. **iced** (Elm-inspired, pure Rust)
   - ✅ Clean architecture
   - ✅ Native widgets
   - ❌ Smaller ecosystem

**Recommendation:** Start with **egui** for native performance, fall back to Tauri if UI complexity demands it.

---

### Flow Storage Format
**Current:** JSON blobs (n8n-style)
**Target:** .hlxa files (human-readable, git-friendly)

**Hybrid Approach:**
- UI saves flows as JSON (for metadata like positions)
- JSON includes generated HLX source as string
- Compile button generates clean .hlxa file for version control
- Load .hlxa files back into UI via decompilation

---

### Node Type System
**Strategy:** Each node type = Rust struct implementing `Node` trait

```rust
trait Node {
    fn type_name(&self) -> &str;
    fn compile(&self, inputs: &[String]) -> String; // Generate HLX code
    fn validate(&self) -> Result<(), Error>;        // Check config
    fn icon(&self) -> Icon;                          // UI representation
}
```

Nodes live in `src/nodes/` directory:
- `src/nodes/http.rs` - HTTP nodes
- `src/nodes/data.rs` - Data manipulation
- `src/nodes/ml.rs` - GPU/ML nodes
- etc.

---

## Success Metrics

**MVP Success (Phase 1-2):**
- [ ] Build a simple HTTP → JSON parse → Print flow in UI
- [ ] Compile to HLX and execute
- [ ] Faster than n8n for same workflow

**Production Success (Phase 3-5):**
- [ ] Handle 1M+ row datasets without OOM
- [ ] 10x faster than n8n on data-heavy workflows
- [ ] Time-travel debugging works on real workflows
- [ ] Users can export standalone binaries

**Market Success (Phase 6-7):**
- [ ] 100+ community nodes
- [ ] Featured on Hacker News
- [ ] n8n users migrating to Autograph
- [ ] Companies using it for production automation

---

## Next Steps (Immediate)

1. **Choose UI framework** (egui vs Tauri)
2. **Prototype canvas** - Drag nodes, draw edges
3. **Integrate flow compiler** - Visual flow → HLX code generation
4. **Test end-to-end** - Build simple workflow, compile, execute

Once Phase 1 MVP is done, iterate rapidly on node types (Phase 2).

---

## Why This Will Win

**n8n's weakness:** Built on Node.js, inherits all its problems (slow, non-deterministic, memory hungry)

**Autograph's strength:** Built on HLX, inherits all its advantages (fast, deterministic, verifiable, GPU-accelerated)

**Key insight:** Workflow automation is fundamentally about **data transformation**. HLX is designed for exactly this - deterministic, composable, verifiable transformations. n8n is fighting against JavaScript's chaos. Autograph is surfing HLX's determinism.

**The killer feature:** Time-travel debugging. Show me a single n8n user who wouldn't kill for "click any node, see exact data at that point, rewind and replay from there." That's trivial in HLX (A2 reversibility). Impossible in n8n.

---

**Let's build this thing.** 🚀

**Owner:** Claude Sonnet 4.5
**Started:** January 15, 2026
**Status:** Planning → Implementation
