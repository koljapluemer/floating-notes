<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { useRouter } from 'vue-router'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { store, type NoteEntry } from '../store/notes'
import { renderMarkdown } from '../utils/markdown'

const props = defineProps<{ refreshNotes: () => Promise<void> }>()

interface FloatingCard {
  id: number
  note: NoteEntry
  top: string
  duration: number
}

const router = useRouter()
const activeCards = ref<FloatingCard[]>([])
const filterInput = ref(store.filterText)
let idCounter = 0
let spawnInterval: ReturnType<typeof setInterval> | null = null

onMounted(async () => {
  const win = getCurrentWindow()
  await win.setResizable(true)
  await win.maximize()
  await props.refreshNotes()
  // Small delay to let notes load before spawning
  setTimeout(() => {
    spawnCard()
    spawnInterval = setInterval(spawnCard, store.spawnInterval)
  }, 200)
})

onBeforeUnmount(() => {
  if (spawnInterval) clearInterval(spawnInterval)
})

function spawnCard() {
  const filter = store.filterText.toLowerCase()
  const pool = store.notes.filter(n =>
    !filter || n.content.toLowerCase().includes(filter)
  )
  if (!pool.length) return

  const note = pool[Math.floor(Math.random() * pool.length)]
  const duration = store.minDuration + Math.random() * (store.maxDuration - store.minDuration)
  const card: FloatingCard = {
    id: idCounter++,
    note,
    top: `${10 + Math.random() * 60}%`,
    duration,
  }
  activeCards.value.push(card)

  setTimeout(() => {
    activeCards.value = activeCards.value.filter(c => c.id !== card.id)
  }, duration * 1000 + 500)
}

function onFilterInput() {
  store.filterText = filterInput.value
}

function editNote(note: NoteEntry) {
  store.editingNote = note
  router.push('/')
}
</script>

<template>
  <div class="floating-stage">
    <div
      v-for="card in activeCards"
      :key="card.id"
      class="floating-card card bg-base-200 shadow-lg w-64 cursor-pointer"
      :style="{
        top: card.top,
        animationDuration: `${card.duration}s`,
      }"
      @click="editNote(card.note)"
    >
      <div class="card-body p-3">
        <div
          class="prose prose-sm max-w-none text-xs"
          v-html="renderMarkdown(card.note.content)"
        />
      </div>
    </div>

    <div class="absolute bottom-0 left-0 right-0 p-3 bg-base-200/80 backdrop-blur-sm border-t border-base-300">
      <input
        v-model="filterInput"
        class="input input-bordered input-sm w-full max-w-xs"
        placeholder="filter..."
        @input="onFilterInput"
      />
    </div>
  </div>
</template>

<style scoped>
.floating-stage {
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

@keyframes float-across {
  from {
    transform: translateX(-320px);
    opacity: 0;
  }
  8% {
    opacity: 1;
  }
  92% {
    opacity: 1;
  }
  to {
    transform: translateX(calc(100vw + 320px));
    opacity: 0;
  }
}

.floating-card {
  position: absolute;
  left: 0;
  animation-name: float-across;
  animation-timing-function: linear;
  animation-fill-mode: forwards;
  will-change: transform;
  backface-visibility: hidden;
}
</style>
