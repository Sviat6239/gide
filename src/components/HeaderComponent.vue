<script setup>
import { computed, ref, watch } from 'vue';
import MenuDropdown from './MenuDropdown.vue';
import '../styles/Header.css'


const emit = defineEmits(['create-file', 'import-files']);

const activeMenu = ref(null);
const activeSubmenu = ref(null);

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
  const variantExtensions = Array.isArray(variant?.extensions)
      ? variant.extensions
      : [];

  if (variantExtensions.length && templateGroup?.allowSeparateType === false) {
    return variantExtensions;
  }

  return variantExtensions.length ? variantExtensions : typeExtensions;
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

function openFile() {
  fileInput.value.click();
  activeMenu.value = null;
}

function openFolder() {
  folderInput.value.click();
  activeMenu.value = null;
}

function saveFile() {
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
          <li><button @click="openCreateModal">Text File</button></li>
          <li><button @click="openCreateModal">Python File</button></li>
        </ul>
      </li>
    </MenuDropdown>

    <MenuDropdown
        label="Edit"
        :activeMenu="activeMenu"
        @toggle="toggleMenu"
    >
      <li><button>Undo</button></li>
      <li><button>Redo</button></li>
      <li><button>Copy</button></li>
      <li><button>Paste</button></li>
    </MenuDropdown>

    <MenuDropdown
        label="View"
        :activeMenu="activeMenu"
        @toggle="toggleMenu"
    >
      <li><button>Toggle Sidebar</button></li>
      <li><button>Zoom In</button></li>
      <li><button>Zoom Out</button></li>
    </MenuDropdown>

  </div>

  <div v-if="isCreateModalOpen" class="modal-overlay" @click.self="closeCreateModal">
    <div class="create-modal">
      <h3>Create New File</h3>
      <input v-model="fileName" placeholder="main" />
      <button @click="closeCreateModal">Close</button>
    </div>
  </div>

  <input type="file" ref="fileInput" style="display:none" />
  <input type="file" ref="folderInput" style="display:none" />
</template>

<style scoped>

</style>