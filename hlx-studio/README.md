# HLX Dev Studio

The official IDE for the HLX language, written *in* HLX.

## Architecture
- **Language:** HLX (compiled via `hlx compile-native`)
- **Graphics:** SDL2 (via HLX intrinsics)
- **Status:** Skeleton / Prototype

## Building

```bash
# From hlx-apps/hlx-studio
../../hlx-compiler/hlx/target/release/hlx compile-native src/main.hlxa -o studio.o --target x86_64-unknown-linux-gnu
gcc -no-pie studio.o -o studio -lSDL2 -lm
./studio
```

## Features (Planned)
- [x] Window & Renderer
- [x] Basic Rect Drawing (Software Fill)
- [ ] Text Rendering (Bitmap Font)
- [ ] Text Editor Widget
- [ ] File Browser
- [ ] Syntax Highlighting
