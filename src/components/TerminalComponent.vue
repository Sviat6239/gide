<script setup>
import { nextTick, onMounted, ref } from 'vue';
import "../styles/Terminal.css";

const emit = defineEmits(['close']);

const commandInput = ref('');
const terminalHistory = ref([
  { id: 'seed-system-line', type: 'system', text: 'Gide terminal ready. Type help for available commands.' },
]);
const terminalInputRef = ref(null);
const terminalHistoryRef = ref(null);
const previousCommands = ref([]);
const previousCommandIndex = ref(-1);

const scrollHistoryToBottom = () => {
  const historyElement = terminalHistoryRef.value;

  if (!historyElement) {
    return;
  }

  historyElement.scrollTop = historyElement.scrollHeight;
};

const focusInput = () => {
  terminalInputRef.value?.focus();
};

const pushHistory = (entry) => {
  terminalHistory.value.push({
    id: `${Date.now()}-${Math.random().toString(16).slice(2)}`,
    ...entry,
  });
};

const runCommand = (command) => {
  const trimmedCommand = command.trim();

  if (!trimmedCommand) {
    return;
  }

  terminalHistory.value.push({
    id: `${Date.now()}-${Math.random().toString(16).slice(2)}`,
    type: 'command',
    text: `gide@workspace:~$ ${trimmedCommand}`,
  });

  previousCommands.value.unshift(trimmedCommand);
  previousCommandIndex.value = -1;

  if (trimmedCommand === 'clear') {
    terminalHistory.value = [];
    commandInput.value = '';
    return;
  }

  const [commandName, ...commandArgs] = trimmedCommand.split(/\s+/);

  if (commandName === 'help') {
    pushHistory({
      type: 'output',
      text: [
        'Available commands:',
        '  help   - show this message',
        '  clear  - clear terminal history',
        '  echo   - print text back to the terminal',
      ].join('\n'),
    });
  } else if (commandName === 'echo') {
    pushHistory({
      type: 'output',
      text: commandArgs.join(' '),
    });
  } else {
    pushHistory({
      type: 'output',
      text: `gide: command not found: ${trimmedCommand}`,
    });
  }

  commandInput.value = '';
};

const submitCommand = () => {
  runCommand(commandInput.value);
  nextTick(scrollHistoryToBottom);
};

const handleKeydown = (event) => {
  if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault();
    submitCommand();
    return;
  }

  if (event.key === 'ArrowUp') {
    if (previousCommands.value.length === 0) {
      return;
    }

    event.preventDefault();

    if (previousCommandIndex.value < previousCommands.value.length - 1) {
      previousCommandIndex.value += 1;
    }

    commandInput.value = previousCommands.value[previousCommandIndex.value] ?? commandInput.value;
    return;
  }

  if (event.key === 'ArrowDown') {
    if (previousCommandIndex.value <= -1) {
      return;
    }

    event.preventDefault();
    previousCommandIndex.value -= 1;

    if (previousCommandIndex.value === -1) {
      commandInput.value = '';
      return;
    }

    commandInput.value = previousCommands.value[previousCommandIndex.value] ?? '';
  }
};

onMounted(() => {
  focusInput();
  scrollHistoryToBottom();
});
</script>

<template>
  <div class="terminal-area" @pointerdown="focusInput">
    <div class="terminal-header">
      <p>Terminal</p>
      <button type="button" class="terminal-close" @click="emit('close')">-</button>
    </div>

    <div ref="terminalHistoryRef" class="terminal-history">
      <div
          v-for="entry in terminalHistory"
          :key="entry.id"
          class="terminal-line"
          :class="`terminal-line-${entry.type}`"
      >
        <pre>{{ entry.text }}</pre>
      </div>
    </div>

    <label class="terminal-input-row" for="terminal-input">
      <span class="terminal-prompt">gide@workspace:~$</span>
      <input
          id="terminal-input"
          ref="terminalInputRef"
          v-model="commandInput"
          class="terminal-input"
          type="text"
          autocomplete="off"
          spellcheck="false"
          autocapitalize="off"
          @keydown="handleKeydown"
      />
    </label>
  </div>
</template>

<style scoped>
</style>