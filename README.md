# wsl-shim

A tiny Windows executable that forwards commands to WSL.

Rename the compiled binary to match the tool you want to shim (e.g. `docker.exe`, `mise.exe`), place it on your PATH, and it will transparently call the Linux version inside WSL.

## Build

```sh
cargo build --release
```

The binary will be at `target/release/wsl-shim.exe` — rename it to whatever you need.

## Config

Place a `wsl-shim.toml` next to the exe to control behavior.

| Key     | Type | Default | Description                                      |
|---------|------|---------|--------------------------------------------------|
| `login` | bool | `false` | Run via `bash -lc` to load shell profile         |

Example:

```toml
login = true
```

Use `login = true` for tools that are only available after your shell profile loads (e.g. `mise`). Leave it `false` (or omit the file) for tools already on the default PATH (e.g. `docker`).


## Install

Download the latest release and rename it to the tool you want to shim:

```powershell
curl -Lo wsl-shim.exe https://github.com/<owner>/wsl-shim/releases/latest/download/wsl-shim-x86_64-pc-windows-msvc.exe
mv wsl-shim.exe "$env:USERPROFILE\.local\bin\docker.exe"
```

Make sure `%USERPROFILE%\.local\bin` is on your `PATH`.
