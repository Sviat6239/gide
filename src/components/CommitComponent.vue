<script setup>
import { computed, ref } from 'vue';
import '../styles/FileThree.css';

const props = defineProps({
  files: {
    type: Array,
    default: () => [],
  },
});

const emit = defineEmits(['open-file', 'close']);

const expandedMap = ref({});

function normalizePath(path) {
  return String(path || '').replace(/\\/g, '/').replace(/^\/+/, '');
}

function buildTreeFromFiles(files) {
  const root = {
    name: 'workspace',
    path: '',
    isDir: true,
    children: [],
  };

  files.forEach((file) => {
    const fullName = normalizePath(file.fullName);

    if (!fullName) {
      return;
    }

    const parts = fullName.split('/').filter(Boolean);
    let cursor = root;

    parts.forEach((part, index) => {
      const isLast = index === parts.length - 1;
      const nextPath = cursor.path ? `${cursor.path}/${part}` : part;
      let nextNode = cursor.children.find((child) => child.path === nextPath);

      if (!nextNode) {
        nextNode = {
          name: part,
          path: nextPath,
          isDir: !isLast,
          children: [],
        };
        cursor.children.push(nextNode);
      }

      if (!isLast) {
        nextNode.isDir = true;
      }

      cursor = nextNode;
    });
  });

  const sortChildren = (node) => {
    node.children.sort((left, right) => {
      if (left.isDir !== right.isDir) {
        return left.isDir ? -1 : 1;
      }

      return left.name.localeCompare(right.name);
    });

    node.children.forEach(sortChildren);
  };

  sortChildren(root);
  return root;
}

const treeRoot = computed(() => buildTreeFromFiles(props.files));

const visibleNodes = computed(() => {
  if (!treeRoot.value?.children?.length) {
    return [];
  }

  const result = [];

  const walk = (node, depth) => {
    result.push({ node, depth });

    if (!node.isDir || !expandedMap.value[node.path]) {
      return;
    }

    node.children.forEach((child) => walk(child, depth + 1));
  };

  treeRoot.value.children.forEach((child) => walk(child, 0));
  return result;
});

function isExpanded(path) {
  return Boolean(expandedMap.value[path]);
}

function toggleDirectory(path) {
  expandedMap.value = {
    ...expandedMap.value,
    [path]: !expandedMap.value[path],
  };
}

function handleNodeClick(node) {
  if (node.isDir) {
    toggleDirectory(node.path);
    return;
  }

  emit('open-file', { fullName: node.path });
}
</script>

<template>
  <section class="file-tree-wrapper">
    <header class="file-tree-header">
      <p>File Tree</p>
      <button type="button" class="collapse-button" @click="emit('close')">-</button>
    </header>

    <p class="tree-status" v-if="!visibleNodes.length">No files yet. Import a folder or create files.</p>

    <div class="tree-list" v-else>
      <button
          v-for="entry in visibleNodes"
          :key="entry.node.path"
          type="button"
          class="tree-node"
          :style="{ paddingLeft: `${entry.depth * 14 + 8}px` }"
          @click="handleNodeClick(entry.node)"
      >
        <span class="node-caret" v-if="entry.node.isDir">{{ isExpanded(entry.node.path) ? 'v' : '>' }}</span>
        <span class="node-caret" v-else> </span>
        <span class="node-kind">{{ entry.node.isDir ? '[D]' : '[F]' }}</span>
        <span class="node-name">{{ entry.node.name }}</span>
      </button>
    </div>
  </section>
</template>

<style scoped>

</style>