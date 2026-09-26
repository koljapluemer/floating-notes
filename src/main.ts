import { createApp } from 'vue'
import { load } from '@tauri-apps/plugin-store'
import './style.css'
import App from './App.vue'
import router from './router'
import { store } from './store/notes'

async function start() {
  const appStore = await load('settings.json', { defaults: {} })
  const savedFolder = await appStore.get<string>('folderPath')
  const minDuration = await appStore.get<number>('minDuration')
  const maxDuration = await appStore.get<number>('maxDuration')
  const spawnInterval = await appStore.get<number>('spawnInterval')

  if (savedFolder) store.folderPath = savedFolder
  if (minDuration != null) store.minDuration = minDuration
  if (maxDuration != null) store.maxDuration = maxDuration
  if (spawnInterval != null) store.spawnInterval = spawnInterval

  createApp(App).use(router).mount('#app')
}

start().catch((error) => {
  console.error('Failed to start application:', error)
})
