import { createRouter, createWebHistory } from 'vue-router'
import AddNote from '../views/AddNote.vue'
import FloatingNotes from '../views/FloatingNotes.vue'
import GridNotes from '../views/GridNotes.vue'
import Settings from '../views/Settings.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: AddNote },
    { path: '/floating', component: FloatingNotes },
    { path: '/grid', component: GridNotes },
    { path: '/settings', component: Settings },
  ],
})

export default router
