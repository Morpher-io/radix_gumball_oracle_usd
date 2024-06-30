import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/pull-vs-push-oracles',
      name: 'pullvspushoracles',
      component: () => import("../views/PullVsPush.vue")
    },
    {
      path: '/radix/setup',
      name: 'radixsetup',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/RadixSetup.vue')
    },
    {
      path: '/radix/example',
      name: 'radixexample',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/RadixExample.vue')
    }
  ]
})

export default router
