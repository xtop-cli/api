# xtop api

Shared API crates for the xtop ecosystem. Kernel, plugins, effects and
extensions all depend on these crates — they define the contracts between the
core and every extension point.

## Workspace

```
crates/
  plugin-api      -> xtop-plugin-api
  effect-api      -> xtop-effect-api
  extension-api   -> xtop-extension-api
```

## Crates

| Crate | Package | Purpose |
|---|---|---|
| `plugin-api` | `xtop-plugin-api` | Plugin protocol: manifest, capabilities, errors, lifecycle events, data/widget contracts |
| `effect-api` | `xtop-effect-api` | Effect protocol: animation frames and rendering events for the TUI |
| `extension-api` | `xtop-extension-api` | Extension protocol: hooks around config, layout, theme and render pipeline |

The crates are published to crates.io and consumed by:

- `xtop` kernel (github.com/xtop-cli/xtop)
- `plugins` repo (github.com/xtop-cli/plugins)
- `effects` repo (github.com/xtop-cli/effects)
- `extensions` repo (github.com/xtop-cli/extensions)

## License

MIT
