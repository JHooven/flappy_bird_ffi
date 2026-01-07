# Project Prompting Workflow

A lightweight way to drive this embedded display library in small, testable iterations. Use the templates below to open focused tasks and track progress.

---

## How To Use
- Start with a single Project Brief (below). Then iterate one feature per prompt.
- Keep each prompt self‑contained: goal, API sketch, constraints, edge cases, tests, and artifacts.
- Reference files by path and prefer deltas when revising behavior.

---

## Project Brief (imported from copilot_prompts_1.md)

What would be a good workflow for the following project?

1) Generate a comprehensive library to expose an api that allows the user to:
   1.1) Initialize the display library
   1.2) Show an image (.bmp, .jpeg, etc.) on the screen
   1.3) Draw a shape outline
   1.3.1) Rectangle
   1.3.2) Triangle 
   1.3.3) Hexagram (six‑point star)
   1.3.4) Circle 
   1.3.5) Etc.
   1.3.6) Should have parameters for line thickness and color
   1.4) All functions and functionality should have detailed test functions written for the including "corner cases"

---

## Suggested Iteration Order
1. Display init: configuration, reset, orientation, pixel format.
2. Framebuffer/pixel I/O abstraction + clipping rules.
3. Shape outlines: rectangle → triangle → circle → hexagram → polyline.
4. Image rendering: BMP (uncompressed) → JPEG (decoder) → others as feasible.
5. Color handling: RGB565 pipeline, conversions, blending policy (if needed).
6. Test infrastructure: host framebuffer simulator + golden images; property tests for geometry; corner cases.
7. Examples and docs for each API surface.

---

## Progress Snapshot
- Implemented: Display init (`init_display`), framebuffer clear, pixel access.
- Implemented: Rectangle outline with clipping and thickness.
- Implemented: Triangle outline using Bresenham with thickness.
- Tests: `tests/display_init.rs`, `tests/shapes_rect.rs`, `tests/shapes_triangle.rs` all passing.
- Examples: `examples/rect.rs`, `examples/triangle.rs` (outputs PPM files for visual checks).

---

## Feature Prompt Template

Title: <Short, actionable feature>

Context:
- MCU/display: <STM32F49x, bus type, controller>
- Pixel format: <RGB565>, resolution, `no_std`/heap policy, timing constraints
- Allowed crates: e.g., `embedded-graphics`, `tinybmp`, `jpeg-decoder` (or custom only)

Goal:
- One clear outcome the user can do after this change.

API Sketch:
- Function signatures, types, and error variants you expect, e.g.:
  - `fn init_display(cfg: DisplayConfig) -> Result<Display, Error>`
  - `fn draw_rectangle_outline(&mut self, rect: Rect, color: Rgb565, thickness: u8) -> Result<(), Error>`

Constraints:
- Performance bounds, memory limits (no heap?), blocking vs async, `unsafe` policy.

Edge Cases:
- Exhaustive list: zero sizes, off‑screen and partial, overflow, large thickness, degenerate shapes.

Acceptance Criteria:
- Must compile, tests pass, example renders expected output, docs updated with runnable examples.

Test Plan:
- Unit + property tests; simulator golden‑image diffs; any hardware‑in‑loop checks (optional).

Artifacts:
- Files to create/update (paths), examples, docs.

Open Questions:
- Any ambiguity to confirm (e.g., thickness semantics, clipping, coordinate origin).

---

## Issue/Bug Prompt Template

Summary:
- One‑line problem statement.

Environment:
- Toolchain/target/hardware; branch or commit SHA.

Steps to Reproduce:
- Minimal steps or code.

Expected vs Actual:
- Be explicit.

Logs/Artifacts:
- Error text, backtraces, failing tests, screenshots.

Scope/Impact:
- What’s blocked and urgency.

Hypothesis (optional):
- Your suspicion; we’ll verify.

---

## Delta Prompt Template (for revisions)

Summary of Change:
- What’s different from the previous spec.

Reason:
- Why the change is needed (bug, performance, UX, hardware limits).

Impacted APIs/Files:
- List affected signatures and file paths.

Updated Acceptance Criteria & Tests:
- What must now be true; new/updated tests.

---

## Example Feature Prompts

### Example 1 — Display Initialization
- Context: STM32F49x, RGB565, 320x240, SPI display via HAL, `no_std`, no heap.
- Goal: Provide `init_display()` to configure controller, orientation, and clear screen.
- API Sketch:
  - `fn init_display(cfg: DisplayConfig) -> Result<Display, Error>`
  - `impl Display { fn clear(&mut self, color: Rgb565) -> Result<(), Error> }`
- Constraints: ≤ 50 ms init; no dynamic allocation; retries on transient bus errors.
- Edge Cases: invalid cfg (dimensions, orientation), bus timeouts, repeated init.
- Acceptance: Unit tests for cfg validation; doc example compiles; simulator clears to color.
- Artifacts: `src/lib.rs`, `src/display.rs`, tests in `tests/display_init.rs`, example `examples/init.rs`.

### Example 2 — Rectangle Outline
- Context: Same as above; hard clipping within bounds.
- Goal: `draw_rectangle_outline(rect, color, thickness)` draws perimeter inset by thickness.
- API Sketch:
  - `fn draw_rectangle_outline(&mut self, rect: Rect, color: Rgb565, thickness: u8) -> Result<(), Error>`
- Constraints: ≤ 1 ms for 100x50, thickness 1–10.
- Edge Cases: zero size, thickness > side/2, off‑screen/partial, overflow coords.
- Acceptance: Property tests for perimeter coverage; golden image matches; doc example builds.
- Artifacts: `src/shapes.rs`, tests `tests/shapes_rect.rs`, example `examples/rect.rs`.

### Example 3 — BMP Rendering
- Context: Host‑side BMP decode; stream to display via pixel iterator; no heap.
- Goal: Render an in‑memory BMP (16‑bit or converted to RGB565) at `(x, y)` with clipping.
- API Sketch:
  - `fn draw_bmp(&mut self, origin: Point, bmp: &BmpImage) -> Result<(), Error>`
- Constraints: No heap; bounded stack; handle stride/padding; clip efficiently.
- Edge Cases: malformed header, unsupported bpp, negative origins, full/partial off‑screen.
- Acceptance: Unit tests with tiny BMP fixtures; golden images for several sizes; error cases validated.
- Artifacts: `src/images/bmp.rs`, tests `tests/images_bmp.rs`, example `examples/bmp.rs`.

---

## Artifacts & Paths (recommended)
- Library: `src/lib.rs`, `src/display.rs`, `src/shapes.rs`, `src/images/mod.rs`, `src/images/bmp.rs`, `src/images/jpeg.rs`
- Tests: `tests/display_init.rs`, `tests/shapes_*.rs`, `tests/images_*.rs`
- Examples: `examples/init.rs`, `examples/rect.rs`, `examples/bmp.rs`
- Docs: doc comments + this `PROMPTS.md`

---

## Testing Strategy (at a glance)
- Geometry: unit + property tests (degenerate/edge inputs).
- Rendering: host framebuffer simulator and golden‑image diffs.
- Images: small fixtures, malformed cases, clipping paths.
- Performance: micro‑bench on host where feasible; spot‑checks on hardware later.

---

If you want, we can generate GitHub issue templates from these. Just say the word and which sections you want by default.