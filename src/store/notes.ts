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
})
