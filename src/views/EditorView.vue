<script setup>
import { onBeforeUnmount, ref } from 'vue';
import CodeAreaComponent from '../components/CodeAreaComponent.vue';
import TerminalComponent from '../components/TerminalComponent.vue';
import HeaderComponent from '../components/HeaderComponent.vue';
import LeftSideBarComponent from '../components/LeftSideBarComponent.vue';
import RightSideBarComponent from '../components/RightSideBarComponent.vue';
import FooterComponent from '../components/FooterComponent.vue';

const tabs = ref([]);
const activeTabId = ref(null);
let nextTabId = 1;
const leftPaneWidth = ref(260);
const rightPaneWidth = ref(240);
const terminalHeight = ref(240);

let stopResize = null;

function splitFileName(fullName) {
  const slashIndex = Math.max(fullName.lastIndexOf('/'), fullName.lastIndexOf('\\'));
  const fileName = slashIndex >= 0 ? fullName.slice(slashIndex + 1) : fullName;
  const dotIndex = fileName.lastIndexOf('.');

  if (dotIndex <= 0) {
    return { name: fileName, extension: '' };
  }

  return {
    name: fileName.slice(0, dotIndex),
    extension: fileName.slice(dotIndex),
  };
}

function normalizeExtension(extension) {
  if (!extension) {
    return '';
  }

  return extension.startsWith('.') ? extension : `.${extension}`;
}

function makeUniqueName(baseName, extension) {
  const normalizedBase = baseName.trim() || 'untitled';
  const normalizedExtension = normalizeExtension(extension);
  const existingNames = new Set(tabs.value.map((tab) => tab.fullName));

  let suffix = 0;
  let candidate = `${normalizedBase}${normalizedExtension}`;

  while (existingNames.has(candidate)) {
    suffix += 1;
    candidate = `${normalizedBase} (${suffix})${normalizedExtension}`;
  }

  return candidate;
}

function createTab({ baseName, extension = '', content = '', fullName: explicitFullName = null }) {
  const fullName = explicitFullName ?? makeUniqueName(baseName, extension);
  const parts = splitFileName(fullName);

  const newTab = {
    id: nextTabId,
    fullName,
    name: parts.name,
    extension: parts.extension,
    content,
  };

  nextTabId += 1;
  tabs.value.push(newTab);
  activeTabId.value = newTab.id;
}

function openOrActivateTab(fullName, content) {
  const existingTab = tabs.value.find((tab) => tab.fullName === fullName);

  if (existingTab) {
    existingTab.content = content;
    activeTabId.value = existingTab.id;
    return;
  }

  const parts = splitFileName(fullName);
  createTab({
    baseName: parts.name,
    extension: parts.extension,
    content,
    fullName,
  });
}

function handleCreateFile(payload) {
  const rawName = (payload.baseName ?? payload.name ?? '').trim();

  if (!rawName) {
    return;
  }

  if (payload.extension) {
    createTab({
      baseName: rawName,
      extension: payload.extension,
      content: payload.content ?? '',
    });
    return;
  }

  const parts = splitFileName(rawName);
  createTab({
    baseName: parts.name || rawName,
    extension: parts.extension,
    content: payload.content ?? '',
  });
}

function handleImportFiles(importedFiles) {
  importedFiles.forEach((item) => {
    const parts = splitFileName(item.fullName);
    createTab({
      baseName: parts.name,
      extension: parts.extension,
      content: item.content,
    });
  });
}

function selectTab(tabId) {
  activeTabId.value = tabId;
}

function closeTab(tabId) {
  const index = tabs.value.findIndex((tab) => tab.id === tabId);

  if (index === -1) {
    return;
  }

  const isClosingActive = activeTabId.value === tabId;
  tabs.value.splice(index, 1);

  if (!tabs.value.length) {
    activeTabId.value = null;
    return;
  }

  if (!isClosingActive) {
    return;
  }

  const nextIndex = Math.min(index, tabs.value.length - 1);
  activeTabId.value = tabs.value[nextIndex].id;
}

function updateTabContent(payload) {
  const tab = tabs.value.find((item) => item.id === payload.id);

  if (!tab) {
    return;
  }

  tab.content = payload.content;
}

function handleOpenFileFromTree(payload) {
  if (!payload?.fullName) {
    return;
  }

  openOrActivateTab(payload.fullName, payload.content ?? '');
}

function setupPointerResize(onMove) {
  if (stopResize) {
    stopResize();
  }

  const handlePointerMove = (event) => onMove(event);
  const handlePointerUp = () => {
    window.removeEventListener('pointermove', handlePointerMove);
    window.removeEventListener('pointerup', handlePointerUp);
    stopResize = null;
  };

  window.addEventListener('pointermove', handlePointerMove);
  window.addEventListener('pointerup', handlePointerUp);

  stopResize = handlePointerUp;
}

function startLeftResize(event) {
  const shellBounds = event.currentTarget.closest('.editor-shell')?.getBoundingClientRect();

  if (!shellBounds) {
    return;
  }

  setupPointerResize((moveEvent) => {
    const width = moveEvent.clientX - shellBounds.left;
    leftPaneWidth.value = Math.min(460, Math.max(140, width));
  });
}

function startRightResize(event) {
  const shellBounds = event.currentTarget.closest('.editor-shell')?.getBoundingClientRect();

  if (!shellBounds) {
    return;
  }

  setupPointerResize((moveEvent) => {
    const width = shellBounds.right - moveEvent.clientX;
    rightPaneWidth.value = Math.min(460, Math.max(140, width));
  });
}

function startTerminalResize(event) {
  const contentBounds = event.currentTarget.closest('.center-content')?.getBoundingClientRect();

  if (!contentBounds) {
    return;
  }

  setupPointerResize((moveEvent) => {
    const height = contentBounds.bottom - moveEvent.clientY;
    terminalHeight.value = Math.min(420, Math.max(120, height));
  });
}

onBeforeUnmount(() => {
  if (stopResize) {
    stopResize();
  }
});

</script>

<template>
  <div class="editor-shell">
    <div class="header">
      <HeaderComponent @create-file="handleCreateFile" @import-files="handleImportFiles" />
    </div>

    <div class="main-layout">
      <aside class="left-sidebar" :style="{ width: `${leftPaneWidth}px` }">
        <LeftSideBarComponent @open-file="handleOpenFileFromTree" />
      </aside>

      <div class="resize-handle vertical" @pointerdown.prevent="startLeftResize"></div>

      <main class="center-content">
        <div class="editor-view" :style="{ minHeight: `calc(100% - ${terminalHeight}px - 8px)` }">
          <CodeAreaComponent
              :tabs="tabs"
              :active-tab-id="activeTabId"
              @select-tab="selectTab"
              @close-tab="closeTab"
              @update-content="updateTabContent"
          />
        </div>

        <div class="resize-handle horizontal" @pointerdown.prevent="startTerminalResize"></div>

        <div class="terminal-view" :style="{ height: `${terminalHeight}px` }">
          <TerminalComponent />
        </div>
      </main>

      <div class="resize-handle vertical" @pointerdown.prevent="startRightResize"></div>

      <aside class="right-sidebar" :style="{ width: `${rightPaneWidth}px` }">
        <RightSideBarComponent />
      </aside>
    </div>

    <footer class="footer">
      <FooterComponent />
    </footer>
  </div>
</template>

<style scoped>
.editor-shell {
  --footer-height: 44px;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  padding-bottom: var(--footer-height);
  box-sizing: border-box;
}

.header {
  position: relative;
  z-index: 100;
}

.main-layout {
  display: flex;
  flex: 1;
  min-height: 0;
}

.left-sidebar,
.right-sidebar {
  min-height: 0;
  overflow: hidden;
  border: 1px solid #3a4253;
  background-color: var(--app-bg-color);
}

.left-sidebar {
  border-right: none;
}

.right-sidebar {
  border-left: none;
}

.center-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
  min-height: 0;
}

.editor-view {
  min-height: 0;
  display: flex;
}

.terminal-view {
  min-height: 0;
  display: flex;
}

.resize-handle {
  flex: 0 0 4px;
  background-color: #3a4253;
  transition: background-color 0.15s;
}

.resize-handle:hover {
  background-color: #4d8dff;
}

.resize-handle.vertical {
  cursor: col-resize;
}

.resize-handle.horizontal {
  cursor: row-resize;
  width: 100%;
  flex-basis: 4px;
}

.footer {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  height: var(--footer-height);
  z-index: 120;
  border-top: 1px solid #3a4253;
  background-color: var(--app-bg-color);
}
</style>