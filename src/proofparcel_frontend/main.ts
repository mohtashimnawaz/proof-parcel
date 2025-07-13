import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'
import './index.css'

// Import components
import Dashboard from './components/Dashboard.vue'
import CreateDelivery from './components/CreateDelivery.vue'
import DeliveryDetails from './components/DeliveryDetails.vue'
import ConfirmDelivery from './components/ConfirmDelivery.vue'
import MyDeliveries from './components/MyDeliveries.vue'
import MyNFTs from './components/MyNFTs.vue'
import Profile from './components/Profile.vue'

// Create router
const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: Dashboard },
    { path: '/create', component: CreateDelivery },
    { path: '/delivery/:id', component: DeliveryDetails },
    { path: '/confirm/:id', component: ConfirmDelivery },
    { path: '/my-deliveries', component: MyDeliveries },
    { path: '/my-nfts', component: MyNFTs },
    { path: '/profile', component: Profile },
  ]
})

// Create and mount app
const app = createApp(App)
app.use(router)
app.mount('#app') 