# Amoret

- Rust 2021, async with tokio, serde for config, clap for CLI
- Section labels as C-block comments: `/* mods */`, `/* imports */`, `/* structs */`, `/* fns */`
- 4-space indent, snake_case fns/vars, PascalCase types
- Discord RPC via `discord-rich-presence` crate
- Config reloads on file change (3s poll); Steel (Scheme) plugin rewrites config via `watch` channel
- `--daemon` forks background (Unix process group / Windows detach)
