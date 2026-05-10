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

The current prototype includes a fully functional IDE interface with dynamic panels and a working Rust-Vue bridge:

### Frontend Features
- **JetBrains-style Left Sidebar**: Multi-tab toolbar with exclusive toggle logic (Project Tree, Commit, Pull Requests, Todo) — only one panel visible at a time.
- **Code Editor with Tab Management**: 
  - Open, close, and switch between multiple files
  - Line numbering with synchronized scrolling
  - Real-time content editing
  - Undo/Redo infrastructure (prepared for integration)
- **Dynamic Bottom Panel**: Context-heavy tools (Terminal, Git, Todo) sharing the same space with toggle capability.
- **Responsive Layout**: 
  - Drag-to-resize left sidebar (140px–460px)
  - Drag-to-resize bottom panel (120px–max)
  - Right sidebar for future extensions
- **Modern UI Components**: Modular Vue 3 structure with Header, Footer, Left/Right Sidebars, and specialized views.

### Backend (Rust) Features
- **File System Integration**:
  - `read_directory_tree()`: Recursively builds file tree with excluded directories (`node_modules`, `.git`, `target`)
  - `read_text_file()`: Load file contents into editor
  - `save_file()`: Persist editor changes to disk
  - `get_default_root()`: Auto-detect project root directory
- **Smart Directory Filtering**: Automatically excludes build and dependency folders
- **Performance Optimized**: Sorted directory listings (directories first) for intuitive file browsing

## Technology stack

### Frontend

- **Vue 3** (Composition API with `<script setup>` syntax)
- **Vite 6** (fast module bundler and dev server)
- **Custom CSS** for high-fidelity JetBrains-style UI
- **Tauri API v2** for seamless desktop integration

### Backend (Rust)

- **Rust 1.94** (2021 edition)
- **Tauri 2.0** (lightweight desktop framework)
- **Serde & Serde JSON** (JSON serialization for Vue-Rust bridge)

### Key Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| `tauri` | `2` | Desktop application framework |
| `serde` | `1.x` | Serialization/deserialization |
| `serde_json` | `1.x` | JSON handling |
| Vue | `3.5.13` | Frontend framework |
| Vite | `6.0.3` | Build tool |
| @tauri-apps/api | `2.x` | Tauri API bindings |

## Backend Commands (Tauri)

The following Rust commands are available to the Vue frontend via `invoke()`:

### Filesystem Operations

#### `get_default_root() → Result<String, String>`
Returns the default project root directory (parent of `src-tauri` or current directory).

```javascript
const rootPath = await invoke('get_default_root');
```

#### `read_directory_tree(root_path: String) → Result<FileTreeNode, String>`
Recursively builds a file tree starting from `root_path`. Automatically excludes `node_modules`, `.git`, and `target` directories. Returns sorted structure (directories first).

```javascript
const tree = await invoke('read_directory_tree', { rootPath: '/my/project' });
// Returns: { name, path, isDir, children: [...] }
```

#### `read_text_file(file_path: String) → Result<String, String>`
Reads entire file contents as a UTF-8 string.

```javascript
const content = await invoke('read_text_file', { filePath: '/path/to/file.js' });
```

#### `save_file(file_path: String, content: String) → Result<(), String>`
Writes or overwrites file with the given content.

```javascript
await invoke('save_file', { 
  filePath: '/path/to/file.js', 
  content: 'console.log("hello");' 
});
```

## Why Tauri instead of Electron

- **Significantly smaller app size**: No bundled Chromium (typically 150MB+ smaller).
- **Lower memory footprint**: Uses native WebKit/WebView instead of full browser engine.
- **Native backend performance**: Rust for heavy tasks like directory indexing without IPC overhead.
- **Natural system integration**: File access, process management, and OS APIs through Tauri's safe command layer.
- **Security model**: Message-based communication with strict scope control prevents backend vulnerabilities from cascading to the UI.

## Quick start

### Requirements

- **Node.js 16+** with npm
- **Rust toolchain** (`rustup` 1.90+, `cargo`)
- **Tauri system dependencies**: 
  - **Windows**: No additional dependencies
  - **macOS**: Xcode command line tools (`xcode-select --install`)
  - **Linux**: `libwebkit2gtk-4.1-dev`, `build-essential`, `curl`, `wget` (see [Tauri docs](https://tauri.app/v1/guides/getting-started/prerequisites))

### Install dependencies

```bash
npm install
```

This installs both the Vue and Tauri CLI tooling.

### Run in development mode

```bash
npm run tauri dev
```

This starts the Vite dev server on `localhost:5173` and opens the Tauri window with hot module reloading.

### Build for desktop

```bash
npm run build
```

Builds both the Vue frontend and Rust backend into a distributable application bundle for your platform.

### Additional commands

- `npm run dev` — Run Vite dev server (frontend only, without Tauri window)
- `npm run build:web` — Build Vue frontend for web distribution
- `npm run preview` — Preview production build locally

## Project structure

```
gide/
├── src/                           # Vue 3 frontend (Composition API)
│   ├── components/                # Reusable UI components
│   │   ├── CodeAreaComponent.vue  # Editor with tabs, line numbers, content sync
│   │   ├── FileThreeComponent.vue # File tree builder from open files
│   │   ├── LeftSideBarComponent.vue # Left panel container (Project/Commit/PR/Todo)
│   │   ├── TerminalComponent.vue  # Terminal panel (placeholder)
│   │   ├── GitComponent.vue       # Git panel (UI ready)
│   │   ├── CommitComponent.vue    # Commit panel (UI ready)
│   │   ├── PullRequestsComponent.vue # PR panel (UI ready)
│   │   ├── TodoComponent.vue      # Todo panel (UI ready)
│   │   ├── HeaderComponent.vue    # Top toolbar (file ops, sidebar toggle)
│   │   ├── RightSideBarComponent.vue # Right sidebar (extensible)
│   │   └── FooterComponent.vue    # Status bar
│   ├── views/
│   │   └── EditorView.vue         # Main layout orchestrator (tabs, resizing, panel state)
│   ├── styles/                    # Component-specific CSS
│   │   ├── CodeArea.css
│   │   ├── FileThree.css
│   │   ├── Terminal.css
│   │   ├── Header.css
│   │   ├── Footer.css
│   │   └── style.css              # Global variables and resets
│   ├── App.vue                    # Root component
│   └── main.js                    # Vue app entry point
├── src-tauri/                     # Rust + Tauri backend
│   ├── src/
│   │   ├── lib.rs                 # FS commands and Tauri setup
│   │   │   ├── get_default_root()
│   │   │   ├── read_directory_tree()
│   │   │   ├── read_text_file()
│   │   │   └── save_file()
│   │   └── main.rs                # Entry point
│   ├── Cargo.toml                 # Rust dependencies
│   ├── tauri.conf.json            # App config (name, window, bundle)
│   ├── capabilities/              # Tauri permissions (WIP)
│   └── icons/                     # Platform-specific app icons
├── vite.config.js                 # Vite bundler configuration
├── package.json                   # npm scripts and dependencies
└── README.md                       # This file
```

## Roadmap

### Phase 1: Foundation (Current)
- [x] **Multi-tab Code Editor**: Create, open, close, switch files
- [x] **File System Integration**: Read/write with Rust backend
- [x] **Dynamic UI Layout**: Resizable panels, exclusive left sidebar tabs
- [x] **Bottom Panel System**: Toggle Terminal, Git, Todo
- [ ] **Undo/Redo**: Infrastructure ready, keyboard shortcuts needed

### Phase 2: Core IDE Features
- [ ] **Real Terminal**: Integrate Xterm.js for interactive shell (`npm install xterm`)
- [ ] **Syntax Highlighting**: Monaco Editor or Prism.js for code coloring
- [ ] **File Operations**: Rename, delete, create files/folders from UI
- [ ] **Search & Replace**: Basic find with regex support
- [ ] **Settings Panel**: Configurable themes, font sizes, keybindings

### Phase 3: Advanced Features
- [ ] **Advanced Git**: Stage/Commit/Push UI, interactive history, blame view
- [ ] **Pull Requests**: GitHub/GitLab integration for PR browsing and review
- [ ] **Code Completion**: Language Server Protocol (LSP) integration
- [ ] **Debug Console**: Breakpoints, step-through debugging
- [ ] **Global Command Palette**: Ctrl+Shift+P / Cmd+Shift+P for quick actions

### Phase 4: Ecosystem
- [ ] **Plugin System**: Modular architecture for extensions
- [ ] **Theme Engine**: Share and install community themes
- [ ] **Keybinding Profiles**: Vim, Emacs, VS Code preset support
- [ ] **Workspace Snapshots**: Save/restore editor state

## Status

**Phase**: Active Prototyping (Foundation complete)

The project has a **working, functional prototype** with:
- ✅ Full IDE interface layout (header, sidebars, code area, bottom panels)
- ✅ File editing with multi-tab support
- ✅ Rust-Vue bridge for filesystem operations
- ✅ Responsive drag-to-resize panels
- ✅ Modular component architecture

**Focus Areas Right Now**:
1. **Placeholder Features**: Terminal and Git panels are UI stubs awaiting backend integration
2. **Editor Power**: Adding syntax highlighting and code completion
3. **Performance**: Optimizing directory tree rendering for large projects

**Next Steps**:
- Integrate Xterm.js for real terminal support
- Add Monaco Editor or CodeMirror for syntax highlighting
- Implement git command integration (stage, commit, push)
- Polish the UI with transitions and animations

---

## Contributing

This is an experimental prototype to explore IDE design and Tauri capabilities.

If you're interested in:
- **Desktop UI/UX design** at scale
- **Rust performance engineering**
- **Tight Vue + Rust integration**
- **Building the next-generation IDE**

…feel free to engage with ideas, PRs, or just watch how this evolves.

---

## License

(Currently not specified — open source friendly)

---

## Acknowledgments

Inspired by:
- **JetBrains** products for UI/UX excellence
- **VS Code** for the web-based editor paradigm
- **Tauri** for making lightweight desktop apps possible
