<script setup>
import { computed } from 'vue';

const props = defineProps({
  label: String,
  activeMenu: String,
});

const emit = defineEmits(['toggle']);

const isOpen = computed(() => props.activeMenu === props.label);

function toggle() {
  emit('toggle', props.label);
}
</script>

<template>
  <div class="dropdown">
    <button class="dropdown-toggle" @click="toggle">
      {{ label }}
    </button>

    <ul v-if="isOpen" class="dropdown-menu">
      <slot />
    </ul>
  </div>
</template>