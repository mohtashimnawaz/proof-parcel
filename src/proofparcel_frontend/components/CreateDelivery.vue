<template>
  <div class="max-w-2xl mx-auto">
    <div class="card">
      <div class="mb-6">
        <h1 class="text-2xl font-bold text-gray-900">Create New Delivery</h1>
        <p class="mt-2 text-gray-600">
          Create a new delivery transaction. The buyer will need to confirm delivery with an OTP to release escrow.
        </p>
      </div>

      <form @submit.prevent="createDelivery" class="space-y-6">
        <!-- Buyer Principal -->
        <div>
          <label for="buyer" class="block text-sm font-medium text-gray-700 mb-2">
            Buyer Principal ID
          </label>
          <input
            id="buyer"
            v-model="form.buyer"
            type="text"
            required
            class="input-field"
            placeholder="Enter buyer's principal ID (e.g., 2vxsx-fae)"
            :disabled="isLoading"
          />
          <p class="mt-1 text-sm text-gray-500">
            The principal ID of the person receiving the delivery
          </p>
        </div>

        <!-- Amount -->
        <div>
          <label for="amount" class="block text-sm font-medium text-gray-700 mb-2">
            Amount (ICP)
          </label>
          <div class="relative">
            <input
              id="amount"
              v-model="form.amount"
              type="number"
              step="0.0001"
              min="0.0001"
              required
              class="input-field pr-12"
              placeholder="0.0000"
              :disabled="isLoading"
            />
            <div class="absolute inset-y-0 right-0 flex items-center pr-3">
              <span class="text-gray-500 text-sm">ICP</span>
            </div>
          </div>
          <p class="mt-1 text-sm text-gray-500">
            Amount to be held in escrow until delivery is confirmed
          </p>
        </div>

        <!-- Description -->
        <div>
          <label for="description" class="block text-sm font-medium text-gray-700 mb-2">
            Delivery Description
          </label>
          <textarea
            id="description"
            v-model="form.description"
            rows="4"
            required
            class="input-field"
            placeholder="Describe the items being delivered..."
            :disabled="isLoading"
          ></textarea>
          <p class="mt-1 text-sm text-gray-500">
            Brief description of what is being delivered
          </p>
        </div>

        <!-- Terms -->
        <div class="bg-gray-50 rounded-lg p-4">
          <h3 class="text-sm font-medium text-gray-900 mb-2">Delivery Process</h3>
          <ul class="text-sm text-gray-600 space-y-1">
            <li class="flex items-start">
              <span class="text-primary-600 mr-2">1.</span>
              <span>Funds will be held in escrow until delivery is confirmed</span>
            </li>
            <li class="flex items-start">
              <span class="text-primary-600 mr-2">2.</span>
              <span>Seller generates OTP when delivery is ready</span>
            </li>
            <li class="flex items-start">
              <span class="text-primary-600 mr-2">3.</span>
              <span>Buyer confirms delivery using the OTP</span>
            </li>
            <li class="flex items-start">
              <span class="text-primary-600 mr-2">4.</span>
              <span>NFT certificate is minted and escrow is released</span>
            </li>
          </ul>
        </div>

        <!-- Submit Button -->
        <div class="flex items-center justify-between pt-4">
          <router-link to="/" class="btn-secondary">
            Cancel
          </router-link>
          <button
            type="submit"
            :disabled="isLoading || !isFormValid"
            class="btn-primary disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span v-if="isLoading" class="flex items-center">
              <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              Creating...
            </span>
            <span v-else>Create Delivery</span>
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
          <h3 class="text-lg font-medium text-gray-900 mt-4">Delivery Created Successfully!</h3>
          <div class="mt-2 px-7 py-3">
            <p class="text-sm text-gray-500">
              Delivery ID: <span class="font-mono text-gray-900">{{ createdDeliveryId }}</span>
            </p>
            <p class="text-sm text-gray-500 mt-2">
              The delivery has been created and funds are held in escrow. You can now start the delivery process.
            </p>
          </div>
          <div class="items-center px-4 py-3">
            <button
              @click="closeSuccessModal"
              class="btn-primary w-full"
            >
              Continue
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
import { useRouter } from 'vue-router'

const router = useRouter()

// Form data
const form = ref({
  buyer: '',
  amount: '',
  description: ''
})

// UI state
const isLoading = ref(false)
const error = ref('')
const showSuccessModal = ref(false)
const createdDeliveryId = ref('')

// Computed
const isFormValid = computed(() => {
  return form.value.buyer.trim() !== '' && 
         parseFloat(form.value.amount) > 0 && 
         form.value.description.trim() !== ''
})

// Methods
const createDelivery = async () => {
  if (!isFormValid.value) return

  isLoading.value = true
  error.value = ''

  try {
    // TODO: Replace with actual canister call
    // const result = await canister.create_delivery({
    //   buyer: form.value.buyer,
    //   amount: BigInt(parseFloat(form.value.amount) * 100000000), // Convert to e8s
    //   description: form.value.description
    // })

    // Mock response for now
    await new Promise(resolve => setTimeout(resolve, 2000))
    const mockDeliveryId = 'del-' + Math.random().toString(36).substr(2, 9)
    
    createdDeliveryId.value = mockDeliveryId
    showSuccessModal.value = true
    
    // Reset form
    form.value = {
      buyer: '',
      amount: '',
      description: ''
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to create delivery'
  } finally {
    isLoading.value = false
  }
}

const closeSuccessModal = () => {
  showSuccessModal.value = false
  router.push('/my-deliveries')
}
</script> 