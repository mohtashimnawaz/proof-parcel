<template>
  <div class="max-w-3xl mx-auto py-8">
    <h2 class="text-2xl font-bold mb-4">Profile</h2>
    <div class="mb-6">
      <div class="text-gray-600 dark:text-gray-300">Plug Principal:</div>
      <div class="font-mono text-sm bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded break-all">{{ principal }}</div>
    </div>

    <div class="mb-8">
      <h3 class="text-xl font-semibold mb-2">My Deliveries (as Buyer)</h3>
      <div v-if="deliveriesAsBuyer.length === 0" class="text-gray-500">No deliveries found.</div>
      <ul v-else class="space-y-2">
        <li v-for="d in deliveriesAsBuyer" :key="d.id" class="p-3 bg-white dark:bg-gray-900 rounded shadow border border-gray-200 dark:border-gray-700">
          <div><span class="font-semibold">ID:</span> {{ d.id }}</div>
          <div><span class="font-semibold">Seller:</span> {{ d.seller }}</div>
          <div><span class="font-semibold">Status:</span> {{ d.status }}</div>
          <div><span class="font-semibold">Amount:</span> {{ d.amount }}</div>
        </li>
      </ul>
    </div>

    <div class="mb-8">
      <h3 class="text-xl font-semibold mb-2">My Deliveries (as Seller)</h3>
      <div v-if="deliveriesAsSeller.length === 0" class="text-gray-500">No deliveries found.</div>
      <ul v-else class="space-y-2">
        <li v-for="d in deliveriesAsSeller" :key="d.id" class="p-3 bg-white dark:bg-gray-900 rounded shadow border border-gray-200 dark:border-gray-700">
          <div><span class="font-semibold">ID:</span> {{ d.id }}</div>
          <div><span class="font-semibold">Buyer:</span> {{ d.buyer }}</div>
          <div><span class="font-semibold">Status:</span> {{ d.status }}</div>
          <div><span class="font-semibold">Amount:</span> {{ d.amount }}</div>
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
          <div><span class="font-semibold">Metadata:</span> <span class="break-all">{{ nft.metadata }}</span></div>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
declare global {
  interface Window {
    ic?: any;
  }
}
import { ref, onMounted, watch } from 'vue'
import { Principal } from '@dfinity/principal'

const props = defineProps<{ principal: string }>()
const deliveriesAsBuyer = ref<any[]>([])
const deliveriesAsSeller = ref<any[]>([])
const nfts = ref<any[]>([])

async function fetchProfileData() {
  if (!props.principal) return
  // @ts-ignore
  const agent = window.ic?.plug?.agent
  if (!agent) return
  const backendCanisterId = import.meta.env.VITE_BACKEND_CANISTER_ID || '<CANISTER_ID>'
  const backend = await window.ic.plug.createActor({ canisterId: backendCanisterId, interfaceFactory: undefined })
  deliveriesAsBuyer.value = await backend.get_deliveries_by_buyer(Principal.fromText(props.principal))
  deliveriesAsSeller.value = await backend.get_deliveries_by_seller(Principal.fromText(props.principal))
  nfts.value = await backend.get_nfts_by_owner(Principal.fromText(props.principal))
}

onMounted(fetchProfileData)
watch(() => props.principal, fetchProfileData)
</script> 