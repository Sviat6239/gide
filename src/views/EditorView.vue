<script setup>
import { computed, onBeforeUnmount, ref } from 'vue';
import CodeAreaComponent from '../components/CodeAreaComponent.vue';
import TerminalComponent from '../components/TerminalComponent.vue';
import GitComponent from '../components/GitComponent.vue';
import HeaderComponent from '../components/HeaderComponent.vue';
import LeftSideBarComponent from '../components/LeftSideBarComponent.vue';
import RightSideBarComponent from '../components/RightSideBarComponent.vue';
import FooterComponent from '../components/FooterComponent.vue';

const tabs = ref([]);
const activeTabId = ref(null);
let nextTabId = 1;
const leftPaneWidth = ref(260);
const terminalHeight = ref(240);

const isFileTreeOpen = ref(true);
const isTerminalOpen = ref(false);
const isGitOpen = ref(false);
const isCommitOpen = ref(false);
const isPullRequestsOpen = ref(false);

const leftToolbarWidth = 44;
const rightPaneWidth = 40;

const isAnyLeftPanelOpen = computed(() =>
  isFileTreeOpen.value || isCommitOpen.value || isPullRequestsOpen.value
);

const isAnyBottomPanelOpen = computed(() =>
  isTerminalOpen.value || isGitOpen.value
);

const effectiveLeftPaneWidth = computed(() => (isAnyLeftPanelOpen.value ? leftPaneWidth.value : leftToolbarWidth));

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

  const normalizedPath = String(payload.fullName).replace(/\\/g, '/');
  const existingTab = tabs.value.find((tab) => String(tab.fullName).replace(/\\/g, '/') === normalizedPath);

  if (existingTab) {
    activeTabId.value = existingTab.id;
    return;
  }

  if (payload.content !== undefined) {
    openOrActivateTab(payload.fullName, payload.content ?? '');
  }
}

function closeAllLeftPanels() {
  isFileTreeOpen.value = false;
  isCommitOpen.value = false;
  isPullRequestsOpen.value = false;
}

function toggleFileTree() {
  const target = !isFileTreeOpen.value;
  closeAllLeftPanels();
  isFileTreeOpen.value = target;
}

function toggleCommit() {
  const target = !isCommitOpen.value;
  closeAllLeftPanels();
  isCommitOpen.value = target;
}

function togglePullRequests() {
  const target = !isPullRequestsOpen.value;
  closeAllLeftPanels();
  isPullRequestsOpen.value = target;
}

function toggleTerminal() {
  const target = !isTerminalOpen.value;
  isGitOpen.value = false;
  isTerminalOpen.value = target;
}

function toggleGit() {
  const target = !isGitOpen.value;
  isTerminalOpen.value = false;
  isGitOpen.value = target;
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
  const layoutBounds = event.currentTarget.closest('.content-row')?.getBoundingClientRect();

  if (!layoutBounds || !isAnyLeftPanelOpen.value) {
    return;
  }

  setupPointerResize((moveEvent) => {
    const width = moveEvent.clientX - layoutBounds.left;
    leftPaneWidth.value = Math.min(460, Math.max(140, width));
  });
}

function startTerminalResize() {
  const layoutElement = document.querySelector('.main-layout');

  if (!layoutElement) {
    return;
  }

  const layoutBounds = layoutElement.getBoundingClientRect();

  setupPointerResize((moveEvent) => {
    const rawHeight = layoutBounds.bottom - moveEvent.clientY;
    const maxHeight = Math.max(120, layoutBounds.height - 120 - 4);
    terminalHeight.value = Math.min(maxHeight, Math.max(120, rawHeight));
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
      <HeaderComponent
          @create-file="handleCreateFile"
          @import-files="handleImportFiles"
          :active-tab="tabs.find(tab => tab.id === activeTabId) || null"
      />
    </div>

    <div class="main-layout">
      <div class="content-row">

        <!-- LEFT -->
        <aside class="left-sidebar" :style="{ width: `${effectiveLeftPaneWidth}px` }">
          <LeftSideBarComponent
              :files="tabs"
              :is-file-tree-open="isFileTreeOpen"
              :is-terminal-open="isTerminalOpen"
              :is-git-open="isGitOpen"
              :is-commit-open="isCommitOpen"
              :is-pull-requests-open="isPullRequestsOpen"
              @toggle-file-tree="toggleFileTree"
              @toggle-terminal="toggleTerminal"
              @toggle-git="toggleGit"
              @toggle-commit="toggleCommit"
              @toggle-pull-requests="togglePullRequests"
              @open-file="handleOpenFileFromTree"
          />
        </aside>

        <div
            v-if="isAnyLeftPanelOpen"
            class="resize-handle vertical"
            @pointerdown.prevent="startLeftResize"
        ></div>

        <section class="workspace-area">
          <div class="editor-view">
            <CodeAreaComponent
                :tabs="tabs"
                :active-tab-id="activeTabId"
                @select-tab="selectTab"
                @close-tab="closeTab"
                @update-content="updateTabContent"
            />
          </div>

          <div
              v-if="isAnyBottomPanelOpen"
              class="resize-handle horizontal"
              @pointerdown.prevent="startTerminalResize"
          ></div>

          <div
              v-if="isAnyBottomPanelOpen"
              class="bottom-view"
              :style="{ height: `${terminalHeight}px` }"
          >
            <TerminalComponent v-if="isTerminalOpen" @close="toggleTerminal" />
            <GitComponent v-if="isGitOpen" @close="toggleGit" />
          </div>
        </section>

        <aside class="right-sidebar" :style="{ width: `${rightPaneWidth}px` }">
          <RightSideBarComponent />
        </aside>

      </div>
    </div>

    <footer class="footer">
      <FooterComponent />
    </footer>
  </div>
</template>

<style scoped>
.editor-shell {
  --footer-height: 38px;
  height: 100dvh;
  max-height: 100dvh;
  display: flex;
  flex-direction: column;
  padding-bottom: var(--footer-height);
  box-sizing: border-box;
  overflow: hidden;
}

.header {
  position: relative;
  z-index: 100;
}

.main-layout {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.content-row {
  flex: 1;
  min-height: 0;
  display: flex;
  overflow: hidden;
}

.workspace-area {
  flex: 1;
  min-width: 100px;
  min-height: 100px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.left-sidebar,
.right-sidebar {
  flex: 0 0 auto;
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
  min-width: 0;
  min-height: 0;
  overflow: hidden;
}

.editor-view {
  flex: 1;
  display: flex;
  min-height: 0;
  overflow: hidden;
}

.bottom-view {
  flex: 0 0 auto;
  min-height: 0;
  display: flex;
  overflow: hidden;
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