# Triggerware SDKs

Triggerware SDKs are typed clients for communicating with a Triggerware server without constructing JSON-RPC requests directly. They expose Triggerware tables, queries, triggers, agents, tools, connectors, and other runtime operations through native language APIs.

## Install Triggerware

Install the Triggerware server and built-in `tw` connector using the installer for your platform.

### macOS

```sh
curl -fsSL https://tw2-releases.s3.us-east-2.amazonaws.com/install.sh | sh
```

### Windows

```powershell
irm https://tw2-releases.s3.us-east-2.amazonaws.com/install.ps1 | iex
```

## SDKs

Triggerware supports [Python](python/README.md) (`pip install triggerware`), [Rust](rust/README.md) (`cargo add triggerware`), and [Node.js](javascript-sdk/README.md) (`npm install triggerware`).
