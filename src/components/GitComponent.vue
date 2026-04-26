<script setup>
import "../styles/Terminal.css";

const emit = defineEmits(['close']);
</script>

<template>
  <div class="terminal-area">
    <div class="terminal-header">
      <p>Git</p>
      <button type="button" class="terminal-close" @click="emit('close')">-</button>
    </div>
    <textarea name="" id="" cols="30" rows="10" class="terminal-textarea"></textarea>
  </div>
</template>

<style scoped>

</style>