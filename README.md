# gide

**gide** is an experiment in building a modern IDE with web technologies while preserving that distinctive JetBrains-like interface feel.

The idea is simple and ambitious: take the flexibility of the VS Code approach (frontend as a web app), keep the expression and atmosphere of a desktop IDE, and replace heavyweight Electron with **Tauri** + **Rust**.

---

## Why this project exists

We want to test a hypothesis:

- can we recreate the UI/UX magic of a JetBrains-level IDE;
- can we do it on a modern web stack (Vue + Vite);
- can we deliver a native desktop app with lower resource usage thanks to Tauri.

`gide` is not just an editor with panels, but an attempt to find the balance between interface beauty, speed, and engineering practicality.

## Core philosophy

- **Interface as a thinking tool**: UI should help you focus on code, not distract you.
- **Web approach without desktop quality compromises**: fast iterative frontend and a native shell.
- **Lightweight over heavyweight**: Tauri instead of Electron where footprint and performance matter.

## What is already implemented

The current prototype includes a basic IDE layout:

- top menu and dropdown navigation;
- sidebars (left and right);
- code area;
- file tree panel;
- bottom terminal panel;
- modular Vue component structure for further growth.

## Technology stack

### Frontend

- **Vue 3**
- **Vite 6**
- CSS style modules for UI layers

### Desktop / Backend shell

- **Tauri 2**
- **Rust 1.94**
- `serde`, `serde_json`

## Why Tauri instead of Electron

- smaller final app size;
- lower memory usage in typical scenarios;
- native backend performance with Rust;
- natural integration with system capabilities through the Tauri API.

## Quick start

### Requirements

- Node.js + npm
- Rust toolchain (`rustup`, `cargo`)
- Tauri system dependencies for Linux/macOS/Windows

### Install dependencies

```bash
npm install
```

### Run in development mode

```bash
npm run tauri dev
```

### Build frontend

```bash
npm run build:web
```

### Build desktop application

```bash
npm run build
```

## Project structure

```text
gide/
├── src/                  # Vue frontend
│   ├── components/       # IDE UI components
│   ├── styles/           # Interface styles
│   └── views/            # Screen-level views
├── src-tauri/            # Tauri + Rust part
│   ├── src/              # Rust entry points
│   └── tauri.conf.json   # Desktop app configuration
└── package.json          # npm scripts and frontend dependencies
```

## Roadmap

- [ ] Expand menu and command system (command palette / actions)
- [ ] Connect real filesystem operations
- [ ] Improve code editor capabilities (highlighting, tabs, state)
- [ ] Evolve terminal and integration with dev workflow
- [ ] Add theme settings, UX polish, and keyboard shortcuts
- [ ] Design an extensibility architecture (plugins/modules)

## Status

The project is in an active prototyping phase.

Right now, the focus is on building a strong UI/UX foundation to confidently scale toward IDE-level functionality without sacrificing performance.

---

If the idea of a lightweight yet powerful next-generation IDE resonates with you, welcome to `gide`.
