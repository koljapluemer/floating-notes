<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { LogicalSize } from '@tauri-apps/api/dpi'
import { store } from '../store/notes'
import { slugifyToFilename } from '../utils/slugify'

const props = defineProps<{ refreshNotes: () => Promise<void> }>()

const content = ref('')

onMounted(async () => {
  const win = getCurrentWindow()
  await win.setResizable(false)
  await win.setSize(new LogicalSize(480, 380))
})

async function handleSubmit() {
  const text = content.value.trim()
  if (!text || !store.folderPath) return

  const filename = slugifyToFilename(text)

  try {
    await invoke('save_note', { folder: store.folderPath, filename, body: text })
    await props.refreshNotes()
    content.value = ''
  } catch (e) {
    console.error('save_note error:', e)
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
      <button class="btn btn-primary btn-sm" @click="handleSubmit">
        add
      </button>
    </div>
  </div>
</template>
