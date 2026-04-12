<script setup>

import {ref} from 'vue';

const isOpen = ref(false);
const selected = ref(null);

let dirs = [];
let files = [];

function toggle(){
  isOpen.value = !isOpen.value;
}

function select(option){
  selected.value = option;
  isOpen.value = false;
}

</script>

<template>
  <div class="dropdown">
    <button @click="toggle">File</button>

    <ul v-if="isOpen" class="dropdown-menu">
      <li @click="select('Option 1')">Open file</li>
      <li @click="select('Option 2')">Open folder</li>
      <li @click="select('Option 3')">Create file</li>
      <li @click="select('Option 4')">Create folder</li>

    </ul>
  </div>
</template>

<style scoped>

</style>