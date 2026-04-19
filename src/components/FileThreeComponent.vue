<script setup>
import { computed, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import '../styles/FileThree.css';

const emit = defineEmits(['open-file']);

const rootPath = ref('');
const treeRoot = ref(null);
const expandedMap = ref({});
const loading = ref(false);
const errorText = ref('');

const visibleNodes = computed(() => {
  if (!treeRoot.value) {
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

  walk(treeRoot.value, 0);
  return result;
});

async function loadTree() {
  loading.value = true;
  errorText.value = '';

  try {
	if (!rootPath.value) {
	  rootPath.value = await invoke('get_default_root');
	}

	const root = await invoke('read_directory_tree', { rootPath: rootPath.value });
	treeRoot.value = root;
	expandedMap.value = {
	  [root.path]: true,
	};
  } catch (error) {
	errorText.value = `Failed to load tree: ${String(error)}`;
  } finally {
	loading.value = false;
  }
}

function isExpanded(path) {
  return Boolean(expandedMap.value[path]);
}

function toggleDirectory(path) {
  expandedMap.value = {
	...expandedMap.value,
	[path]: !expandedMap.value[path],
  };
}

async function handleNodeClick(node) {
  if (node.isDir) {
	toggleDirectory(node.path);
	return;
  }

  try {
	const content = await invoke('read_text_file', { filePath: node.path });
	emit('open-file', {
	  fullName: node.path,
	  content,
	});
  } catch (error) {
	errorText.value = `Failed to open file: ${String(error)}`;
  }
}

onMounted(loadTree);
</script>

<template>
  <section class="file-tree-wrapper">
	<header class="file-tree-header">
	  <p>File Tree</p>
	  <button type="button" class="reload-button" @click="loadTree">Reload</button>
	</header>

	<p class="root-path" v-if="rootPath">{{ rootPath }}</p>
	<p class="tree-status" v-if="loading">Loading...</p>
	<p class="tree-error" v-else-if="errorText">{{ errorText }}</p>

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