<script setup>
import { computed } from "vue";
import "../styles/Terminal.css";

const emit = defineEmits(["close"]);

const sections = [
  {
    title: "Сьогодні",
    accent: "#64748b",
    items: [
      { title: "Перевірити вхідні pull request'и", note: "2 зауваження на рев'ю", state: "in progress" },
      { title: "Зібрати список багів із трекера", note: "3 критичні", state: "open" },
    ],
  },
  {
    title: "У роботі",
    accent: "#64748b",
    items: [
      { title: "Синхронізувати todo з Git-завданнями", note: "Власник: frontend", state: "in progress" },
      { title: "Довести інтерфейс панелі редактора", note: "Доопрацювання макета", state: "open" },
    ],
  },
  {
    title: "Готово",
    accent: "#64748b",
    items: [
      { title: "Додати базову навігацію панелі", note: "Завершено сьогодні", state: "done" },
      { title: "Підключити стилі для термінала", note: "Стабільно", state: "done" },
    ],
  },
];

const totalTasks = computed(() => sections.reduce((count, section) => count + section.items.length, 0));
const doneTasks = computed(() => sections.find((section) => section.title === "Готово")?.items.length ?? 0);
const activeTasks = computed(() => totalTasks.value - doneTasks.value);
</script>

<template>
  <div class="terminal-area todo-panel">
    <div class="terminal-header todo-header">
      <div>
        <p>TODO</p>
        <span class="todo-subtitle">План завдань, поточні справи та виконані пункти</span>
      </div>
      <button type="button" class="terminal-close" @click="emit('close')">×</button>
    </div>

    <div class="todo-summary">
      <div class="todo-summary-card">
        <span class="todo-summary-label">Усього завдань</span>
        <strong>{{ totalTasks }}</strong>
      </div>
      <div class="todo-summary-card">
        <span class="todo-summary-label">У роботі</span>
        <strong>{{ activeTasks }}</strong>
      </div>
      <div class="todo-summary-card">
        <span class="todo-summary-label">Готово</span>
        <strong>{{ doneTasks }}</strong>
      </div>
    </div>

    <div class="todo-sections">
      <section v-for="section in sections" :key="section.title" class="todo-section">
        <div class="todo-section-header">
          <span class="todo-section-dot" :style="{ backgroundColor: section.accent }"></span>
          <h3>{{ section.title }}</h3>
          <span class="todo-section-count">{{ section.items.length }}</span>
        </div>

        <div class="todo-items">
          <article v-for="item in section.items" :key="item.title" class="todo-item">
            <div class="todo-item-top">
              <h4>{{ item.title }}</h4>
              <span class="todo-item-state" :data-state="item.state">{{ item.state }}</span>
            </div>
            <p>{{ item.note }}</p>
          </article>
        </div>
      </section>
    </div>
  </div>
</template>

<style scoped>
.todo-panel {
  display: flex;
  flex-direction: column;
  min-height: 100%;
  background: var(--app-bg-color);
}

.todo-header {
  align-items: flex-start;
}

.todo-header p {
  margin-bottom: 2px;
}

.todo-subtitle {
  display: block;
  font-size: 12px;
  color: rgba(107, 114, 128, 0.95);
}

.todo-summary {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 10px;
  padding: 12px 12px 4px;
}

.todo-summary-card {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 10px 12px;
  border: 1px solid rgba(148, 163, 184, 0.45);
  background: rgba(255, 255, 255, 0.02);
}

.todo-summary-label {
  font-size: 11px;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  color: rgba(107, 114, 128, 0.95);
}

.todo-summary-card strong {
  font-size: 24px;
  line-height: 1;
}

.todo-sections {
  display: grid;
  gap: 12px;
  padding: 8px 12px 12px;
  overflow: auto;
}

.todo-section {
  padding: 12px;
  border: 1px solid rgba(148, 163, 184, 0.45);
  background: rgba(255, 255, 255, 0.02);
}

.todo-section-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
}

.todo-section-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
}

.todo-section-dot {
  width: 10px;
  height: 10px;
  background-color: #64748b;
}

.todo-section-count {
  margin-left: auto;
  font-size: 12px;
  color: rgba(107, 114, 128, 0.95);
}

.todo-items {
  display: grid;
  gap: 8px;
}

.todo-item {
  padding: 10px 12px;
  border: 1px solid rgba(148, 163, 184, 0.35);
  background: rgba(255, 255, 255, 0.03);
}

.todo-item-top {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  justify-content: space-between;
}

.todo-item h4 {
  margin: 0;
  font-size: 13px;
  line-height: 1.35;
}

.todo-item p {
  margin: 6px 0 0;
  font-size: 12px;
  color: rgba(107, 114, 128, 0.95);
}

.todo-item-state {
  flex-shrink: 0;
  padding: 2px 8px;
  border: 1px solid rgba(148, 163, 184, 0.45);
  font-size: 11px;
  text-transform: capitalize;
  color: rgba(55, 65, 81, 0.95);
  background: rgba(255, 255, 255, 0.04);
}

.todo-item-state[data-state="in progress"] {
  border-color: rgba(148, 163, 184, 0.55);
  color: #475569;
}

.todo-item-state[data-state="open"] {
  border-color: rgba(148, 163, 184, 0.55);
  color: #475569;
}

.todo-item-state[data-state="done"] {
  border-color: rgba(148, 163, 184, 0.55);
  color: #475569;
}

@media (max-width: 920px) {
  .todo-summary {
    grid-template-columns: 1fr;
  }
}
</style>