<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { LogicalSize } from '@tauri-apps/api/dpi'
import { open } from '@tauri-apps/plugin-dialog'
import { load } from '@tauri-apps/plugin-store'
import { store } from '../store/notes'
import { FolderOpenIcon } from 'lucide-vue-next'

onMounted(async () => {
  const win = getCurrentWindow()
  await win.setResizable(false)
  await win.setSize(new LogicalSize(480, 380))
})

const minDuration = ref(store.minDuration)
const maxDuration = ref(store.maxDuration)
const spawnIntervalSec = ref(store.spawnInterval / 1000)

async function getAppStore() {
  return load('settings.json', { defaults: {} })
}

async function changeFolder() {
  const selected = await open({ directory: true, multiple: false, title: 'Choose notes folder' })
  if (selected && typeof selected === 'string') {
    store.folderPath = selected
    const appStore = await getAppStore()
    await appStore.set('folderPath', selected)
    await appStore.save()
  }
}

watch(minDuration, async (val) => {
  // Clamp so min never exceeds max
  if (val > maxDuration.value) maxDuration.value = val
  store.minDuration = val
  const appStore = await getAppStore()
  await appStore.set('minDuration', val)
  await appStore.save()
})

watch(maxDuration, async (val) => {
  if (val < minDuration.value) minDuration.value = val
  store.maxDuration = val
  const appStore = await getAppStore()
  await appStore.set('maxDuration', val)
  await appStore.save()
})

watch(spawnIntervalSec, async (val) => {
  store.spawnInterval = Math.round(val * 1000)
  const appStore = await getAppStore()
  await appStore.set('spawnInterval', store.spawnInterval)
  await appStore.save()
})
</script>

<template>
  <div class="flex flex-col h-full p-5 gap-5 overflow-y-auto">

    <!-- Folder -->
    <div class="flex flex-col gap-1">
      <span class="text-xs font-semibold uppercase tracking-wider opacity-50">Notes folder</span>
      <div class="flex items-center gap-2">
        <span class="text-xs font-mono truncate flex-1 opacity-70">{{ store.folderPath || '—' }}</span>
        <button class="btn btn-ghost btn-xs gap-1" @click="changeFolder">
          <FolderOpenIcon :size="13" />
          change
        </button>
      </div>
    </div>

    <div class="divider my-0" />

    <!-- Speed -->
    <div class="flex flex-col gap-3">
      <span class="text-xs font-semibold uppercase tracking-wider opacity-50">Card speed</span>

      <div class="flex flex-col gap-1">
        <div class="flex justify-between text-xs opacity-60">
          <span>min (fast)</span>
          <span>{{ minDuration }}s</span>
        </div>
        <input
          v-model.number="minDuration"
          type="range" min="3" max="60" step="1"
          class="range range-xs"
        />
      </div>

      <div class="flex flex-col gap-1">
        <div class="flex justify-between text-xs opacity-60">
          <span>max (slow)</span>
          <span>{{ maxDuration }}s</span>
        </div>
        <input
          v-model.number="maxDuration"
          type="range" min="3" max="60" step="1"
          class="range range-xs"
        />
      </div>
    </div>

    <div class="divider my-0" />

    <!-- Spawn rate -->
    <div class="flex flex-col gap-3">
      <span class="text-xs font-semibold uppercase tracking-wider opacity-50">Spawn rate</span>
      <div class="flex flex-col gap-1">
        <div class="flex justify-between text-xs opacity-60">
          <span>interval</span>
          <span>{{ spawnIntervalSec.toFixed(1) }}s</span>
        </div>
        <input
          v-model.number="spawnIntervalSec"
          type="range" min="0.5" max="10" step="0.5"
          class="range range-xs"
        />
      </div>
    </div>

  </div>
</template>
