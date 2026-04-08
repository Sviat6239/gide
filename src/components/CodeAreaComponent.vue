<script setup>
import { computed, ref } from "vue";
import "../styles/CodeArea.css";

const text = ref("");
const editorRef = ref(null);
const linesRef = ref(null);

const lineCount = computed(() => text.value.split("\n").length);

function syncScroll() {
  if (!editorRef.value || !linesRef.value) {
    return;
  }

  linesRef.value.scrollTop = editorRef.value.scrollTop;
}
</script>
<template>
  <div class="code-area">
    <div class="line-count" ref="linesRef">
      <div class="line-number" v-for="n in lineCount" :key="n">{{ n }}</div>
    </div>
    <textarea ref="editorRef" class="editor" v-model="text" @scroll="syncScroll"></textarea>
  </div>

</template>

<style scoped>

</style>