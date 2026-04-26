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

The current prototype includes a functional IDE layout with dynamic panels:

- **JetBrains-style Sidebar**: A multi-tab left sidebar (Project, Commit, Pull Requests) with exclusive toggle logic (only one panel open at a time).
- **Dynamic Workspace**: Code editor area with tab support (open, close, switch).
- **Tool Window Management**: Bottom panel for context-heavy tools like **Terminal** and **Git**, sharing the same space with the ability to toggle between them.
- **File System Integration**: Rust-powered backend commands to read directory trees and file contents.
- **Responsive Resizing**: Interactive vertical and horizontal resizers for sidebars and tool windows.
- **Modern UI Components**: Modular Vue-based structure including Header, Footer, Sidebars, and specialized views.

## Technology stack

### Frontend

- **Vue 3 (Composition API)**
- **Vite 6**
- Custom CSS for high-fidelity IDE look and feel

### Desktop / Backend shell (Tauri 2.0)

- **Rust 1.94**
- **Native FS Commands**: Optimized directory tree building and file reading.
- `serde`, `serde_json` for seamless bridge between Rust and Vue.

## Why Tauri instead of Electron

- Smaller final app size (no bundled Chromium).
- Significantly lower memory usage.
- Native backend performance with Rust for heavy tasks like indexing or FS watching.
- Natural integration with system capabilities through the Tauri 2.0 API.

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

### Build desktop application

```bash
npm run build
```

## Project structure

```text
gide/
├── src/                  # Vue frontend (composition API)
│   ├── components/       # UI Components (Editor, Terminal, Git, etc.)
│   ├── views/            # Main layout orchestration (EditorView.vue)
│   └── styles/           # Global and component-specific styling
├── src-tauri/            # Tauri + Rust core
│   ├── src/              # Rust commands and logic
│   └── tauri.conf.json   # Desktop app and bundle configuration
└── package.json          # npm scripts and dependencies
```

## Roadmap

- [ ] **Real Terminal**: Integrate Xterm.js for a fully functional terminal emulator.
- [ ] **Advanced Git**: Stage/Commit UI and interactive history log.
- [ ] **Editor Enhancements**: Syntax highlighting, code completion, and state persistence.
- [ ] **Global Command Palette**: Quick access to actions and files (Ctrl+Shift+A / Ctrl+P).
- [ ] **Plugin System**: Modular architecture for extending IDE functionality.

## Status

The project is in an active prototyping phase.

Right now, the focus is on building a strong UI/UX foundation and optimizing the Rust-Vue bridge to ensure the IDE feels snappy and responsive.

---

If the idea of a lightweight yet powerful next-generation IDE resonates with you, welcome to `gide`.
