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
const folderInputAttrs = {
  webkitdirectory: true,
  directory: true,
};

const selectedExtension = ref('.txt');
const selectedVariant = ref(null);
const currentType = ref(null);

const currentExtensions = ref([]);
const currentVariants = ref([]);

const fileTemplates = {
  txt: {
    allowSeparateType: false,
    variants: {
      plain: {
        label: 'Plain text',
        extensions: ['.txt'],
        template: '',
      },
    },
  },
  md: {
    allowSeparateType: false,
    variants: {
      article: {
        label: 'Markdown',
        extensions: ['.md'],
        template: `# Title

Write your text here.
`,
      },
    },
  },
  rust: {
    allowSeparateType: false,
    variants: {
      binary: {
        label: 'Binary',
        extensions: ['.rs'],
        template:
            `fn main() {
    println!("Hello, Rust!");
}
`,
      },
    },
  },
  python: {
    allowSeparateType: false,
    variants: {
      file: {
        label: 'Python File',
        extensions: ['.py'],
        template: `def main():
    print("Hello, Python!")


if __name__ == "__main__":
    main()
`,
      },
      unitTest: {
        label: 'Python unit test',
        extensions: ['.py'],
        template: `import unittest


class TestExample(unittest.TestCase):
    def test_something(self):
        self.assertTrue(True)


if __name__ == "__main__":
    unittest.main()
`,
      },
    },
  },
  lua: {
    allowSeparateType: false,
    variants: {
      script: {
        label: 'Lua script',
        extensions: ['.lua'],
        template: `print("Hello, Lua!")
`,
      },
    },
  },
  gamma: {
    allowSeparateType: true,
    variants: {
      script: {
        label: 'Gamma script',
        extensions: ['.gamma', '.gma', '.gm'],
        template: `# Gamma file
`,
      },
    },
  },
  html: {
    allowSeparateType: false,
    variants: {
      page: {
        label: 'HTML page',
        extensions: ['.html'],
        template: `<!doctype html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Document</title>
</head>
<body>

</body>
</html>
`,
      },
    },
  },
  css: {
    allowSeparateType: false,
    variants: {
      stylesheet: {
        label: 'CSS stylesheet',
        extensions: ['.css'],
        template: `:root {
  --primary: #4f46e5;
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  font-family: sans-serif;
}
`,
      },
    },
  },
  js: {
    allowSeparateType: false,
    variants: {
      script: {
        label: 'JavaScript',
        extensions: ['.js', '.jsx'],
        template: `console.log("Hello, JavaScript!");
`,
      },
      component: {
        label: 'React JSX',
        extensions: ['.jsx'],
        template: `export default function App() {
  return <div>Hello, React</div>;
}
`,
    },
  },
  },
  ts: {
    allowSeparateType: false,
    variants: {
      script: {
        label: 'TypeScript',
        extensions: ['.ts', '.tsx'],
        template: `const message: string = "Hello, TypeScript!";
console.log(message);
`,
      },

      component: {
        label: 'React TSX',
        extensions: ['.tsx'],
        template: `export default function App(): JSX.Element {
  return <div>Hello, TypeScript React</div>;
}
`,
      },
    },
  },
  vue: {
    allowSeparateType: false,
    variants: {

    },
  },
  ccpp: {
    allowSeparateType: false,
    variants: {
      c: {
        label: 'C File',
        extensions: ['.c'],
        template: `#include <stdio.h>

int main(void) {
    printf("Hello, C!\\n");
    return 0;
}
`,
      },

      cpp: {
        label: 'C++ File',
        extensions: ['.cpp'],
        template: `#include <iostream>

int main() {
    std::cout << "Hello, C++!" << std::endl;
    return 0;
}
`,
      },

      h: {
        label: 'C Header',
        extensions: ['.h'],
        template: `#pragma once
`,
      },

      hpp: {
        label: 'C++ Header',
        extensions: ['.hpp'],
        template: `#pragma once
`,
      },
    }
  },
  asm: {
    allowSeparateType: true,
    variants: {
      nasm: {
        label: 'NASM',
        template: `; NASM template
`,
      },

      fasm: {
        label: 'FASM',
        template: `; FASM template
`,
      },

      tasm: {
        label: 'TASM',
        template: `; TASM template
`,
      },

      rasm: {
        label: 'RASM',
        template: `; RASM template
`,
      },
    }
  },
};

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
  css: {
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
  vue: {
    label: 'Vue File',
    extensions: ['.vue'],
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

const fileTypeEntries = Object.entries(fileTypes);

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

  return !templateGroup?.variants || templateGroup.allowSeparateType !== false;
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
  currentType.value = fileTypes[type] ? type : 'txt';
  refreshTypeState(currentType.value);
  fileName.value = '';
  isOpen.value = false;
  isSubOpen.value = false;
  isCreateModalOpen.value = true;
}

function handleTypeChange(event) {
  refreshTypeState(event.target.value);
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
          <li v-for="([key, info]) in fileTypeEntries" :key="key">
            <button @click="openCreateModal(key)">{{ info.label }}</button>
          </li>
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

        <label for="file-extension">Extension</label>
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
        style="display:none"
        v-bind="folderInputAttrs"
        @change="handleFolder"
    />
  </div>
</template>

<style scoped>

</style>