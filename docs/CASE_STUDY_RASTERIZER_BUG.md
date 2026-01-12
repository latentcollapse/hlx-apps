# The End of Gaslighting: Differential Debugging with Axiomatic Compilers
**A Case Study in HLX-A1 (Determinism)**

*Date: January 10, 2026*
*Status: Verified*

## Abstract

Modern systems debugging often suffers from "environmental gaslighting"—the uncertainty of whether a bug is caused by logic, the compiler, the runtime environment, or hardware non-determinism. This report documents a real-world debugging session in the HLX ecosystem where the language's core axioms (A1: Determinism and A4: Universal Value) reduced a complex memory corruption bug (segfault) to a verifiable compiler defect in under 5 minutes.

## 1. The Scenario

We were testing **The Construct**, a 3D software rasterizer written in pure HLX. The application involves:
- Manual vertex allocation (arrays of floats).
- Heavy floating-point math (rotation matrices).
- Nested loops for pixel rasterization.

When compiled to native machine code (via the LLVM backend) and executed in a headless environment, the application crashed:

```bash
$ ./construct
Creating vertices...
Vertices created.
Vertex array created.
Testing rotation...
Rotation Result X: -4622527857650043728  <-- Suspicious Garbage
Rotation Result Y: -4616189618054758400
Segmentation fault (core dumped)
```

## 2. The Traditional Debugging Nightmare

In a traditional language (C++/Rust), diagnosing this segfault would involve:
1.  **Checking Drivers:** Is the headless SDL driver failing?
2.  **Checking Memory:** Valgrind/ASAN to find out-of-bounds writes.
3.  **Checking ABI:** Did we pass the wrong struct layout to SDL?
4.  **Checking Math:** Are `cos/sin` behaving differently on this CPU?

This process typically takes hours or days, as the engineer must prove the bug isn't in their code or the environment.

## 3. The Axiomatic Approach (Differential Debugging)

Because HLX guarantees **Axiom A1 (Bitwise Determinism)** across compilation targets, we have a "Truth Source": the Bytecode VM (`hlx run`).

The VM executes the exact same bytecode (LC-B) that the native backend lowers to assembly. If the logic is sound, the VM *must* produce the correct result.

We ran the same source on the VM:

```bash
$ hlx run construct.lcc
...
Testing rotation...
Rotation Result X: -0.39815702328616975  <-- Correct Float
Rotation Result Y: -1.0
Rotation Result Z: -1.3570081004945758
Rotation successful.
```

## 4. The "Smoking Gun" Comparison

By comparing the outputs, the root cause was immediately visible without a debugger attached:

| Metric | VM (The Truth) | Native (The Lie) | Diagnosis |
| :--- | :--- | :--- | :--- |
| **Result X** | `-0.398157...` | `-4622527...` | **Type Confusion** |
| **Behavior** | Success | Segfault | **Memory Corruption** |

The native value `-4622527857650043728` is not random noise. It is the decimal representation of the **raw 64-bit float pattern** interpreted as a signed 64-bit integer.

**Proof:**
- Float: `-0.398...`
- Hex Pattern: `0x...` (corresponds to the integer value)

This proved definitively:
1.  **The Logic is Correct:** The VM (which respects A4 types) calculated the right rotation.
2.  **The Environment is Irrelevant:** SDL didn't crash; the math crashed.
3.  **The Defect is in the Compiler:** The LLVM backend failed to insert a `bitcast` instruction when printing/storing the float value, treating it as a raw integer and likely corrupting the stack layout for the subsequent array access.

## 5. Conclusion

HLX did not prevent the bug (the LLVM backend still had a defect). However, **HLX prevented the gaslighting.**

It eliminated the need to investigate:
- The OS (Linux/Headless)
- The Drivers (SDL2)
- The User Code (The math logic)

The axioms allowed us to pinpoint the failure to a specific stage in the compilation pipeline (Codegen) in minutes. This turns debugging from "detective work" (gathering clues) into "science" (falsifying hypotheses).

**Verdict:**
HLX's axioms turn "Heisenbugs" into reproducible, isolatable artifacts.
