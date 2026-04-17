<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { store, type NoteEntry } from '../store/notes'
import { renderMarkdown } from '../utils/markdown'

const props = defineProps<{ refreshNotes: () => Promise<void> }>()

type GridSize = '1x1' | '1x2' | '2x1' | '2x2'

interface GridCard {
  id: string
  note: NoteEntry
  size: GridSize
  rowSpan: number
  colSpan: number
}

const sizeCycle: GridSize[] = ['1x1', '1x2', '2x1', '2x2']
const router = useRouter()
const filterInput = ref(store.filterText)
const shuffleSalt = ref(Date.now())

const filteredNotes = computed(() => {
  const filter = store.filterText.toLowerCase()
  return store.notes.filter(n =>
    !filter || n.content.toLowerCase().includes(filter)
  )
})

const gridCards = computed<GridCard[]>(() => {
  return filteredNotes.value
    .map(note => ({
      note,
      score: seededScore(`${note.filename}:${note.content}:${store.filterText}`, shuffleSalt.value),
    }))
    .sort((a, b) => a.score - b.score)
    .map(({ note }, index) => {
      const size = sizeCycle[index % sizeCycle.length]
      return {
        id: note.filename,
        note,
        size,
        colSpan: size.startsWith('2') ? 2 : 1,
        rowSpan: size.endsWith('2') ? 2 : 1,
      }
    })
})

onMounted(async () => {
  const win = getCurrentWindow()
  await win.setResizable(true)
  await win.maximize()
  await props.refreshNotes()
})

watch(
  () => [store.notes.length, store.filterText],
  () => {
    shuffleSalt.value = Date.now()
  }
)

function onFilterInput() {
  store.filterText = filterInput.value
}

function editNote(note: NoteEntry) {
  store.editingNote = note
  router.push('/')
}

function seededScore(value: string, salt: number): number {
  let hash = salt >>> 0
  for (let i = 0; i < value.length; i++) {
    hash ^= value.charCodeAt(i)
    hash = Math.imul(hash, 16777619)
  }
  return hash >>> 0
}
</script>

<template>
  <div class="grid-stage">
    <div class="grid-scroll">
      <div v-if="gridCards.length" class="notes-grid">
        <button
          v-for="card in gridCards"
          :key="card.id"
          class="grid-note card bg-base-200 shadow-md text-left"
          :class="`grid-note-${card.size}`"
          :style="{
            gridColumn: `span ${card.colSpan}`,
            gridRow: `span ${card.rowSpan}`,
          }"
          @click="editNote(card.note)"
        >
          <div class="card-body p-3">
            <div
              class="prose prose-sm max-w-none text-xs"
              v-html="renderMarkdown(card.note.content)"
            />
          </div>
        </button>
      </div>
      <div v-else class="empty-grid text-sm opacity-60">
        no notes
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
.grid-stage {
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.grid-scroll {
  height: 100%;
  overflow: auto;
  padding: 1rem 1rem 4.5rem;
}

.notes-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(11rem, 1fr));
  grid-auto-flow: dense;
  grid-auto-rows: minmax(8.5rem, auto);
  gap: 0.75rem;
  align-items: stretch;
}

.grid-note {
  width: 100%;
  min-width: 0;
  min-height: 100%;
  border-radius: 8px;
  cursor: pointer;
}

.grid-note :deep(.card-body) {
  height: 100%;
  min-width: 0;
}

.grid-note :deep(.prose) {
  overflow-wrap: anywhere;
  word-break: break-word;
}

.empty-grid {
  display: grid;
  height: 100%;
  place-items: center;
}

@media (max-width: 520px) {
  .notes-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
    grid-auto-rows: minmax(7.5rem, auto);
  }
}
</style>
