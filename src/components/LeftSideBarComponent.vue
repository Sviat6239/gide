<script setup>
import FileThreeComponent from './FileThreeComponent.vue';
import PullRequestsComponent from "./PullRequestsComponent.vue";
import CommitComponent from "./CommitComponent.vue";
import '../styles/LeftSideBar.css';

const props = defineProps({
  files: { type: Array, default: () => [] },

  isFileTreeOpen: { type: Boolean, default: true },
  isTerminalOpen: { type: Boolean, default: false },

  isGitOpen: { type: Boolean, default: false },
  isCommitOpen: { type: Boolean, default: false },
  isPullRequestsOpen: { type: Boolean, default: false },

  terminalHeight: { type: Number, default: 240 },
  fileTreeMaxHeight: { type: String, default: '100%' },
});

const emit = defineEmits([
  'open-file',
  'toggle-file-tree',
  'toggle-terminal',
  'toggle-git',
  'toggle-commit',
  'toggle-pull-requests',
]);
</script>

<template>
  <div class="left-sidebar-shell">

    <!-- TOOLBAR -->
    <nav class="left-toolbar">

      <!-- TOP SECTION -->
      <div class="toolbar-top">

        <!-- FILE TREE -->
        <button
            class="tool-button"
            :class="{ active: props.isFileTreeOpen }"
            title="Project"
            @click="emit('toggle-file-tree')">

          <svg viewBox="0 0 24 24" class="tool-icon">
            <path d="M3 5h7l2 2h9v12H3z" fill="none" stroke="currentColor" stroke-width="1.8"/>
            <path d="M3 9h18" fill="none" stroke="currentColor" stroke-width="1.8"/>
          </svg>
        </button>

        <!-- COMMIT -->
        <button
            class="tool-button"
            :class="{ active: props.isCommitOpen }"
            title="Commit"
            @click="emit('toggle-commit')">

          <svg viewBox="0 0 24 24" class="tool-icon">
            <path d="M4 12h16" stroke="currentColor" stroke-width="1.6"/>
            <circle cx="12" cy="12" r="3" fill="none" stroke="currentColor" stroke-width="1.6"/>
          </svg>
        </button>

        <!-- PULL REQUESTS -->
        <button
            class="tool-button"
            :class="{ active: props.isPullRequestsOpen }"
            title="Pull Requests"
            @click="emit('toggle-pull-requests')">

          <svg viewBox="0 0 24 24" class="tool-icon">
            <g transform="rotate(180 12 12)">
              <circle cx="6" cy="6" r="2" fill="currentColor"/>
              <circle cx="18" cy="6" r="2" fill="currentColor"/>
              <circle cx="12" cy="18" r="2" fill="currentColor"/>
              <path d="M6 8v4c0 3 6 3 6 6" stroke="currentColor" stroke-width="1.6" fill="none"/>
              <path d="M18 8v4c0 3-6 3-6 6" stroke="currentColor" stroke-width="1.6" fill="none"/>
            </g>
          </svg>
        </button>

      </div>

      <!-- BOTTOM SECTION -->
      <div class="toolbar-bottom">

        <!-- TERMINAL -->
        <button
            class="tool-button"
            :class="{ active: props.isTerminalOpen }"
            title="Terminal"
            @click="emit('toggle-terminal')">

          <svg viewBox="0 0 24 24" class="tool-icon">
            <path d="M4 5h16v14H4z" fill="none" stroke="currentColor" stroke-width="1.8"/>
            <path d="M8 10l2 2-2 2" fill="none" stroke="currentColor" stroke-width="1.8"/>
            <path d="M12 14h4" fill="none" stroke="currentColor" stroke-width="1.8"/>
          </svg>
        </button>

        <!-- GIT -->
        <button
            class="tool-button"
            :class="{ active: props.isGitOpen }"
            title="Git"
            @click="emit('toggle-git')">

          <svg viewBox="0 0 24 24" class="tool-icon">
            <circle cx="6" cy="6" r="2" fill="currentColor"/>
            <circle cx="18" cy="6" r="2" fill="currentColor"/>
            <circle cx="12" cy="18" r="2" fill="currentColor"/>
            <path d="M6 8v4c0 3 6 3 6 6" stroke="currentColor" stroke-width="1.6" fill="none"/>
            <path d="M18 8v4c0 3-6 3-6 6" stroke="currentColor" stroke-width="1.6" fill="none"/>
          </svg>
        </button>

      </div>

    </nav>

    <!-- PANELS -->
    <div class="left-panels" v-if="isFileTreeOpen || isCommitOpen || isPullRequestsOpen">

      <!-- FILE TREE -->
      <div
          v-if="props.isFileTreeOpen"
          class="left-content"
          :style="{ height: '100%' }">

        <FileThreeComponent
            :files="props.files"
            @open-file="emit('open-file', $event)"
            @close="emit('toggle-file-tree')"
        />
      </div>

      <!-- COMMIT -->
      <div
          v-if="props.isCommitOpen"
          class="left-content"
          :style="{ height: '100%' }">

        <CommitComponent
            :files="props.files"
            @open-file="emit('open-file', $event)"
            @close="emit('toggle-commit')"/>
      </div>

      <!-- Pull Requests-->
      <div
          v-if="props.isPullRequestsOpen"
          class="left-content"
          :style="{ height: '100%' }">

        <PullRequestsComponent @close="emit('toggle-pull-requests')" />
      </div>

    </div>
  </div>
</template>
