import { createRouter, createWebHistory } from 'vue-router'
import AddNote from '../views/AddNote.vue'
import FloatingNotes from '../views/FloatingNotes.vue'
import Settings from '../views/Settings.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: AddNote },
    { path: '/floating', component: FloatingNotes },
    { path: '/settings', component: Settings },
  ],
})

export default router
