<script setup>
import FileThreeComponent from './FileThreeComponent.vue';
import TerminalComponent from './TerminalComponent.vue';
import '../styles/LeftSideBar.css';

const props = defineProps({
  files: {
	type: Array,
	default: () => [],
  },
  isFileTreeOpen: {
	type: Boolean,
	default: true,
  },
  isTerminalOpen: {
	type: Boolean,
	default: true,
  },
  terminalHeight: {
	type: Number,
	default: 240,
  },
  fileTreeMaxHeight: {
	type: String,
	default: '100%',
  },
});

const emit = defineEmits(['open-file', 'toggle-file-tree', 'toggle-terminal', 'start-terminal-resize']);

function toggleTree() {
  emit('toggle-file-tree');
}
</script>

<template>
  <div class="left-sidebar-shell">
	<nav class="left-toolbar">
	  <button
		type="button"
		class="tool-button"
		:class="{ active: props.isFileTreeOpen }"
		title="Toggle File Tree"
		@click="toggleTree"
	  >
		<svg viewBox="0 0 24 24" aria-hidden="true" class="tool-icon">
		  <path d="M3 5h7l2 2h9v12H3z" fill="none" stroke="currentColor" stroke-width="1.8" />
		  <path d="M3 9h18" fill="none" stroke="currentColor" stroke-width="1.8" />
		</svg>
	  </button>

	  <button
		type="button"
		class="tool-button terminal-toggle"
		:class="{ active: props.isTerminalOpen }"
		title="Toggle Terminal"
		@click="emit('toggle-terminal')"
	  >
		<svg viewBox="0 0 24 24" aria-hidden="true" class="tool-icon">
		  <path d="M4 5h16v14H4z" fill="none" stroke="currentColor" stroke-width="1.8" />
		  <path d="M8 10l2 2-2 2" fill="none" stroke="currentColor" stroke-width="1.8" />
		  <path d="M12 14h4" fill="none" stroke="currentColor" stroke-width="1.8" />
		</svg>
	  </button>
	</nav>

    <div class="left-panels">
	  <div
		v-if="props.isFileTreeOpen"
		class="left-content"
		:style="{ height: props.fileTreeMaxHeight, maxHeight: props.fileTreeMaxHeight }"
	  >
		<FileThreeComponent
		  :files="props.files"
		  @open-file="emit('open-file', $event)"
		  @close="emit('toggle-file-tree')"
		/>
	  </div>

	  <div
		v-if="props.isFileTreeOpen && props.isTerminalOpen"
		class="left-resize-handle"
		@pointerdown.prevent="emit('start-terminal-resize')"
	  ></div>

	  <div
		v-if="props.isTerminalOpen"
		class="left-terminal-space"
		:style="{ height: `${props.terminalHeight}px` }"
	  >
		<TerminalComponent @close="emit('toggle-terminal')" />
	  </div>
    </div>
  </div>
</template>

<style scoped>

</style>