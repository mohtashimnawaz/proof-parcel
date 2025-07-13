<template>
  <div class="max-w-4xl mx-auto">
    <div v-if="isLoading" class="text-center py-12">
      <svg class="animate-spin h-8 w-8 text-primary-600 mx-auto" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
      <p class="mt-4 text-gray-600">Loading delivery details...</p>
    </div>

    <div v-else-if="!delivery" class="text-center py-12">
      <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
      </svg>
      <h3 class="mt-2 text-sm font-medium text-gray-900">Delivery not found</h3>
      <p class="mt-1 text-sm text-gray-500">The delivery you're looking for doesn't exist.</p>
      <div class="mt-6">
        <router-link to="/my-deliveries" class="btn-primary">
          Back to My Deliveries
        </router-link>
      </div>
    </div>

    <div v-else class="space-y-6">
      <!-- Header -->
      <div class="card">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-2xl font-bold text-gray-900">Delivery Details</h1>
            <p class="mt-1 text-gray-600">ID: {{ delivery.id }}</p>
          </div>
          <div class="flex items-center space-x-3">
            <span :class="getStatusClass(delivery.status)" class="status-badge">
              {{ getStatusText(delivery.status) }}
            </span>
            <router-link to="/my-deliveries" class="btn-secondary">
              Back to List
            </router-link>
          </div>
        </div>
      </div>

      <!-- Delivery Information -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Basic Info -->
        <div class="card">
          <h2 class="text-lg font-semibold text-gray-900 mb-4">Delivery Information</h2>
          <dl class="space-y-4">
            <div>
              <dt class="text-sm font-medium text-gray-500">Description</dt>
              <dd class="mt-1 text-sm text-gray-900">{{ delivery.description }}</dd>
            </div>
            <div>
              <dt class="text-sm font-medium text-gray-500">Amount</dt>
              <dd class="mt-1 text-sm text-gray-900">{{ formatAmount(delivery.amount) }} ICP</dd>
            </div>
            <div>
              <dt class="text-sm font-medium text-gray-500">Created</dt>
              <dd class="mt-1 text-sm text-gray-900">{{ formatDate(delivery.created_at) }}</dd>
            </div>
            <div v-if="delivery.confirmed_at">
              <dt class="text-sm font-medium text-gray-500">Confirmed</dt>
              <dd class="mt-1 text-sm text-gray-900">{{ formatDate(delivery.confirmed_at) }}</dd>
            </div>
            <div v-if="delivery.escrow_released_at">
              <dt class="text-sm font-medium text-gray-500">Escrow Released</dt>
              <dd class="mt-1 text-sm text-gray-900">{{ formatDate(delivery.escrow_released_at) }}</dd>
            </div>
          </dl>
        </div>

        <!-- Participants -->
        <div class="card">
          <h2 class="text-lg font-semibold text-gray-900 mb-4">Participants</h2>
          <dl class="space-y-4">
            <div>
              <dt class="text-sm font-medium text-gray-500">Seller</dt>
              <dd class="mt-1 text-sm font-mono text-gray-900">{{ delivery.seller }}</dd>
            </div>
            <div>
              <dt class="text-sm font-medium text-gray-500">Buyer</dt>
              <dd class="mt-1 text-sm font-mono text-gray-900">{{ delivery.buyer }}</dd>
            </div>
          </dl>
        </div>
      </div>

      <!-- OTP Section -->
      <div v-if="delivery.status === 'Delivered' && delivery.otp" class="card">
        <h2 class="text-lg font-semibold text-gray-900 mb-4">Delivery OTP</h2>
        <div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
          <div class="flex items-center">
            <svg class="w-5 h-5 text-yellow-400 mr-2" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd"></path>
            </svg>
            <span class="text-sm font-medium text-yellow-800">OTP Generated</span>
          </div>
          <p class="mt-2 text-sm text-yellow-700">
            The delivery OTP has been generated. Share this code with the buyer to confirm delivery.
          </p>
          <div class="mt-3">
            <div class="bg-white border border-yellow-300 rounded-lg p-3">
              <p class="text-center text-2xl font-mono font-bold text-gray-900 tracking-wider">
                {{ delivery.otp }}
              </p>
            </div>
            <p class="mt-2 text-xs text-yellow-600">
              Expires: {{ formatDate(delivery.otp_expires_at || 0) }}
            </p>
          </div>
        </div>
      </div>

      <!-- Actions -->
      <div class="card">
        <h2 class="text-lg font-semibold text-gray-900 mb-4">Actions</h2>
        <div class="space-y-4">
          <!-- Start Delivery -->
          <div v-if="delivery.status === 'Pending'" class="flex items-center justify-between p-4 border border-gray-200 rounded-lg">
            <div>
              <h3 class="text-sm font-medium text-gray-900">Start Delivery</h3>
              <p class="text-sm text-gray-500">Mark the delivery as in transit</p>
            </div>
            <button
              @click="startDelivery"
              :disabled="isActionLoading"
              class="btn-primary disabled:opacity-50"
            >
              Start Delivery
            </button>
          </div>

          <!-- Generate OTP -->
          <div v-if="delivery.status === 'InTransit'" class="flex items-center justify-between p-4 border border-gray-200 rounded-lg">
            <div>
              <h3 class="text-sm font-medium text-gray-900">Generate OTP</h3>
              <p class="text-sm text-gray-500">Generate OTP when delivery is ready for confirmation</p>
            </div>
            <button
              @click="generateOTP"
              :disabled="isActionLoading"
              class="btn-warning disabled:opacity-50"
            >
              Generate OTP
            </button>
          </div>

          <!-- Release Escrow -->
          <div v-if="delivery.status === 'Confirmed'" class="flex items-center justify-between p-4 border border-gray-200 rounded-lg">
            <div>
              <h3 class="text-sm font-medium text-gray-900">Release Escrow</h3>
              <p class="text-sm text-gray-500">Release funds from escrow to your account</p>
            </div>
            <button
              @click="releaseEscrow"
              :disabled="isActionLoading"
              class="btn-success disabled:opacity-50"
            >
              Release Escrow
            </button>
          </div>

          <!-- Confirmed Status -->
          <div v-if="delivery.status === 'EscrowReleased'" class="flex items-center justify-between p-4 border border-green-200 rounded-lg bg-green-50">
            <div>
              <h3 class="text-sm font-medium text-green-900">Delivery Complete</h3>
              <p class="text-sm text-green-700">Escrow has been released and delivery is complete</p>
            </div>
            <div class="flex items-center">
              <svg class="w-5 h-5 text-green-500 mr-2" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
              </svg>
              <span class="text-sm font-medium text-green-900">Completed</span>
            </div>
          </div>
        </div>
      </div>

      <!-- NFT Certificate -->
      <div v-if="delivery.status === 'Confirmed' || delivery.status === 'EscrowReleased'" class="card">
        <h2 class="text-lg font-semibold text-gray-900 mb-4">Delivery NFT Certificate</h2>
        <div class="bg-purple-50 border border-purple-200 rounded-lg p-4">
          <div class="flex items-center">
            <svg class="w-5 h-5 text-purple-500 mr-2" fill="currentColor" viewBox="0 0 20 20">
              <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
            <span class="text-sm font-medium text-purple-900">NFT Minted</span>
          </div>
          <p class="mt-2 text-sm text-purple-700">
            A delivery certificate NFT has been minted for this transaction.
          </p>
          <div class="mt-3">
            <router-link to="/my-nfts" class="btn-secondary">
              View My NFTs
            </router-link>
          </div>
        </div>
      </div>
    </div>

    <!-- Error Alert -->
    <div v-if="error" class="fixed top-4 right-4 bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded z-50">
      <div class="flex items-center">
        <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"></path>
        </svg>
        <span class="text-sm font-medium">{{ error }}</span>
        <button @click="error = ''" class="ml-auto">
          <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path>
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()

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
const delivery = ref<Delivery | null>(null)
const isLoading = ref(true)
const isActionLoading = ref(false)
const error = ref('')

// Methods
const formatAmount = (amount: number): string => {
  return (amount / 100000000).toFixed(4)
}

const formatDate = (timestamp: number): string => {
  return new Date(timestamp * 1000).toLocaleString()
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

const loadDelivery = async () => {
  try {
    const deliveryId = route.params.id as string
    
    // TODO: Replace with actual canister call
    // const result = await canister.get_delivery(deliveryId)
    
    // Mock data for now
    await new Promise(resolve => setTimeout(resolve, 1000))
    delivery.value = {
      id: deliveryId,
      seller: 'seller-1',
      buyer: 'buyer-1',
      amount: 100000000, // 1 ICP
      description: 'Electronics Package',
      status: 'InTransit',
      created_at: Date.now() / 1000 - 86400, // 1 day ago
      otp: undefined,
      otp_expires_at: undefined,
      confirmed_at: undefined,
      escrow_released_at: undefined
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to load delivery'
  } finally {
    isLoading.value = false
  }
}

const startDelivery = async () => {
  if (!delivery.value) return
  
  isActionLoading.value = true
  error.value = ''
  
  try {
    // TODO: Replace with actual canister call
    // await canister.start_delivery(delivery.value.id)
    
    // Mock response
    await new Promise(resolve => setTimeout(resolve, 1000))
    delivery.value.status = 'InTransit'
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to start delivery'
  } finally {
    isActionLoading.value = false
  }
}

const generateOTP = async () => {
  if (!delivery.value) return
  
  isActionLoading.value = true
  error.value = ''
  
  try {
    // TODO: Replace with actual canister call
    // const result = await canister.generate_delivery_otp(delivery.value.id)
    
    // Mock response
    await new Promise(resolve => setTimeout(resolve, 1000))
    const mockOTP = Math.floor(100000 + Math.random() * 900000).toString()
    delivery.value.otp = mockOTP
    delivery.value.otp_expires_at = Date.now() / 1000 + 3600 // 1 hour from now
    delivery.value.status = 'Delivered'
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to generate OTP'
  } finally {
    isActionLoading.value = false
  }
}

const releaseEscrow = async () => {
  if (!delivery.value) return
  
  isActionLoading.value = true
  error.value = ''
  
  try {
    // TODO: Replace with actual canister call
    // await canister.release_escrow(delivery.value.id)
    
    // Mock response
    await new Promise(resolve => setTimeout(resolve, 1000))
    delivery.value.status = 'EscrowReleased'
    delivery.value.escrow_released_at = Date.now() / 1000
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to release escrow'
  } finally {
    isActionLoading.value = false
  }
}

// Lifecycle
onMounted(() => {
  loadDelivery()
})
</script> 