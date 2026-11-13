# Code structure and data flow

This document explains how the mod is organized and how a terrain section moves through it. It describes the current Nightly target in [compatibility.json](compatibility.json).

## Key terms

- **Chunk:** Horizon's normal, high-detail unit of nearby terrain.
- **LOD:** level of detail; a simplified representation used farther from the player.
- **Data point:** one vertical run of terrain information, such as a block type, biome, height, and light values.
- **Section:** a square region containing LOD data. Small sections are combined into larger, coarser sections.
- **Near cutout:** the area where ordinary chunks take precedence over far-terrain LODs.

## Data flow

1. **Capture.** The Horizon bridge reads loaded chunks and samples generated terrain where distant generation is enabled.
2. **Normalize.** Terrain is represented as vertical runs and retained with the relevant biome and lighting information.
3. **Store.** Section data is written to world storage. Derived caches can be rebuilt; the original world chunks are not replaced by LOD data.
4. **Propagate.** Updates flow from detailed sections to larger sections. A coarser representation is not allowed to erase more complete data.
5. **Plan detail.** Distance and the horizontal-quality setting determine which section detail levels are requested.
6. **Mesh.** Runs become terrain faces. Occlusion, neighboring columns, light, materials, and the near cutout determine which faces are emitted.
7. **Draw.** Horizon's renderer draws the resulting far terrain alongside nearby chunks.

## Source layout

| Path | Purpose |
|---|---|
| `mixins/distant_horizons/runtime.rs` | Horizon-independent LOD logic, storage, settings, and core tests. |
| `mixins/distant_horizons/bridge.rs` | Connects the core to Horizon's chunks, world generator, rendering data, and world lifecycle. |
| `mixins/distant_horizons/*.rs` | Focused helpers for rendering, capture, settings, commands, and lifecycle hooks. |
| `mixins/distant_horizons/horizon-mixin.toml` | Declares the modules and hook points applied to the pinned Horizon source. |
| `tools/horizon.py` | Stages Horizon, applies the bundle, runs checks and tests, builds the client, and creates a package. |
| `compatibility.json` | Records the exact Horizon and mixin-tool revisions and the source archive checksum. |

## Rendering details

Distant Horizons data is not a second copy of every nearby block. Each section stores a reduced view suitable for long distances. As the player moves, the detail plan changes and section meshes are regenerated as necessary.

The mesh builder removes covered faces, merges compatible adjacent faces, and applies the texture and biome information supplied by Horizon. Water and aquatic vegetation need additional capture and lighting handling because coarse terrain reduction can otherwise lose important states or create a visible transition where detailed chunks meet far terrain.

A near cutout prevents LOD faces from drawing over currently available ordinary chunks. Updates must retain this boundary information so that changing render distance does not expose empty terrain or stale meshes.

## Compatibility model

This mod directly reads Horizon internals that are not stable native hook points. The manifest therefore pins a specific Horizon commit. A different Nightly must be treated as a separate port: stage the code on that revision, resolve API or behavior changes, run the complete automated checks, and perform an in-game review.

The current code type-checks and builds on the Nightly listed in `compatibility.json`. Its automated test results are in [VALIDATION.md](VALIDATION.md). A successful build alone does not establish visual or multiplayer compatibility.

## Safe development workflow

1. Read the compatibility pin before changing the target.
2. Make a focused source change and add a regression test when practical.
3. Run `cargo test --lib --tests` for the core tests.
4. Run `python3 tools/horizon.py check` and `python3 tools/horizon.py test` against the pinned source.
5. Build with `python3 tools/horizon.py build`.
6. For changes to capture, lighting, or rendering, prepare a disposable world and compare screenshots before and after. Do not use a valuable world as a test fixture.
7. Update [VALIDATION.md](VALIDATION.md) with exactly what was tested and what remains untested.

## Saved data and limits

LOD files and caches are derived terrain data. Cache format changes may cause LOD data to be rebuilt on first use. This is separate from Minecraft chunk storage: the mod is not intended to overwrite player edits or world-generation data.

Far terrain is an approximation. Fine vegetation, water color transitions, lighting, and geometry may differ from nearby chunks. Results in one seed, camera, lighting mode, or render distance do not certify all scenes.
