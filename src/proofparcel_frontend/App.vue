<template>
  <div id="app" class="min-h-screen bg-gray-50 dark:bg-gray-900 dark:text-gray-100">
    <!-- Navigation -->
    <nav class="bg-white dark:bg-gray-800 shadow-sm border-b border-gray-200 dark:border-gray-700">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <router-link to="/" class="flex items-center">
              <div class="flex-shrink-0">
                <div class="w-8 h-8 bg-primary-600 rounded-lg flex items-center justify-center">
                  <span class="text-white font-bold text-sm">P</span>
                </div>
              </div>
              <div class="ml-3">
                <h1 class="text-xl font-bold text-gray-900 dark:text-gray-100">ProofParcel</h1>
              </div>
            </router-link>
          </div>
          
          <div class="flex items-center space-x-4">
            <router-link 
              to="/" 
              class="text-gray-600 dark:text-gray-200 hover:text-gray-900 dark:hover:text-white px-3 py-2 rounded-md text-sm font-medium"
              active-class="text-primary-600 dark:text-primary-400"
            >
              Dashboard
            </router-link>
            <router-link 
              to="/create" 
              class="text-gray-600 dark:text-gray-200 hover:text-gray-900 dark:hover:text-white px-3 py-2 rounded-md text-sm font-medium"
              active-class="text-primary-600 dark:text-primary-400"
            >
              Create Delivery
            </router-link>
            <router-link 
              to="/my-deliveries" 
              class="text-gray-600 dark:text-gray-200 hover:text-gray-900 dark:hover:text-white px-3 py-2 rounded-md text-sm font-medium"
              active-class="text-primary-600 dark:text-primary-400"
            >
              My Deliveries
            </router-link>
            <router-link 
              to="/my-nfts" 
              class="text-gray-600 dark:text-gray-200 hover:text-gray-900 dark:hover:text-white px-3 py-2 rounded-md text-sm font-medium"
              active-class="text-primary-600 dark:text-primary-400"
            >
              My NFTs
            </router-link>
            <router-link
              v-if="plugConnected"
              to="/profile"
              class="text-gray-600 dark:text-gray-200 hover:text-gray-900 dark:hover:text-white px-3 py-2 rounded-md text-sm font-medium"
              active-class="text-primary-600 dark:text-primary-400"
            >
              Profile
            </router-link>
            <!-- Dark mode toggle button -->
            <button @click="toggleDarkMode" class="ml-4 p-2 rounded-full bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors" :aria-label="isDark ? 'Switch to light mode' : 'Switch to dark mode'">
              <span v-if="isDark" class="text-yellow-400">☀️</span>
              <span v-else class="text-gray-800">🌙</span>
            </button>
            <!-- Plug wallet connect button -->
            <div class="ml-4">
              <button v-if="!plugConnected" @click="connectPlug" class="px-3 py-2 rounded-md bg-primary-600 text-white hover:bg-primary-700 transition-colors text-sm font-medium">
                Connect Plug
              </button>
              <div v-else class="flex items-center space-x-2">
                <span class="text-xs font-mono bg-gray-200 dark:bg-gray-700 px-2 py-1 rounded">{{ plugPrincipalShort }}</span>
                <button @click="disconnectPlug" class="px-2 py-1 rounded bg-red-500 text-white text-xs hover:bg-red-600">Disconnect</button>
              </div>
            </div>
            <!-- Notifications -->
            <div class="relative ml-4">
              <button @click="showNotifications = !showNotifications" class="p-2 rounded-full bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors" aria-label="Notifications">
                <svg class="h-5 w-5 text-gray-700 dark:text-gray-200" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" /></svg>
                <span v-if="notifications.length" class="absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full px-1">{{ notifications.length }}</span>
              </button>
              <div v-if="showNotifications" class="absolute right-0 mt-2 w-72 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded shadow-lg z-50">
                <div v-if="notifications.length === 0" class="p-4 text-gray-500 text-sm">No notifications</div>
                <ul v-else class="max-h-64 overflow-y-auto divide-y divide-gray-200 dark:divide-gray-700">
                  <li v-for="(n, i) in notifications" :key="i" class="p-3 text-sm flex items-start">
                    <span :class="[n.type === 'success' ? 'text-green-600' : n.type === 'error' ? 'text-red-600' : 'text-gray-700']">●</span>
                    <span class="ml-2 flex-1">{{ n.message }}</span>
                    <span class="ml-2 text-xs text-gray-400">{{ new Date(n.time).toLocaleTimeString() }}</span>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </div>
    </nav>

    <!-- Main Content -->
    <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
      <div class="px-4 py-6 sm:px-0">
        <router-view />
      </div>
    </main>

    <!-- Footer -->
    <footer class="bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 mt-12">
      <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
        <div class="text-center text-gray-500 dark:text-gray-400 text-sm">
          <p>ProofParcel - Secure On-Chain Delivery Confirmation</p>
          <p class="mt-1">Built on Internet Computer Blockchain</p>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, computed, provide, watch } from 'vue'
import { Principal } from '@dfinity/principal'

const isDark = ref(false)

function setDarkClass(val: boolean) {
  const html = document.documentElement
  if (val) {
    html.classList.add('dark')
  } else {
    html.classList.remove('dark')
  }
}

function toggleDarkMode() {
  isDark.value = !isDark.value
  setDarkClass(isDark.value)
  localStorage.setItem('theme', isDark.value ? 'dark' : 'light')
}

onMounted(() => {
  // Load theme from localStorage or system preference
  const saved = localStorage.getItem('theme')
  if (saved) {
    isDark.value = saved === 'dark'
  } else {
    isDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches
  }
  setDarkClass(isDark.value)
  // Try to auto-connect Plug if already authorized
  checkPlugConnection()
})

// Plug wallet integration
const plugConnected = ref(false)
const plugPrincipal = ref<string | null>(null)
const plugPrincipalShort = computed(() => {
  if (!plugPrincipal.value) return ''
  return plugPrincipal.value.slice(0, 8) + '...' + plugPrincipal.value.slice(-4)
})

async function fetchBackendNotifications() {
  if (!plugPrincipal.value) return
  // @ts-ignore
  const agent = window.ic?.plug?.agent
  if (!agent) return
  const backendCanisterId = import.meta.env.VITE_BACKEND_CANISTER_ID || '<CANISTER_ID>'
  const backend = await window.ic.plug.createActor({ canisterId: backendCanisterId, interfaceFactory: undefined })
  const backendNotifs = await backend.get_notifications(Principal.fromText(plugPrincipal.value))
  // Merge backend notifications (avoid duplicates by id)
  backendNotifs.forEach((n: any) => {
    if (!notifications.value.some(local => local.id === n.id)) {
      notifications.value.unshift({
        id: n.id,
        message: n.message,
        type: n.notif_type,
        time: n.timestamp * 1000 // backend is seconds, JS is ms
      })
    }
  })
  // Keep only the latest 20
  notifications.value = notifications.value.slice(0, 20)
}
watch(plugPrincipal, fetchBackendNotifications)

async function connectPlug() {
  if (!(window as any).ic?.plug) {
    alert('Plug wallet extension not found! Please install Plug to connect.')
    return
  }
  try {
    const connected = await (window as any).ic.plug.requestConnect({ whitelist: [] })
    if (connected) {
      const principal = await (window as any).ic.plug.getPrincipal()
      plugPrincipal.value = principal.toString()
      plugConnected.value = true
    }
  } catch (e) {
    alert('Failed to connect Plug: ' + (e as any).message)
  }
}

function disconnectPlug() {
  plugConnected.value = false
  plugPrincipal.value = null
}

function checkPlugConnection() {
  if ((window as any).ic?.plug?.agent) {
    (window as any).ic.plug.getPrincipal().then((principal: any) => {
      plugPrincipal.value = principal.toString()
      plugConnected.value = true
    })
  }
}

const notifications = ref<Array<{ id: string, message: string, type: string, time: number }>>([])
const showNotifications = ref(false)
function addNotification(message: string, type: string = 'info') {
  notifications.value.unshift({ id: Math.random().toString(36).slice(2), message, type, time: Date.now() })
  if (notifications.value.length > 20) notifications.value.pop()
}
provide('addNotification', addNotification)
</script>

<style scoped>
/* Component-specific styles */
</style> 