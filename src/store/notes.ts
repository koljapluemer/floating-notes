import { reactive } from 'vue'

export interface NoteEntry {
  filename: string
  content: string
}

export const store = reactive({
  folderPath: '' as string,
  notes: [] as NoteEntry[],
  editingNote: null as NoteEntry | null,
  filterText: '' as string,
  // Float settings
  minDuration: 12 as number,      // seconds (fast end)
  maxDuration: 25 as number,      // seconds (slow end)
  spawnInterval: 2500 as number,  // ms between spawns
})
