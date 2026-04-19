<script setup>
import { computed, ref, watch } from 'vue';
import '../styles/Header.css';

const emit = defineEmits(['create-file', 'import-files']);

const isOpen = ref(false);
const isSubOpen = ref(false);
const isCreateModalOpen = ref(false);

const fileInput = ref(null);
const folderInput = ref(null);
const fileName = ref('');

const selectedExtension = ref('.txt');
const selectedVariant = ref(null);
const currentType = ref(null);

const currentExtensions = ref([]);
const currentVariants = ref([]);

const fileTemplates = {
  rust: {
    allowSeparateType: false,
    variants: {
      binary: {
        label: 'Binary',
        extensions: ['.rs'],
        template:
            `fn main() {
    println!("Hello Rust");
}
`,
      },
    },
  },
  python: {
    allowSeparateType: false,
    variants:{
      file:{
        label: 'Python File',
        extensions: ['.py'],
        template:
            `print("Hello, Python!")`,
      },
      unitTest:{
        label: 'Python unit test',
        extensions: ['.py'],
        template: ``,
      }
      }
  },
  lua: {},
  gamma: {},
  html: {},
  css: {},
  js: {},
  jsx: {},
  ts: {},
  tsx: {},
  vue: {},
  ccpp: {
    allowSeparateType: false,
    variants:{
      c: {
        label: 'C File',
        extensions: ['.c'],
        template: ``,
      },

      cpp: {
        label: 'C++ File',
        extensions: ['.cpp'],
        template: ``,
      },

      h: {
        label: 'H File',
        extensions: ['.h'],
        template: ``,
      },

      hpp: {
        label: 'H++ File',
        extensions: ['.hpp'],
        template: ``,
      },
    }
  },
  asm: {
    allowSeparateType: true,
    variants:{
      nasm:{
        label: 'NASM',
        template: `;nasm`,
      },

      fasm:{
        label: 'FASM',
        template: `;fasm`,
      },

      tasm:{
        label: 'TASM',
        template: `;tasm`,
      },

      rasm:{
        label: 'RASM',
        template: `;rasm`,
      },
    }
  },
}

const fileTypes = {
  txt: {
    label: 'Text File',
    extensions: ['.txt'],
  },
  md: {
    label: 'Markdown',
    extensions: ['.md'],
  },
  rust: {
    label: 'Rust File',
    extensions: ['.rs'],
  },
  python: {
    label: 'Python File',
    extensions: ['.py'],
  },
  lua: {
    label: 'Lua File',
    extensions: ['.lua'],
  },
  gamma: {
    label: 'Gamma File',
    extensions: ['.gamma', '.gma', '.gm'],
  },
  html: {
    label: 'HTML File',
    extensions: ['.html', '.htm'],
  },
  css:{
    label: 'CSS File',
    extensions: ['.css'],
  },
  js: {
    label: 'JavaScript File',
    extensions: ['.js', '.jsx'],
  },
  ts: {
    label: 'TypeScript File',
    extensions: ['.ts', '.tsx'],
  },
  ccpp: {
    label: 'C/C++ File',
    extensions: ['.c', '.cpp', '.h', '.hpp'],
  },
  asm: {
    label: 'Assembly File',
    extensions: ['.asm', '.assembly', '.as', '.s'],
  },
};

const effectiveExtensions = computed(() => {
  const type = currentType.value;

  if (!type) {
    return [];
  }

  const typeExtensions = currentExtensions.value;
  const templateGroup = fileTemplates[type];
  const variant = templateGroup?.variants?.[selectedVariant.value];
  const variantExtensions = Array.isArray(variant?.extensions) ? variant.extensions : [];

  if (variantExtensions.length && templateGroup?.allowSeparateType === false) {
    return variantExtensions;
  }

  return variantExtensions.length ? variantExtensions : typeExtensions;
});

const canChooseTypeSeparately = computed(() => {
  const type = currentType.value;
  const templateGroup = fileTemplates[type];

  if (!templateGroup?.variants) {
    return true;
  }

  if (templateGroup.allowSeparateType === true) {
    return true;
  }

  if (templateGroup.allowSeparateType === false) {
    return false;
  }

  return true;
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

function saveFile(){

}

function openCreateModal(type = 'txt') {
  const normalizedType = fileTypes[type] ? type : 'txt';
  const fileType = fileTypes[normalizedType];

  currentType.value = normalizedType;

  currentExtensions.value = [...fileType.extensions];
  selectedExtension.value = fileType.extensions[0];

  const templateGroup = fileTemplates[normalizedType];

  if (templateGroup?.variants) {
    currentVariants.value = Object.entries(templateGroup.variants);
    selectedVariant.value = currentVariants.value[0][0];
  } else {
    currentVariants.value = [];
    selectedVariant.value = null;
  }

  fileName.value = '';
  isOpen.value = false;
  isSubOpen.value = false;
  isCreateModalOpen.value = true;
  syncSelectedExtension();
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
</script>

<template>
  <div class="dropdown">
    <button type="button" class="dropdown-toggle" @click="toggle">File</button>

    <ul v-if="isOpen" class="dropdown-menu">
      <li><button @click="openFile">Open file</button></li>
      <li><button @click="openFolder">Open folder</button></li>
      <li><button @click="saveFile">Save</button></li>
      <li><button @click="saveFile">Save As</button></li>
      <li
          class="has-submenu"
          @mouseenter="isSubOpen = true"
          @mouseleave="isSubOpen = false"
      >
        <button type="button">New</button>

        <ul v-if="isSubOpen" class="dropdown-submenu">
          <li><button @click="openCreateModal('txt')">File</button></li>
          <li><button @click="openCreateModal('rust')">Rust File</button></li>
          <li><button @click="openCreateModal('python')">Python File</button></li>
          <li><button @click="openCreateModal('html')">HTML File</button></li>
          <li><button @click="openCreateModal('css')">CSS File</button></li>
          <li><button @click="openCreateModal('js')">JavaScript File</button></li>
          <li><button @click="openCreateModal('ts')">TypeScript File</button></li>
          <li><button @click="openCreateModal('gamma')">Gamma File</button></li>
          <li><button @click="openCreateModal('ccpp')">C/C++ File</button></li>
          <li><button @click="openCreateModal('md')">Markdown File</button></li>
          <li><button @click="openCreateModal('asm')">Asm File</button></li>
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
        <select v-if="canChooseTypeSeparately" id="file-extension" v-model="selectedExtension">
          <option
              v-for="ext in effectiveExtensions"
              :key="ext"
              :value="ext"
          >
            {{ ext }}
          </option>
        </select>
        <input v-else id="file-extension" :value="selectedExtension" type="text" readonly />
        <div v-if="currentVariants.length">
          <label>Template</label>

          <select v-model="selectedVariant">
            <option
                v-for="([key, variant]) in currentVariants"
                :key="key"
                :value="key"
            >
              {{ variant.label }}
            </option>
          </select>
        </div>

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