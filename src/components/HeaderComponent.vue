<script setup>
import { computed, ref, watch } from 'vue';
import MenuDropdown from './MenuDropdown.vue';
import fileTemplates from './fileTemplates.vue';
import fileTypes from './fileTypes.vue';
import '../styles/Header.css'

const emit = defineEmits([
  'create-file',
  'create-folder',
  'import-files',
  'toggle-sidebar',
  'zoom-in',
  'zoom-out',
  'update-content',
  'paste-content'
]);

const props = defineProps({
  activeTab: {
    type: Object,
    default: null,
  },
});

const undoStack = ref([]);
const redoStack = ref([]);
const lastSavedContent = ref('');

watch(() => props.activeTab?.content, (newContent) => {
  if (props.activeTab && newContent !== lastSavedContent.value) {
    if (lastSavedContent.value !== '') {
      undoStack.value.push(lastSavedContent.value);
      redoStack.value = [];
    }
    lastSavedContent.value = newContent;
  }
}, { deep: true });


const activeMenu = ref(null);
const activeSubmenu = ref(null);
const isSubOpen = ref(false);

function toggleMenu(menu) {
  activeMenu.value = activeMenu.value === menu ? null : menu;
}

function openSubmenu(name) {
  activeSubmenu.value = name;
}

function closeSubmenu() {
  activeSubmenu.value = null;
}

const isCreateModalOpen = ref(false);
const isCreateFolderModalOpen = ref(false);
const folderName = ref('');


const fileInput = ref(null);
const folderInput = ref(null);

const folderInputAttrs = {
  webkitdirectory: true,
  directory: true,
};


const fileName = ref('');
const selectedExtension = ref('.txt');
const selectedVariant = ref(null);
const currentType = ref(null);

const currentExtensions = ref([]);
const currentVariants = ref([]);



const fileTypeEntries = Object.entries(fileTypes);

const effectiveExtensions = computed(() => {
  const type = currentType.value;

  if (!type) {
    return [];
  }

  const typeExtensions = currentExtensions.value;
  const templateGroup = fileTemplates[type];
  const variant = templateGroup?.variants?.[selectedVariant.value];
  const variantExtensions = Array.isArray(variant?.extensions)
      ? variant.extensions
      : [];

  if (variantExtensions.length && templateGroup?.allowSeparateType === false) {
    return variantExtensions;
  }

  return variantExtensions.length ? variantExtensions : typeExtensions;
});

const canChooseTypeSeparately = computed(() => {
  const type = currentType.value;
  if (!type) return false;
  return fileTemplates[type]?.allowSeparateType ?? false;
});

watch([selectedVariant, currentType], () => {
  syncSelectedExtension();
});

  function syncSelectedExtension() {
    const extensions = effectiveExtensions.value;

    if (!extensions.length) {
      selectedExtension.value = '';
      return;
    }

    if (!canChooseTypeSeparately.value || !extensions.includes(selectedExtension.value)) {
      selectedExtension.value = extensions[0];
    }
  }

  watch([selectedVariant, currentType], () => {
    syncSelectedExtension();
  });

function refreshTypeState(type) {
  const normalizedType = fileTypes[type] ? type : 'txt';
  const fileType = fileTypes[normalizedType];
  const templateGroup = fileTemplates[normalizedType];

  currentExtensions.value = [...fileType.extensions];
  selectedExtension.value = fileType.extensions[0] || '';

  if (templateGroup?.variants) {
    currentVariants.value = Object.entries(templateGroup.variants);
    selectedVariant.value = currentVariants.value[0]?.[0] ?? null;
  } else {
    currentVariants.value = [];
    selectedVariant.value = null;
  }

  syncSelectedExtension();
}

function toggleSidebar() {
  emit('toggle-sidebar');
  activeMenu.value = null;
}

function zoomIn() {
  const root = document.documentElement;
  const currentZoom = parseFloat(root.style.fontSize) || 16;
  root.style.fontSize = (currentZoom + 2) + 'px';
  activeMenu.value = null;
}

function zoomOut() {
  const root = document.documentElement;
  const currentZoom = parseFloat(root.style.fontSize) || 16;
  if (currentZoom > 10) {
    root.style.fontSize = (currentZoom - 2) + 'px';
  }
  activeMenu.value = null;
}

async function copyToClipboard() {
  if (!props.activeTab) {
    alert('No file selected');
    return;
  }

  try {
    await navigator.clipboard.writeText(props.activeTab.content);
    console.log('Content copied to clipboard');
    activeMenu.value = null;
  } catch (error) {
    console.error('Failed to copy:', error);
    alert('Failed to copy to clipboard');
  }
}

async function pasteFromClipboard() {
  if (!props.activeTab) {
    alert('No file selected');
    return;
  }

  try {
    const text = await navigator.clipboard.readText();
    const newContent = props.activeTab.content + text;
    emit('update-content', { id: props.activeTab.id, content: newContent });
    activeMenu.value = null;
  } catch (error) {
    console.error('Failed to paste:', error);
    alert('Failed to paste from clipboard');
  }
}


function openFile() {
  fileInput.value.click();
  activeMenu.value = null;
}

function openFolder() {
  folderInput.value.click();
  activeMenu.value = null;
}

async function saveFile() {
  if (!props.activeTab) {
    alert('No file selected');
    return;
  }

  const fileName = props.activeTab.fullName;
  const content = props.activeTab.content;

  if (!fileName || fileName.startsWith('untitled')) {
    alert('Please save file with a specific path first using export or create from existing folder');
    return;
  }

  try {
    if (window.__TAURI__) {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('save_file', {
        filePath: fileName,
        content: content,
      });
      console.log('File saved via Tauri:', fileName);
      alert('File saved successfully!');
    } else {
      const blob = new Blob([content], { type: 'text/plain' });
      const url = URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = fileName;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      URL.revokeObjectURL(url);

      console.log('File downloaded:', fileName);
      alert('File downloaded successfully!');
    }
  } catch (error) {
    console.error('Failed to save file:', error);
    alert('Failed to save file: ' + error);
  }
}

function openCreateModal(type = 'txt') {
  currentType.value = fileTypes[type] ? type : 'txt';
  refreshTypeState(currentType.value);

  fileName.value = '';

  activeMenu.value = null;
  activeSubmenu.value = null;

  isCreateModalOpen.value = true;
}

function closeCreateModal() {
  isCreateModalOpen.value = false;
}

function submitCreateFile() {
  const name = fileName.value.trim();

  if (!name) return;

  let content = '';

  const type = currentType.value;
  const templateGroup = fileTemplates[type];

  if (templateGroup?.variants && selectedVariant.value) {
    content = templateGroup.variants[selectedVariant.value].template || '';
  }

  const extension = selectedExtension.value || effectiveExtensions.value[0] || '';
  const hasSameExtension = extension && name.toLowerCase().endsWith(extension.toLowerCase());
  const baseName = hasSameExtension ? name.slice(0, -extension.length) : name;

  emit('create-file', {
    baseName,
    extension,
    content,
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

async function buildProject(event){

}

async function rebuildProject(event){

}

</script>

<template>
  <div class="menu-bar">

    <MenuDropdown
        label="File"
        :activeMenu="activeMenu"
        @toggle="toggleMenu"
    >
      <li><button @click="openFile">Open file</button></li>
      <li><button @click="openFolder">Open folder</button></li>
      <li><button @click="saveFile">Save</button></li>

      <li
          class="has-submenu"
          @mouseenter="isSubOpen = true"
          @mouseleave="isSubOpen = false"
      >
        <button>New</button>

        <ul v-if="isSubOpen" class="dropdown-submenu">
          <li v-for="(type, key) in fileTypes" :key="key">
            <button @click="openCreateModal(key)">{{ type.label }}</button>
          </li>
        </ul>
      </li>
    </MenuDropdown>

    <MenuDropdown
        label="Edit"
        :activeMenu="activeMenu"
        @toggle="toggleMenu"
    >
      <li><button @click="undo">Undo</button></li>
      <li><button @click="redo">Redo</button></li>
      <li><button @click="copyToClipboard">Copy</button></li>
      <li><button @click="pasteFromClipboard">Paste</button></li>
    </MenuDropdown>

    <MenuDropdown
        label="View"
        :activeMenu="activeMenu"
        @toggle="toggleMenu"
    >
      <li><button @click="toggleSidebar">Toggle Sidebar</button></li>
      <li><button @click="zoomIn">Zoom In</button></li>
      <li><button @click="zoomOut">Zoom Out</button></li>
    </MenuDropdown>

    <MenuDropdown
        label="Navigate"
        :activeMenu="activeMenu"
        @toggle="toggleMenu"
    >
      <li><button>Go to File...</button></li>
      <li><button>Go to Class...</button></li>
      <li><button>Go to Symbol...</button></li>
      <li><button>Go to Line...</button></li>
      <li><button>Recent Files</button></li>
      <li><button>Recent Locations</button></li>
      <li><button>Last Edit Location</button></li>
      <li><button>Next Error</button></li>
    </MenuDropdown>

    <MenuDropdown
        label="Code"
        :activeMenu="activeMenu"
        @toggle="toggleMenu"
    >

      <li><button>Comment with Line Comment</button></li>
      <li><button>Comment with Block Comment</button></li>
      <li><button>Reformat Code</button></li>
      <li><button>Optimize Imports</button></li>
      <li><button>Generate...</button></li>
      <li><button>Override Methods...</button></li>
      <li><button>Implement Methods...</button></li>
      <li><button>Analyze Stack Trace or Log...</button></li>
    </MenuDropdown>

    <MenuDropdown
        label="Refactor"
        :activeMenu="activeMenu"
        @toggle="toggleMenu"
    >

      <li><button>Rename...</button></li>
      <li><button>Extract Method...</button></li>
      <li><button>Extract Variable...</button></li>
      <li><button>Extract Constant...</button></li>
      <li><button>Inline...</button></li>
      <li><button>Change Signature...</button></li>
      <li><button>Move...</button></li>
      <li><button>Copy...</button></li>
    </MenuDropdown>

    <menu-dropdown
        label="Build"
        :activeMenu="activeMenu"
        @toggle="toggleMenu"
    >
      <li><button @click="buildProject">Build Project</button></li>
      <li><button @click="rebuildProject">Rebuild Project</button></li>
    </menu-dropdown>

    <menu-dropdown
        label="Run"
        :activeMenu="activeMenu"
        @toggle="toggleMenu"
    >
      <li><button>Run</button></li>
      <li><button>Debug</button></li>
      <li><button>Attach To Process</button></li>
    </menu-dropdown>

    <menu-dropdown
        label="Git"
        :activeMenu="activeMenu"
        @toggle="toggleMenu"
    >
      <li><button>Commit</button></li>
      <li><button>Push</button></li>
      <li><button>Update Project</button></li>
      <li><button>Pull</button></li>
      <li><button>Fetch</button></li>
      <li><button>Merge</button></li>
      <li><button>Rebase</button></li>
      <li><button>Branches...</button></li>
      <li><button>New Branch</button></li>
      <li><button>New Tag</button></li>
      <li><button>Reset HEAD</button></li>
      <li><button>New Worktree</button></li>
      <li><button>Worktrees...</button></li>
      <li><button>Show Git Log</button></li>
    </menu-dropdown>

    <menu-dropdown
        label="Help"
        :activeMenu="activeMenu"
        @toggle="toggleMenu"
    >
      <li><button>Find Action...</button></li>
      <li><button>Help</button></li>
      <li><button>Tip Of The Day</button></li>
      <li><button>My Productivity</button></li>
      <li><button>What`s New in G-IDE</button></li>
      <li><button>Getting Started</button></li>
      <li><button>Keyboard Shortcuts PDF</button></li>
      <li><button>Contact Support</button></li>
    </menu-dropdown>

  </div>

  <div v-if="isCreateModalOpen" class="modal-overlay" @click.self="closeCreateModal">
    <div class="create-modal">
      <h3>Create New File</h3>

      <div class="form-group">
        <label>File Name:</label>
        <input v-model="fileName" placeholder="main" />
      </div>

      <div v-if="currentVariants.length" class="form-group">
        <label>Template:</label>
        <select v-model="selectedVariant">
          <option v-for="[variantKey, variantData] in currentVariants" :key="variantKey" :value="variantKey">
            {{ variantData.label }}
          </option>
        </select>
      </div>

      <div v-if="canChooseTypeSeparately" class="form-group">
        <label>Extension:</label>
        <select v-model="selectedExtension">
          <option v-for="ext in effectiveExtensions" :key="ext" :value="ext">
            {{ ext }}
          </option>
        </select>
      </div>

      <div class="modal-buttons">
        <button @click="submitCreateFile" class="btn-primary">Create</button>
        <button @click="closeCreateModal" class="btn-secondary">Cancel</button>
      </div>
    </div>
  </div>

  <input type="file" ref="fileInput" style="display:none" @change="handleFile" />
  <input type="file" ref="folderInput" style="display:none" v-bind="folderInputAttrs" @change="handleFolder" />
</template>

<style scoped>

</style>