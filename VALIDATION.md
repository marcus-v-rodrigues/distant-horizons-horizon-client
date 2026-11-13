# Build and validation record

## Target

- Horizon Nightly: `2026-11-08-2`
- Horizon commit: `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca`
- Minecraft: `26.3`
- Mod: `0.1.0`

## Automated checks

| Check | Result |
|---|---|
| `python3 tools/horizon.py stage` | Passed; native bundle applied to the pinned source. |
| `python3 tools/horizon.py check` | Passed; Horizon client type-check completed. |
| `python3 tools/horizon.py test` | Passed; 41 mod tests ran in Horizon's test binary. |
| `cargo test --lib --tests` | Passed; 70 standalone unit and integration tests. |
| `python3 tools/horizon.py build` | Passed; optimized Horizon client built. |
| `python3 tools/horizon.py package` | Passed; native source bundle and distribution ZIP created. |

The client build took approximately three minutes in this environment after dependencies were available. Linux builds may require system development packages; this environment needed ALSA and udev development libraries.

## Package outputs

- Client: `target/cargo/fast/horizon-client`
- Native bundle: `target/distant_horizons-0.1.0.hnmod`
- Bundle SHA-256: `b2d674aeea56663b2b3459497b00d0b10452defde3ca655f1e5ba607ac1e4854`
- Distribution ZIP: `target/distant-horizons-horizon-client-0.1.0.zip`
- ZIP SHA-256: `d2a83e6360196c7634bdfd53e9ec79c35e467432d2e12c1c8c996ffa4579a3e7`

These outputs are local build artifacts. The `.hnmod` contains native source to be compiled into a client; it is not a prebuilt game executable.

## Not yet tested on this target

- Launching a game with privately prepared Minecraft resources.
- In-game settings, commands, world generation, saving, and reload behavior.
- Visual quality across detail levels, biomes, lighting modes, or shader configurations.
- Multiplayer and LAN behavior.
- Performance, memory use, and long-running sessions.

A successful compile and automated test run confirms source compatibility for the checked build paths. It does not prove that the renderer looks correct in a running game. Complete those runtime checks before treating the Nightly as fully validated.

