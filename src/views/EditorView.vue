<script setup>
import CodeAreaComponent from "../components/CodeAreaComponent.vue";
import TerminalComponent from "../components/TerminalComponent.vue";
import HeaderComponent from "../components/HeaderComponent.vue";

</script>

<template>

  <div class="header">
    <HeaderComponent/>
  </div>

  <div class="editor-view">
    <CodeAreaComponent/>
  </div>

  <div class="terminal-view">
    <TerminalComponent/>
  </div>

</template>

<style scoped>
.editor-view {
  margin-bottom: 8px;
}

.terminal-view {
  margin-top: 8px;
}
</style>