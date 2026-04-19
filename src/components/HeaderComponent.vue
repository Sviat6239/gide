<script setup>
import { ref } from 'vue';
import '../styles/Header.css';

const isOpen = ref(false);
const isSubOpen = ref(false);

const fileInput = ref(null);
const folderInput = ref(null);

function toggle() {
  isOpen.value = !isOpen.value;
}

function toggleSub(){
  isSubOpen.value = !isSubOpen.value;
}

function openFile() {
  fileInput.value.click();
}

function openFolder() {
  folderInput.value.click();
}

function createNew(){

}

function createEmptyFile(){
  alert("You have been created new empty file")
}

function createRustFile(){
  alert("You have been created new Rust file")
}

function createPythonFile(){
  alert("You have been created new Python file")
}

function createHtmlFile(){
  alert("You have been created new HTML file")
}

function createCssFile(){
  alert("You have been created new CSS file")
}

function createJsFile(){
  alert("You have been created new JavaScript file")
}

function createTsFile(){
  alert("You have been created new TypeScript file")
}

function createFolder(){
  alert("You have been created new folder")
}

function handleFile(event) {
  const files = event.target.files;
  console.log(files);
}

function handleFolder(event) {
  const files = event.target.files;
  console.log(files);
}
</script>

<template>
  <div class="dropdown">
    <button type="button" class="dropdown-toggle" @click="toggle">File</button>

    <ul v-if="isOpen" class="dropdown-menu">
      <li><button @click="openFile">Open file</button></li>
      <li><button @click="openFolder">Open folder</button></li>

      <li
          class="has-submenu"
          @mouseenter="isSubOpen = true"
          @mouseleave="isSubOpen = false"
      >
        <button type="button" @click="createNew()">New</button>

        <ul v-if="isSubOpen" class="dropdown-submenu">
          <li><button @click="createEmptyFile()">File</button></li>
          <li><button @click="createRustFile()">Rust File</button></li>
          <li><button @click="createPythonFile()">Python File</button></li>
          <li><button @click="createHtmlFile()">HTML File</button></li>
          <li><button @click="createCssFile()">CSS File</button></li>
          <li><button @click="createJsFile()">JavaScript File</button></li>
          <li><button @click="createTsFile()">TypeScript File</button></li>
          <li><button @click="createFolder">Folder</button></li>
        </ul>
      </li>
    </ul>

    <input
        type="file"
        ref="fileInput"
        style="display:none"
        @change="handleFile"
    />

    <input
        type="file"
        ref="folderInput"
        webkitdirectory
        directory
        style="display:none"
        @change="handleFolder"
    />
  </div>
</template>

<style scoped>

</style>