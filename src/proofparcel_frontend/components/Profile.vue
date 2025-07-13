<template>
  <div class="max-w-3xl mx-auto py-8">
    <!-- Toast notification -->
    <div v-if="toast.show" :class="['fixed top-6 left-1/2 transform -translate-x-1/2 z-50 px-4 py-2 rounded shadow text-white', toast.type === 'success' ? 'bg-green-600' : 'bg-red-600']">
      {{ toast.message }}
    </div>

    <div class="flex items-center mb-6">
      <img :src="avatarUrl" alt="avatar" class="w-16 h-16 rounded-full border-2 border-primary-600 mr-4" />
      <div>
        <h2 class="text-2xl font-bold">Profile</h2>
        <div class="text-gray-600 dark:text-gray-300 text-xs">Plug Principal:</div>
        <div class="font-mono text-sm bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded break-all">{{ principal }}</div>
      </div>
    </div>

    <div v-if="loading" class="flex justify-center items-center py-12">
      <svg class="animate-spin h-8 w-8 text-primary-600" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"/></svg>
    </div>

    <div v-else>
      <div class="mb-8">
        <h3 class="text-xl font-semibold mb-2">My Deliveries (as Buyer)</h3>
        <div v-if="deliveriesAsBuyer.length === 0" class="text-gray-500">No deliveries found.</div>
        <ul v-else class="space-y-2">
          <li v-for="d in deliveriesAsBuyer" :key="d.id" class="p-3 bg-white dark:bg-gray-900 rounded shadow border border-gray-200 dark:border-gray-700">
            <div class="flex justify-between items-center">
              <div>
                <div><span class="font-semibold">ID:</span> {{ d.id }}</div>
                <div><span class="font-semibold">Seller:</span> {{ d.seller }}</div>
                <div><span class="font-semibold">Status:</span> {{ d.status }}</div>
                <div><span class="font-semibold">Amount:</span> {{ d.amount }}</div>
              </div>
              <div class="flex flex-col space-y-2 ml-4">
                <button class="px-2 py-1 rounded bg-primary-600 text-white text-xs hover:bg-primary-700 disabled:opacity-50" :disabled="actionLoading" @click="viewDeliveryDetails(d)">View Details</button>
                <button v-if="d.status === 'Delivered'" class="px-2 py-1 rounded bg-green-600 text-white text-xs hover:bg-green-700 disabled:opacity-50" :disabled="actionLoading" @click="confirmDelivery(d)">
                  <span v-if="actionLoading && actionTargetId === d.id" class="inline-block animate-spin mr-1"><svg class="h-4 w-4" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"/></svg></span>
                  Confirm Delivery
                </button>
              </div>
            </div>
            <DeliveryTimeline :status="d.status" :createdAt="d.created_at" :confirmedAt="d.confirmed_at" :escrowReleasedAt="d.escrow_released_at" />
          </li>
        </ul>
      </div>

      <div class="mb-8">
        <h3 class="text-xl font-semibold mb-2">My Deliveries (as Seller)</h3>
        <div v-if="deliveriesAsSeller.length === 0" class="text-gray-500">No deliveries found.</div>
        <ul v-else class="space-y-2">
          <li v-for="d in deliveriesAsSeller" :key="d.id" class="p-3 bg-white dark:bg-gray-900 rounded shadow border border-gray-200 dark:border-gray-700">
            <div class="flex justify-between items-center">
              <div>
                <div><span class="font-semibold">ID:</span> {{ d.id }}</div>
                <div><span class="font-semibold">Buyer:</span> {{ d.buyer }}</div>
                <div><span class="font-semibold">Status:</span> {{ d.status }}</div>
                <div><span class="font-semibold">Amount:</span> {{ d.amount }}</div>
              </div>
              <div class="flex flex-col space-y-2 ml-4">
                <button class="px-2 py-1 rounded bg-primary-600 text-white text-xs hover:bg-primary-700 disabled:opacity-50" :disabled="actionLoading" @click="viewDeliveryDetails(d)">View Details</button>
                <button v-if="d.status === 'Pending'" class="px-2 py-1 rounded bg-yellow-500 text-white text-xs hover:bg-yellow-600 disabled:opacity-50" :disabled="actionLoading" @click="startDelivery(d)">
                  <span v-if="actionLoading && actionTargetId === d.id" class="inline-block animate-spin mr-1"><svg class="h-4 w-4" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"/></svg></span>
                  Start Delivery
                </button>
                <button v-if="d.status === 'InTransit'" class="px-2 py-1 rounded bg-blue-600 text-white text-xs hover:bg-blue-700 disabled:opacity-50" :disabled="actionLoading" @click="generateOtp(d)">
                  <span v-if="actionLoading && actionTargetId === d.id" class="inline-block animate-spin mr-1"><svg class="h-4 w-4" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"/></svg></span>
                  Generate OTP
                </button>
              </div>
            </div>
            <DeliveryTimeline :status="d.status" :createdAt="d.created_at" :confirmedAt="d.confirmed_at" :escrowReleasedAt="d.escrow_released_at" />
          </li>
        </ul>
      </div>

      <div>
        <h3 class="text-xl font-semibold mb-2">My NFTs</h3>
        <div v-if="nfts.length === 0" class="text-gray-500">No NFTs found.</div>
        <ul v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <li v-for="nft in nfts" :key="nft.id" class="p-3 bg-white dark:bg-gray-900 rounded shadow border border-gray-200 dark:border-gray-700">
            <div><span class="font-semibold">NFT ID:</span> {{ nft.id }}</div>
            <div><span class="font-semibold">Delivery ID:</span> {{ nft.delivery_id }}</div>
            <button class="mt-2 px-2 py-1 rounded bg-primary-600 text-white text-xs hover:bg-primary-700" @click="viewNftMetadata(nft)">View Metadata</button>
          </li>
        </ul>
      </div>

      <!-- NFT Metadata Modal -->
      <div v-if="showNftModal" class="fixed inset-0 bg-black bg-opacity-40 flex items-center justify-center z-50">
        <div class="bg-white dark:bg-gray-900 p-6 rounded shadow-lg max-w-lg w-full">
          <h4 class="text-lg font-bold mb-2">NFT Metadata</h4>
          <pre class="bg-gray-100 dark:bg-gray-800 p-2 rounded text-xs overflow-x-auto">{{ selectedNftMetadata }}</pre>
          <button class="mt-4 px-3 py-1 rounded bg-primary-600 text-white hover:bg-primary-700" @click="showNftModal = false">Close</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
declare global {
  interface Window {
    ic?: any;
  }
}
import { ref, onMounted, watch, computed } from 'vue'
import { Principal } from '@dfinity/principal'
import DeliveryTimeline from './DeliveryTimeline.vue'

const props = defineProps<{ principal: string }>()
const deliveriesAsBuyer = ref<any[]>([])
const deliveriesAsSeller = ref<any[]>([])
const nfts = ref<any[]>([])
const showNftModal = ref(false)
const selectedNftMetadata = ref('')
const loading = ref(true)
const actionLoading = ref(false)
const actionTargetId = ref('')
const toast = ref({ show: false, message: '', type: 'success' })

const avatarUrl = computed(() =>
  `https://api.dicebear.com/7.x/identicon/svg?seed=${props.principal}`
)

function showToast(message: string, type: 'success' | 'error' = 'success') {
  toast.value = { show: true, message, type }
  setTimeout(() => { toast.value.show = false }, 3000)
}

async function fetchProfileData() {
  loading.value = true
  try {
    if (!props.principal) return
    // @ts-ignore
    const agent = window.ic?.plug?.agent
    if (!agent) return
    const backendCanisterId = import.meta.env.VITE_BACKEND_CANISTER_ID || '<CANISTER_ID>'
    const backend = await window.ic.plug.createActor({ canisterId: backendCanisterId, interfaceFactory: undefined })
    deliveriesAsBuyer.value = await backend.get_deliveries_by_buyer(Principal.fromText(props.principal))
    deliveriesAsSeller.value = await backend.get_deliveries_by_seller(Principal.fromText(props.principal))
    nfts.value = await backend.get_nfts_by_owner(Principal.fromText(props.principal))
  } catch (e) {
    showToast('Failed to load profile data', 'error')
  } finally {
    loading.value = false
  }
}

function viewDeliveryDetails(delivery: any) {
  showToast('Delivery details coming soon!', 'success')
}
async function confirmDelivery(delivery: any) {
  const otp = prompt('Enter OTP to confirm delivery:')
  if (!otp) return
  actionLoading.value = true
  actionTargetId.value = delivery.id
  try {
    const backendCanisterId = import.meta.env.VITE_BACKEND_CANISTER_ID || '<CANISTER_ID>'
    const backend = await window.ic.plug.createActor({ canisterId: backendCanisterId, interfaceFactory: undefined })
    const result = await backend.confirm_delivery({ delivery_id: delivery.id, otp })
    if ('Ok' in result) {
      showToast('Delivery confirmed! NFT minted: ' + result.Ok, 'success')
      await fetchProfileData()
    } else {
      showToast('Error: ' + result.Err, 'error')
    }
  } catch (e) {
    showToast('Failed to confirm delivery', 'error')
  } finally {
    actionLoading.value = false
    actionTargetId.value = ''
  }
}
async function startDelivery(delivery: any) {
  actionLoading.value = true
  actionTargetId.value = delivery.id
  try {
    const backendCanisterId = import.meta.env.VITE_BACKEND_CANISTER_ID || '<CANISTER_ID>'
    const backend = await window.ic.plug.createActor({ canisterId: backendCanisterId, interfaceFactory: undefined })
    const result = await backend.start_delivery(delivery.id)
    if ('Ok' in result) {
      showToast('Delivery started!', 'success')
      await fetchProfileData()
    } else {
      showToast('Error: ' + result.Err, 'error')
    }
  } catch (e) {
    showToast('Failed to start delivery', 'error')
  } finally {
    actionLoading.value = false
    actionTargetId.value = ''
  }
}
async function generateOtp(delivery: any) {
  actionLoading.value = true
  actionTargetId.value = delivery.id
  try {
    const backendCanisterId = import.meta.env.VITE_BACKEND_CANISTER_ID || '<CANISTER_ID>'
    const backend = await window.ic.plug.createActor({ canisterId: backendCanisterId, interfaceFactory: undefined })
    const result = await backend.generate_delivery_otp(delivery.id)
    if ('Ok' in result) {
      showToast('OTP generated: ' + result.Ok, 'success')
      await fetchProfileData()
    } else {
      showToast('Error: ' + result.Err, 'error')
    }
  } catch (e) {
    showToast('Failed to generate OTP', 'error')
  } finally {
    actionLoading.value = false
    actionTargetId.value = ''
  }
}
function viewNftMetadata(nft: any) {
  selectedNftMetadata.value = nft.metadata
  showNftModal.value = true
}

onMounted(fetchProfileData)
watch(() => props.principal, fetchProfileData)
</script> 