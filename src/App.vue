<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import "./styles/style.css";

import "./views/EditorView.vue";
import EditorView from "./views/EditorView.vue";

const greetMsg = ref("");
const name = ref("");

</script>

<template>
  <EditorView />
</template>

