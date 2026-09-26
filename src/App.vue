<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { store, type NoteEntry } from './store/notes'
import { LayersIcon, SettingsIcon } from 'lucide-vue-next'

async function refreshNotes() {
  if (!store.folderPath) return
  try {
    const notes = await invoke<NoteEntry[]>('list_notes', { folder: store.folderPath })
    store.notes = notes
  } catch (e) {
    console.error('list_notes error:', e)
  }
}

</script>

<template>
  <div class="flex flex-col h-screen bg-base-100 text-base-content select-none">
    <nav class="navbar bg-base-200 border-b border-base-300 min-h-10 px-3 py-1">
      <div class="navbar-start">
        <span class="text-sm font-semibold tracking-tight">floating-notes</span>
      </div>
      <div class="navbar-end gap-1">
        <RouterLink to="/" class="btn btn-ghost btn-xs gap-1">
          <LayersIcon :size="13" />
          <span>float</span>
        </RouterLink>
        <RouterLink to="/settings" class="btn btn-ghost btn-xs gap-1">
          <SettingsIcon :size="13" />
        </RouterLink>
      </div>
    </nav>
    <main class="flex-1 overflow-hidden">
      <RouterView :refresh-notes="refreshNotes" />
    </main>
  </div>
</template>
