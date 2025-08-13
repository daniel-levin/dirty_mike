# `dirty_mike`

> Quick and dirty self-instrumentation for Rust

## Example
See `dirty_mike/examples/basic.rs`.
This compares `serde_json` and `toml`'s parsing of an equivalent payload.

### Sample Output (Release Mode)

```
Data sizes:
  JSON: 5458 bytes
  TOML: 5222 bytes

Performance metrics:
JSON parsing:
  cycles/b: 22.68
  instructions/b: 35.53
  total cycles: 123772
  total instructions: 193915
  time running: 54.366us
  time enabled: 54.366us

TOML parsing:
  cycles/b: 80.86
  instructions/b: 126.45
  total cycles: 422265
  total instructions: 660299
  time running: 173.791us
  time enabled: 173.791us

Comparison (JSON vs TOML):
  cycles/b (JSON): 22.68
  cycles/b (TOML): 80.86
  instructions/b (JSON): 35.53
  instructions/b (TOML): 126.45
  time running (JSON): 54.366us
  time running (TOML): 173.79us
  IPC (JSON): 1.57
  IPC (TOML): 1.56
```
