<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">My Deliveries</h1>
        <p class="mt-1 text-gray-600">Track all your delivery transactions</p>
      </div>
      <router-link to="/create" class="btn-primary">
        Create New Delivery
      </router-link>
    </div>

    <!-- Filters -->
    <div class="card">
      <div class="flex flex-col sm:flex-row gap-4">
        <div class="flex-1">
          <label for="search" class="block text-sm font-medium text-gray-700 mb-1">Search</label>
          <input
            id="search"
            v-model="filters.search"
            type="text"
            class="input-field"
            placeholder="Search by description or ID..."
          />
        </div>
        <div class="sm:w-48">
          <label for="status" class="block text-sm font-medium text-gray-700 mb-1">Status</label>
          <select
            id="status"
            v-model="filters.status"
            class="input-field"
          >
            <option value="">All Status</option>
            <option value="Pending">Pending</option>
            <option value="InTransit">In Transit</option>
            <option value="Delivered">Delivered</option>
            <option value="Confirmed">Confirmed</option>
            <option value="EscrowReleased">Released</option>
          </select>
        </div>
        <div class="sm:w-48">
          <label for="role" class="block text-sm font-medium text-gray-700 mb-1">Role</label>
          <select
            id="role"
            v-model="filters.role"
            class="input-field"
          >
            <option value="">All Roles</option>
            <option value="seller">As Seller</option>
            <option value="buyer">As Buyer</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Deliveries List -->
    <div class="card">
      <div v-if="isLoading" class="text-center py-12">
        <svg class="animate-spin h-8 w-8 text-primary-600 mx-auto" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <p class="mt-4 text-gray-600">Loading deliveries...</p>
      </div>

      <div v-else-if="filteredDeliveries.length === 0" class="text-center py-12">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
        </svg>
        <h3 class="mt-2 text-sm font-medium text-gray-900">No deliveries found</h3>
        <p class="mt-1 text-sm text-gray-500">
          {{ filters.search || filters.status || filters.role ? 'Try adjusting your filters.' : 'Get started by creating your first delivery.' }}
        </p>
        <div class="mt-6">
          <router-link to="/create" class="btn-primary">
            Create Delivery
          </router-link>
        </div>
      </div>

      <div v-else class="space-y-4">
        <div
          v-for="delivery in filteredDeliveries"
          :key="delivery.id"
          class="border border-gray-200 rounded-lg p-4 hover:bg-gray-50 transition-colors duration-200"
        >
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-4">
              <div class="flex-shrink-0">
                <div class="w-10 h-10 bg-primary-100 rounded-lg flex items-center justify-center">
                  <span class="text-primary-600 font-medium text-sm">{{ delivery.id.slice(0, 2) }}</span>
                </div>
              </div>
              <div>
                <h3 class="text-sm font-medium text-gray-900">{{ delivery.description }}</h3>
                <p class="text-sm text-gray-500">ID: {{ delivery.id }}</p>
                <p class="text-sm text-gray-500">
                  {{ formatAmount(delivery.amount) }} ICP • 
                  {{ formatDate(delivery.created_at) }}
                </p>
              </div>
            </div>
            <div class="flex items-center space-x-3">
              <span :class="getStatusClass(delivery.status)" class="status-badge">
                {{ getStatusText(delivery.status) }}
              </span>
              <router-link :to="`/delivery/${delivery.id}`" class="text-primary-600 hover:text-primary-700 text-sm font-medium">
                View Details
              </router-link>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

// Types
interface Delivery {
  id: string
  seller: string
  buyer: string
  amount: number
  description: string
  status: string
  created_at: number
  otp?: string
  otp_expires_at?: number
  confirmed_at?: number
  escrow_released_at?: number
}

// Reactive data
const deliveries = ref<Delivery[]>([])
const isLoading = ref(true)
const filters = ref({
  search: '',
  status: '',
  role: ''
})

// Computed
const filteredDeliveries = computed(() => {
  let filtered = deliveries.value

  // Search filter
  if (filters.value.search) {
    const search = filters.value.search.toLowerCase()
    filtered = filtered.filter(delivery =>
      delivery.description.toLowerCase().includes(search) ||
      delivery.id.toLowerCase().includes(search)
    )
  }

  // Status filter
  if (filters.value.status) {
    filtered = filtered.filter(delivery => delivery.status === filters.value.status)
  }

  // Role filter
  if (filters.value.role) {
    // TODO: Implement role filtering based on current user
    // For now, just return all deliveries
  }

  return filtered
})

// Methods
const formatAmount = (amount: number): string => {
  return (amount / 100000000).toFixed(4)
}

const formatDate = (timestamp: number): string => {
  return new Date(timestamp * 1000).toLocaleDateString()
}

const getStatusClass = (status: string): string => {
  const statusMap: Record<string, string> = {
    'Pending': 'status-pending',
    'InTransit': 'status-intransit',
    'Delivered': 'status-delivered',
    'Confirmed': 'status-confirmed',
    'EscrowReleased': 'status-released'
  }
  return statusMap[status] || 'status-pending'
}

const getStatusText = (status: string): string => {
  const statusMap: Record<string, string> = {
    'Pending': 'Pending',
    'InTransit': 'In Transit',
    'Delivered': 'Delivered',
    'Confirmed': 'Confirmed',
    'EscrowReleased': 'Released'
  }
  return statusMap[status] || 'Unknown'
}

const loadDeliveries = async () => {
  try {
    // TODO: Replace with actual canister calls
    // const sellerDeliveries = await canister.get_deliveries_by_seller(currentUser)
    // const buyerDeliveries = await canister.get_deliveries_by_buyer(currentUser)
    
    // Mock data for now
    await new Promise(resolve => setTimeout(resolve, 1000))
    deliveries.value = [
      {
        id: 'del-001',
        seller: 'seller-1',
        buyer: 'buyer-1',
        amount: 100000000, // 1 ICP
        description: 'Electronics Package',
        status: 'Confirmed',
        created_at: Date.now() / 1000 - 86400 // 1 day ago
      },
      {
        id: 'del-002',
        seller: 'seller-2',
        buyer: 'buyer-2',
        amount: 50000000, // 0.5 ICP
        description: 'Books Delivery',
        status: 'InTransit',
        created_at: Date.now() / 1000 - 3600 // 1 hour ago
      },
      {
        id: 'del-003',
        seller: 'seller-3',
        buyer: 'buyer-3',
        amount: 200000000, // 2 ICP
        description: 'Furniture Set',
        status: 'Pending',
        created_at: Date.now() / 1000
      }
    ]
  } catch (error) {
    console.error('Error loading deliveries:', error)
  } finally {
    isLoading.value = false
  }
}

// Lifecycle
onMounted(() => {
  loadDeliveries()
})
</script> 