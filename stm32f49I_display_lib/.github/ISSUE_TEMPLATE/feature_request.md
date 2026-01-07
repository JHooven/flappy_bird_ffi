---
name: Feature Request
about: Propose a new API or capability using the feature template
title: "Feature: <short title>"
labels: [enhancement]
assignees: []
---

## Context
- MCU/Display: <e.g., STM32F49x, controller, bus>
- Pixel format & resolution: <e.g., RGB565, 320x240>
- Constraints: `no_std`/heap policy, timing, power, allowed crates

## Goal
- What the user can do after this change (one sentence).

## API Sketch
- Signatures and types you expect (pseudocode ok):
  - `fn ... -> Result<..., Error>`
  - `impl ... { fn ... }`

## Constraints
- Performance bounds, memory limits, blocking vs async, `unsafe` policy.

## Edge Cases
- Zero/degenerate inputs, off-screen/partial rendering, overflow, large thickness, malformed inputs, etc.

## Acceptance Criteria
- Compiles; tests pass; example renders as expected; docs updated with runnable examples.

## Test Plan
- Unit + property tests; simulator golden-image diffs; fixtures for malformed inputs.

## Artifacts
- Files to create/update (paths), examples, docs.

## Open Questions
- Any ambiguity to confirm (e.g., clipping, thickness semantics, coordinate origin).