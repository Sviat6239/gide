<script setup>
import { ref } from 'vue';
import FileThreeComponent from './FileThreeComponent.vue';
import '../styles/LeftSideBar.css';

const emit = defineEmits(['open-file']);
const activeSection = ref('file-tree');

function setSection(section) {
  activeSection.value = section;
}
</script>

<template>
  <div class="left-sidebar-shell">
	<nav class="left-toolbar">
	  <button
		type="button"
		class="tool-button"
		:class="{ active: activeSection === 'file-tree' }"
		title="File Tree"
		@click="setSection('file-tree')"
	  >
		<svg viewBox="0 0 24 24" aria-hidden="true" class="tool-icon">
		  <path d="M3 5h7l2 2h9v12H3z" fill="none" stroke="currentColor" stroke-width="1.8" />
		  <path d="M3 9h18" fill="none" stroke="currentColor" stroke-width="1.8" />
		</svg>
	  </button>
	</nav>

	<div class="left-content">
	  <FileThreeComponent
		v-if="activeSection === 'file-tree'"
		@open-file="emit('open-file', $event)"
	  />
	</div>
  </div>
</template>

<style scoped>

</style>