<script setup>
import { ref } from 'vue';
import '../styles/Header.css';

const emit = defineEmits(['create-file', 'import-files']);

const isOpen = ref(false);
const isSubOpen = ref(false);
const isCreateModalOpen = ref(false);

const fileInput = ref(null);
const folderInput = ref(null);
const fileName = ref('');
const selectedExtension = ref('.txt');

const extensionOptions = [
  '.txt',
  '.rs',
  '.py',
  '.html',
  '.css',
  '.js',
  '.ts',
  '.gamma',
  '.c',
  '.cpp',
  '.h',
  '.hpp',
  '.md',
  '.asm',
];

function toggle() {
  isOpen.value = !isOpen.value;
}

function openFile() {
  fileInput.value.click();
  isOpen.value = false;
}

function openFolder() {
  folderInput.value.click();
  isOpen.value = false;
}

function openCreateModal(extension = '.txt') {
  selectedExtension.value = extension;
  fileName.value = '';
  isCreateModalOpen.value = true;
  isOpen.value = false;
  isSubOpen.value = false;
}

function closeCreateModal() {
  isCreateModalOpen.value = false;
}

function submitCreateFile() {
  const normalizedName = fileName.value.trim();

  if (!normalizedName) {
    return;
  }

  emit('create-file', {
    name: normalizedName,
    extension: selectedExtension.value,
  });

  closeCreateModal();
}

async function handleFile(event) {
  const files = event.target.files;

  if (!files?.length) {
    return;
  }

  const importedFiles = await Promise.all(
    Array.from(files).map(async (file) => ({
      fullName: file.name,
      content: await file.text(),
    })),
  );

  emit('import-files', importedFiles);
  event.target.value = '';
}

async function handleFolder(event) {
  const files = event.target.files;

  if (!files?.length) {
    return;
  }

  const importedFiles = await Promise.all(
    Array.from(files)
      .filter((file) => file.type.startsWith('text/') || !file.type)
      .map(async (file) => ({
        fullName: file.webkitRelativePath || file.name,
        content: await file.text(),
      })),
  );

  emit('import-files', importedFiles);
  event.target.value = '';
}
</script>

<template>
  <div class="dropdown">
    <button type="button" class="dropdown-toggle" @click="toggle">File</button>

    <ul v-if="isOpen" class="dropdown-menu">
      <li><button @click="openFile">Open file</button></li>
      <li><button @click="openFolder">Open folder</button></li>

      <li
          class="has-submenu"
          @mouseenter="isSubOpen = true"
          @mouseleave="isSubOpen = false"
      >
        <button type="button">New</button>

        <ul v-if="isSubOpen" class="dropdown-submenu">
          <li><button @click="openCreateModal('.txt')">File</button></li>
          <li><button @click="openCreateModal('.rs')">Rust File</button></li>
          <li><button @click="openCreateModal('.py')">Python File</button></li>
          <li><button @click="openCreateModal('.html')">HTML File</button></li>
          <li><button @click="openCreateModal('.css')">CSS File</button></li>
          <li><button @click="openCreateModal('.js')">JavaScript File</button></li>
          <li><button @click="openCreateModal('.ts')">TypeScript File</button></li>
          <li><button @click="openCreateModal('.gamma')">Gamma File</button></li>
          <li><button @click="openCreateModal('.c')">C File</button></li>
          <li><button @click="openCreateModal('.cpp')">C++ File</button></li>
          <li><button @click="openCreateModal('.h')">Header File</button></li>
          <li><button @click="openCreateModal('.md')">Markdown File</button></li>
          <li><button @click="openCreateModal('.asm')">Asm File</button></li>
        </ul>
      </li>
    </ul>

    <div v-if="isCreateModalOpen" class="modal-overlay" @click.self="closeCreateModal">
      <div class="create-modal">
        <h3>Create New File</h3>
        <label for="file-name">File name</label>
        <input
            id="file-name"
            v-model="fileName"
            type="text"
            placeholder="main"
            @keydown.enter.prevent="submitCreateFile"
        />

        <label for="file-extension">Type</label>
        <select id="file-extension" v-model="selectedExtension">
          <option v-for="extension in extensionOptions" :key="extension" :value="extension">
            {{ extension }}
          </option>
        </select>

        <div class="modal-actions">
          <button type="button" class="ghost-button" @click="closeCreateModal">Cancel</button>
          <button type="button" class="primary-button" @click="submitCreateFile">Create</button>
        </div>
      </div>
    </div>

    <input
        type="file"
        ref="fileInput"
        multiple
        style="display:none"
        @change="handleFile"
    />

    <input
        type="file"
        ref="folderInput"
        webkitdirectory
        directory
        style="display:none"
        @change="handleFolder"
    />
  </div>
</template>

<style scoped>

</style>