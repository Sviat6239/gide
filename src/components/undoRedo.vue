<script>
function undo() {
  if (!props.activeTab || undoStack.value.length === 0) {
    return;
  }

  redoStack.value.push(props.activeTab.content);

  const previousContent = undoStack.value.pop();

  emit('update-content', { id: props.activeTab.id, content: previousContent });
  lastSavedContent.value = previousContent;

  activeMenu.value = null;
}

function redo() {
  if (!props.activeTab || redoStack.value.length === 0) {
    return;
  }

  undoStack.value.push(props.activeTab.content);

  const nextContent = redoStack.value.pop();

  emit('update-content', { id: props.activeTab.id, content: nextContent });
  lastSavedContent.value = nextContent;

  activeMenu.value = null;
}
</script>