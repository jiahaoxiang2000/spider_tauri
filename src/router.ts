import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Home from './views/Home.vue'; // Adjust the import path as necessary
import About from './views/About.vue'; // Adjust the import path as necessary

const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        name: 'Home',
        component: Home,
    },
    {
        path: '/about',
        name: 'About',
        component: About,
      },
    // Define more routes as needed
];

const router = createRouter({
    history: createWebHistory('/'),
    routes,
});

export default router;