<script setup>
import { computed, ref } from 'vue';
import '../styles/CodeArea.css';

const props = defineProps({
  tabs: {
    type: Array,
    default: () => [],
  },
  activeTabId: {
    type: Number,
    default: null,
  },
});

const emit = defineEmits(['select-tab', 'close-tab', 'update-content']);

const editorRef = ref(null);
const linesRef = ref(null);

const activeTab = computed(() => props.tabs.find((tab) => tab.id === props.activeTabId) || null);

const editorText = computed({
  get: () => activeTab.value?.content ?? '',
  set: (value) => {
    if (!activeTab.value) {
      return;
    }

    emit('update-content', { id: activeTab.value.id, content: value });
  },
});

const lineCount = computed(() => {
  const content = editorText.value;
  return content ? content.split('\n').length : 1;
});

function syncScroll() {
  if (!editorRef.value || !linesRef.value) {
    return;
  }

  linesRef.value.scrollTop = editorRef.value.scrollTop;
}
</script>
<template>
  <div class="code-area">
    <div class="file-tabs">
      <div
          v-for="tab in tabs"
          :key="tab.id"
          class="file-tab"
          :class="{ active: tab.id === activeTabId }"
          @click="emit('select-tab', tab.id)"
      >
        <p>{{ tab.fullName }}</p>
        <button class="close-tab" @click.stop="emit('close-tab', tab.id)">x</button>
      </div>
      <p v-if="!tabs.length" class="empty-tabs">No open files</p>
    </div>
    <div class="editor-container">
      <div class="line-count" ref="linesRef">
        <div class="line-number" v-for="n in lineCount" :key="n">{{ n }}</div>
      </div>
      <textarea
          ref="editorRef"
          class="editor"
          v-model="editorText"
          :disabled="!activeTab"
          :placeholder="activeTab ? '' : 'Create or open a file to start editing'"
          @scroll="syncScroll"
      ></textarea>
    </div>
  </div>

</template>

<style scoped>

</style>