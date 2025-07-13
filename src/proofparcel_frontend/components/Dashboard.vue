<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
      <h1 class="text-3xl font-bold text-gray-900">Welcome to ProofParcel</h1>
      <p class="mt-2 text-gray-600">
        Secure on-chain delivery confirmation for physical goods using blockchain technology.
      </p>
    </div>

    <!-- Stats Grid -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <div class="card">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="w-8 h-8 bg-primary-100 rounded-lg flex items-center justify-center">
              <svg class="w-5 h-5 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500">Total Deliveries</p>
            <p class="text-2xl font-bold text-gray-900">{{ stats.totalDeliveries }}</p>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="w-8 h-8 bg-success-100 rounded-lg flex items-center justify-center">
              <svg class="w-5 h-5 text-success-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500">Confirmed Deliveries</p>
            <p class="text-2xl font-bold text-gray-900">{{ stats.confirmedDeliveries }}</p>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="w-8 h-8 bg-warning-100 rounded-lg flex items-center justify-center">
              <svg class="w-5 h-5 text-warning-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1" />
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500">Escrow Balance</p>
            <p class="text-2xl font-bold text-gray-900">{{ formatAmount(stats.escrowBalance) }} ICP</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Quick Actions -->
    <div class="card">
      <h2 class="text-lg font-semibold text-gray-900 mb-4">Quick Actions</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <router-link 
          to="/create" 
          class="flex items-center p-4 border border-gray-200 rounded-lg hover:border-primary-300 hover:bg-primary-50 transition-colors duration-200"
        >
          <div class="flex-shrink-0">
            <div class="w-10 h-10 bg-primary-100 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <h3 class="text-sm font-medium text-gray-900">Create New Delivery</h3>
            <p class="text-sm text-gray-500">Start a new delivery transaction</p>
          </div>
        </router-link>

        <router-link 
          to="/my-deliveries" 
          class="flex items-center p-4 border border-gray-200 rounded-lg hover:border-primary-300 hover:bg-primary-50 transition-colors duration-200"
        >
          <div class="flex-shrink-0">
            <div class="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <h3 class="text-sm font-medium text-gray-900">View My Deliveries</h3>
            <p class="text-sm text-gray-500">Track your delivery status</p>
          </div>
        </router-link>

        <router-link 
          to="/my-nfts" 
          class="flex items-center p-4 border border-gray-200 rounded-lg hover:border-primary-300 hover:bg-primary-50 transition-colors duration-200"
        >
          <div class="flex-shrink-0">
            <div class="w-10 h-10 bg-purple-100 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zM21 5a2 2 0 00-2-2h-4a2 2 0 00-2 2v12a4 4 0 004 4h4a2 2 0 002-2V5z" />
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <h3 class="text-sm font-medium text-gray-900">My Delivery NFTs</h3>
            <p class="text-sm text-gray-500">View your delivery certificates</p>
          </div>
        </router-link>

        <div class="flex items-center p-4 border border-gray-200 rounded-lg bg-gray-50">
          <div class="flex-shrink-0">
            <div class="w-10 h-10 bg-gray-100 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <h3 class="text-sm font-medium text-gray-500">How It Works</h3>
            <p class="text-sm text-gray-400">Learn about the process</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Recent Deliveries -->
    <div class="card">
      <h2 class="text-lg font-semibold text-gray-900 mb-4">Recent Deliveries</h2>
      <div v-if="recentDeliveries.length === 0" class="text-center py-8">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
        </svg>
        <h3 class="mt-2 text-sm font-medium text-gray-900">No deliveries yet</h3>
        <p class="mt-1 text-sm text-gray-500">Get started by creating your first delivery.</p>
        <div class="mt-6">
          <router-link to="/create" class="btn-primary">
            Create Delivery
          </router-link>
        </div>
      </div>
      <div v-else class="space-y-4">
        <div 
          v-for="delivery in recentDeliveries" 
          :key="delivery.id"
          class="flex items-center justify-between p-4 border border-gray-200 rounded-lg hover:bg-gray-50"
        >
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <div class="w-8 h-8 bg-primary-100 rounded-lg flex items-center justify-center">
                <span class="text-primary-600 font-medium text-sm">{{ delivery.id.slice(0, 2) }}</span>
              </div>
            </div>
            <div class="ml-4">
              <p class="text-sm font-medium text-gray-900">{{ delivery.description }}</p>
              <p class="text-sm text-gray-500">{{ formatAmount(delivery.amount) }} ICP</p>
            </div>
          </div>
          <div class="flex items-center space-x-2">
            <span :class="getStatusClass(delivery.status)" class="status-badge">
              {{ getStatusText(delivery.status) }}
            </span>
            <router-link :to="`/delivery/${delivery.id}`" class="text-primary-600 hover:text-primary-700 text-sm font-medium">
              View
            </router-link>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

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

interface Stats {
  totalDeliveries: number
  confirmedDeliveries: number
  escrowBalance: number
}

// Reactive data
const stats = ref<Stats>({
  totalDeliveries: 0,
  confirmedDeliveries: 0,
  escrowBalance: 0
})

const recentDeliveries = ref<Delivery[]>([])

// Methods
const formatAmount = (amount: number): string => {
  return (amount / 100000000).toFixed(4) // Convert from e8s to ICP
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

const loadDashboardData = async () => {
  try {
    // TODO: Replace with actual canister calls
    // For now, using mock data
    stats.value = {
      totalDeliveries: 12,
      confirmedDeliveries: 8,
      escrowBalance: 2500000000 // 25 ICP in e8s
    }
    
    recentDeliveries.value = [
      {
        id: 'del-001',
        seller: 'seller-1',
        buyer: 'buyer-1',
        amount: 100000000, // 1 ICP
        description: 'Electronics Package',
        status: 'Confirmed',
        created_at: Date.now() / 1000
      },
      {
        id: 'del-002',
        seller: 'seller-2',
        buyer: 'buyer-2',
        amount: 50000000, // 0.5 ICP
        description: 'Books Delivery',
        status: 'InTransit',
        created_at: Date.now() / 1000
      }
    ]
  } catch (error) {
    console.error('Error loading dashboard data:', error)
  }
}

// Lifecycle
onMounted(() => {
  loadDashboardData()
})
</script> 