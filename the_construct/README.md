# The Construct 🌌
**A Procedural Reality Engine built in HLX**

> "This isn't a game engine. This is a language executing its own reality."

## Concept
The Construct is a 3D visualization engine that generates its content procedurally on the GPU. It relies on **HLX Determinism** to ensure that the universe it generates is invariant—every atom is exactly where it should be, every time, on every machine.

## Architecture
- **Language**: HLX (Helix)
- **Runtime**: Native HLX Runtime (Rust + LLVM)
- **Graphics**: Vulkan Compute & Graphics Pipelines
- **Windowing**: SDL2 (via HLX Contracts)

## Roadmap
1.  **Phase 1: The Pulse** - ✅ SDL2 Window + Main Loop (Input/Render).
    - Status: Compiled to native `construct` binary. Linked with `libSDL2`.
2.  **Phase 2: The Void** - Vulkan Context Initialization + Clear Screen.
3.  **Phase 3: The Grid** - Procedural mesh generation (Compute Shaders).
4.  **Phase 4: The Flow** - Physics & Animation.

## Building & Running

**Prerequisites:**
- `gcc`
- `libsdl2-dev`

**Build:**
```bash
# 1. Compile HLX to Object File
../hlx-compiler/hlx/target/release/hlx compile-native src/main.hlxa -o construct.o --target x86_64-unknown-linux-gnu

# 2. Link (Disable PIE for now)
gcc -no-pie construct.o -o construct -lSDL2 -lm

# 3. Run
./construct
```

## Logs
- `2026-01-08`: Initial skeleton successfully linked and executed. 
- NOTE: Requires GUI environment for SDL video init. Headless environments will exit with code 255.