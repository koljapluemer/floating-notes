<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { LogicalSize } from '@tauri-apps/api/dpi'
import { store } from '../store/notes'
import { slugifyToFilename } from '../utils/slugify'
import { Trash2Icon } from 'lucide-vue-next'

const props = defineProps<{ refreshNotes: () => Promise<void> }>()

const content = ref('')
const isEditing = computed(() => store.editingNote !== null)

onMounted(async () => {
  const win = getCurrentWindow()
  await win.setResizable(false)
  await win.setSize(new LogicalSize(480, 380))

  if (store.editingNote) {
    content.value = store.editingNote.content
  }
})

async function handleSubmit() {
  const text = content.value.trim()
  if (!text || !store.folderPath) return

  const filename = isEditing.value
    ? store.editingNote!.filename
    : slugifyToFilename(text)

  try {
    await invoke('save_note', { folder: store.folderPath, filename, content: text })
    await props.refreshNotes()
    content.value = ''
    store.editingNote = null
  } catch (e) {
    console.error('save_note error:', e)
  }
}

async function handleDelete() {
  if (!store.editingNote || !store.folderPath) return
  try {
    await invoke('delete_note', { folder: store.folderPath, filename: store.editingNote.filename })
    await props.refreshNotes()
    content.value = ''
    store.editingNote = null
  } catch (e) {
    console.error('delete_note error:', e)
  }
}
</script>

<template>
  <div class="flex flex-col h-full p-4 gap-3">
    <textarea
      v-model="content"
      class="textarea textarea-bordered flex-1 resize-none text-sm font-mono"
      placeholder="..."
      autofocus
      @keydown.ctrl.enter="handleSubmit"
      @keydown.meta.enter="handleSubmit"
    />
    <div class="flex gap-2 justify-end">
      <button
        v-if="isEditing"
        class="btn btn-error btn-sm gap-1"
        @click="handleDelete"
      >
        <Trash2Icon :size="14" />
        delete
      </button>
      <button class="btn btn-primary btn-sm" @click="handleSubmit">
        {{ isEditing ? 'save' : 'add' }}
      </button>
    </div>
  </div>
</template>
