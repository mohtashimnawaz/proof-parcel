<template>
  <div class="space-y-6">
    <!-- Header -->
    <div>
      <h1 class="text-2xl font-bold text-gray-900">My Delivery NFTs</h1>
      <p class="mt-1 text-gray-600">Your delivery certificate NFTs from confirmed deliveries</p>
    </div>

    <!-- NFTs Grid -->
    <div class="card">
      <div v-if="isLoading" class="text-center py-12">
        <svg class="animate-spin h-8 w-8 text-primary-600 mx-auto" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <p class="mt-4 text-gray-600">Loading NFTs...</p>
      </div>

      <div v-else-if="nfts.length === 0" class="text-center py-12">
        <div class="mx-auto h-24 w-24 bg-purple-100 rounded-full flex items-center justify-center">
          <svg class="h-12 w-12 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zM21 5a2 2 0 00-2-2h-4a2 2 0 00-2 2v12a4 4 0 004 4h4a2 2 0 002-2V5z" />
          </svg>
        </div>
        <h3 class="mt-4 text-lg font-medium text-gray-900">No NFTs yet</h3>
        <p class="mt-2 text-sm text-gray-500">
          You'll receive delivery certificate NFTs when you confirm deliveries.
        </p>
        <div class="mt-6">
          <router-link to="/my-deliveries" class="btn-primary">
            View My Deliveries
          </router-link>
        </div>
      </div>

      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div
          v-for="nft in nfts"
          :key="nft.id"
          class="bg-white border border-gray-200 rounded-lg overflow-hidden hover:shadow-lg transition-shadow duration-200"
        >
          <!-- NFT Header -->
          <div class="bg-gradient-to-r from-purple-500 to-blue-600 px-4 py-3">
            <div class="flex items-center justify-between">
              <h3 class="text-white font-medium">Delivery Certificate</h3>
              <div class="w-8 h-8 bg-white bg-opacity-20 rounded-full flex items-center justify-center">
                <svg class="w-4 h-4 text-white" fill="currentColor" viewBox="0 0 20 20">
                  <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
              </div>
            </div>
          </div>

          <!-- NFT Content -->
          <div class="p-4">
            <div class="space-y-3">
              <div>
                <p class="text-xs font-medium text-gray-500 uppercase tracking-wide">NFT ID</p>
                <p class="text-sm font-mono text-gray-900">{{ nft.id }}</p>
              </div>
              
              <div>
                <p class="text-xs font-medium text-gray-500 uppercase tracking-wide">Delivery ID</p>
                <p class="text-sm font-mono text-gray-900">{{ nft.delivery_id }}</p>
              </div>
              
              <div>
                <p class="text-xs font-medium text-gray-500 uppercase tracking-wide">Description</p>
                <p class="text-sm text-gray-900">{{ getDeliveryDescription(nft.delivery_id) }}</p>
              </div>
              
              <div>
                <p class="text-xs font-medium text-gray-500 uppercase tracking-wide">Amount</p>
                <p class="text-sm text-gray-900">{{ getDeliveryAmount(nft.delivery_id) }} ICP</p>
              </div>
              
              <div>
                <p class="text-xs font-medium text-gray-500 uppercase tracking-wide">Minted</p>
                <p class="text-sm text-gray-900">{{ formatDate(nft.minted_at) }}</p>
              </div>
            </div>

            <!-- Actions -->
            <div class="mt-4 pt-4 border-t border-gray-200">
              <div class="flex space-x-2">
                <button
                  @click="viewNFTDetails(nft)"
                  class="flex-1 btn-secondary text-sm py-2"
                >
                  View Details
                </button>
                <button
                  @click="downloadNFT(nft)"
                  class="flex-1 btn-primary text-sm py-2"
                >
                  Download
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- NFT Details Modal -->
    <div v-if="selectedNFT" class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
      <div class="relative top-10 mx-auto p-5 border w-full max-w-2xl shadow-lg rounded-md bg-white">
        <div class="mt-3">
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-lg font-medium text-gray-900">NFT Details</h3>
            <button @click="selectedNFT = null" class="text-gray-400 hover:text-gray-600">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
            </button>
          </div>
          
          <div class="space-y-4">
            <div class="bg-gray-50 rounded-lg p-4">
              <h4 class="text-sm font-medium text-gray-900 mb-2">Metadata</h4>
              <pre class="text-xs text-gray-600 whitespace-pre-wrap">{{ selectedNFT.metadata }}</pre>
            </div>
            
            <div class="grid grid-cols-2 gap-4">
              <div>
                <p class="text-sm font-medium text-gray-500">NFT ID</p>
                <p class="text-sm text-gray-900 font-mono">{{ selectedNFT.id }}</p>
              </div>
              <div>
                <p class="text-sm font-medium text-gray-500">Delivery ID</p>
                <p class="text-sm text-gray-900 font-mono">{{ selectedNFT.delivery_id }}</p>
              </div>
              <div>
                <p class="text-sm font-medium text-gray-500">Owner</p>
                <p class="text-sm text-gray-900 font-mono">{{ selectedNFT.owner }}</p>
              </div>
              <div>
                <p class="text-sm font-medium text-gray-500">Minted</p>
                <p class="text-sm text-gray-900">{{ formatDate(selectedNFT.minted_at) }}</p>
              </div>
            </div>
          </div>
          
          <div class="mt-6 flex justify-end space-x-3">
            <button @click="selectedNFT = null" class="btn-secondary">
              Close
            </button>
            <button @click="downloadNFT(selectedNFT)" class="btn-primary">
              Download Certificate
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

// Types
interface DeliveryNFT {
  id: string
  delivery_id: string
  owner: string
  metadata: string
  minted_at: number
}

interface Delivery {
  id: string
  description: string
  amount: number
}

// Reactive data
const nfts = ref<DeliveryNFT[]>([])
const deliveries = ref<Delivery[]>([])
const isLoading = ref(true)
const selectedNFT = ref<DeliveryNFT | null>(null)

// Methods
const formatDate = (timestamp: number): string => {
  return new Date(timestamp * 1000).toLocaleString()
}

const getDeliveryDescription = (deliveryId: string): string => {
  const delivery = deliveries.value.find(d => d.id === deliveryId)
  return delivery?.description || 'Unknown'
}

const getDeliveryAmount = (deliveryId: string): string => {
  const delivery = deliveries.value.find(d => d.id === deliveryId)
  return delivery ? (delivery.amount / 100000000).toFixed(4) : '0.0000'
}

const viewNFTDetails = (nft: DeliveryNFT) => {
  selectedNFT.value = nft
}

const downloadNFT = (nft: DeliveryNFT) => {
  // Create a JSON file with NFT data
  const data = {
    nft_id: nft.id,
    delivery_id: nft.delivery_id,
    owner: nft.owner,
    metadata: JSON.parse(nft.metadata),
    minted_at: new Date(nft.minted_at * 1000).toISOString(),
    certificate_type: 'Delivery Confirmation',
    platform: 'ProofParcel'
  }
  
  const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `delivery-certificate-${nft.id}.json`
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
}

const loadNFTs = async () => {
  try {
    // TODO: Replace with actual canister calls
    // const result = await canister.get_nfts_by_owner(currentUser)
    
    // Mock data for now
    await new Promise(resolve => setTimeout(resolve, 1000))
    nfts.value = [
      {
        id: 'nft-001',
        delivery_id: 'del-001',
        owner: 'buyer-1',
        metadata: JSON.stringify({
          delivery_id: 'del-001',
          description: 'Electronics Package',
          amount: 100000000,
          confirmed_at: Date.now() / 1000 - 3600,
          seller: 'seller-1',
          buyer: 'buyer-1'
        }),
        minted_at: Date.now() / 1000 - 3600
      },
      {
        id: 'nft-002',
        delivery_id: 'del-004',
        owner: 'buyer-1',
        metadata: JSON.stringify({
          delivery_id: 'del-004',
          description: 'Books Collection',
          amount: 75000000,
          confirmed_at: Date.now() / 1000 - 7200,
          seller: 'seller-2',
          buyer: 'buyer-1'
        }),
        minted_at: Date.now() / 1000 - 7200
      }
    ]
    
    // Mock delivery data
    deliveries.value = [
      {
        id: 'del-001',
        description: 'Electronics Package',
        amount: 100000000
      },
      {
        id: 'del-004',
        description: 'Books Collection',
        amount: 75000000
      }
    ]
  } catch (error) {
    console.error('Error loading NFTs:', error)
  } finally {
    isLoading.value = false
  }
}

// Lifecycle
onMounted(() => {
  loadNFTs()
})
</script> 