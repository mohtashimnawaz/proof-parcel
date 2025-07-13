<template>
  <div class="max-w-2xl mx-auto">
    <div class="card">
      <div class="mb-6">
        <h1 class="text-2xl font-bold text-gray-900">Confirm Delivery</h1>
        <p class="mt-2 text-gray-600">
          Enter the OTP code provided by the seller to confirm delivery and release escrow.
        </p>
      </div>

      <form @submit.prevent="confirmDelivery" class="space-y-6">
        <!-- Delivery ID -->
        <div>
          <label for="deliveryId" class="block text-sm font-medium text-gray-700 mb-2">
            Delivery ID
          </label>
          <input
            id="deliveryId"
            v-model="form.deliveryId"
            type="text"
            required
            class="input-field"
            placeholder="Enter delivery ID"
            :disabled="isLoading"
          />
        </div>

        <!-- OTP Code -->
        <div>
          <label for="otp" class="block text-sm font-medium text-gray-700 mb-2">
            OTP Code
          </label>
          <input
            id="otp"
            v-model="form.otp"
            type="text"
            required
            maxlength="6"
            class="input-field text-center text-2xl font-mono tracking-wider"
            placeholder="000000"
            :disabled="isLoading"
          />
          <p class="mt-1 text-sm text-gray-500">
            Enter the 6-digit OTP code provided by the seller
          </p>
        </div>

        <!-- Submit Button -->
        <div class="flex items-center justify-between pt-4">
          <router-link to="/" class="btn-secondary">
            Cancel
          </router-link>
          <button
            type="submit"
            :disabled="isLoading || !isFormValid"
            class="btn-success disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span v-if="isLoading" class="flex items-center">
              <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              Confirming...
            </span>
            <span v-else>Confirm Delivery</span>
          </button>
        </div>
      </form>
    </div>

    <!-- Success Modal -->
    <div v-if="showSuccessModal" class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
      <div class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
        <div class="mt-3 text-center">
          <div class="mx-auto flex items-center justify-center h-12 w-12 rounded-full bg-success-100">
            <svg class="h-6 w-6 text-success-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
            </svg>
          </div>
          <h3 class="text-lg font-medium text-gray-900 mt-4">Delivery Confirmed!</h3>
          <div class="mt-2 px-7 py-3">
            <p class="text-sm text-gray-500">
              Your delivery has been confirmed and an NFT certificate has been minted.
            </p>
            <p class="text-sm text-gray-500 mt-2">
              NFT ID: <span class="font-mono text-gray-900">{{ nftId }}</span>
            </p>
          </div>
          <div class="items-center px-4 py-3">
            <button
              @click="closeSuccessModal"
              class="btn-success w-full"
            >
              View My NFTs
            </button>
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
import { ref, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()

// Form data
const form = ref({
  deliveryId: route.params.id as string || '',
  otp: ''
})

// UI state
const isLoading = ref(false)
const error = ref('')
const showSuccessModal = ref(false)
const nftId = ref('')

// Computed
const isFormValid = computed(() => {
  return form.value.deliveryId.trim() !== '' && 
         form.value.otp.trim().length === 6
})

// Methods
const confirmDelivery = async () => {
  if (!isFormValid.value) return

  isLoading.value = true
  error.value = ''

  try {
    // TODO: Replace with actual canister call
    // const result = await canister.confirm_delivery({
    //   delivery_id: form.value.deliveryId,
    //   otp: form.value.otp
    // })

    // Mock response for now
    await new Promise(resolve => setTimeout(resolve, 2000))
    const mockNftId = 'nft-' + Math.random().toString(36).substr(2, 9)
    
    nftId.value = mockNftId
    showSuccessModal.value = true
    
    // Reset form
    form.value.otp = ''
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to confirm delivery'
  } finally {
    isLoading.value = false
  }
}

const closeSuccessModal = () => {
  showSuccessModal.value = false
  router.push('/my-nfts')
}
</script> 