import { createRouter, createWebHistory } from 'vue-router'
import FloatingNotes from '../views/FloatingNotes.vue'
import Settings from '../views/Settings.vue'
import { store } from '../store/notes'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: FloatingNotes },
    { path: '/settings', component: Settings },
  ],
})

router.beforeEach((to) => {
  if (to.path === '/' && !store.folderPath) return '/settings'
})

export default router
