<script setup>
import { ref } from 'vue';
import CodeAreaComponent from '../components/CodeAreaComponent.vue';
import TerminalComponent from '../components/TerminalComponent.vue';
import HeaderComponent from '../components/HeaderComponent.vue';

const tabs = ref([]);
const activeTabId = ref(null);
let nextTabId = 1;

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

function createTab({ baseName, extension = '', content = '' }) {
  const fullName = makeUniqueName(baseName, extension);
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

function handleCreateFile(payload) {
  createTab({
    baseName: payload.name,
    extension: payload.extension,
    content: '',
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

</script>

<template>

  <div class="header">
    <HeaderComponent @create-file="handleCreateFile" @import-files="handleImportFiles" />
  </div>

  <div class="editor-view">
    <CodeAreaComponent
        :tabs="tabs"
        :active-tab-id="activeTabId"
        @select-tab="selectTab"
        @close-tab="closeTab"
        @update-content="updateTabContent"
    />
  </div>

  <div class="terminal-view">
    <TerminalComponent/>
  </div>

</template>

<style scoped>
.header {
  position: relative;
  z-index: 100;
}

.editor-view {
  margin-bottom: 4px;
}

.terminal-view {
  margin-top: 4px;
}
</style>