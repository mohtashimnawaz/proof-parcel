<template>
  <div id="app" class="min-h-screen bg-gray-50">
    <!-- Navigation -->
    <nav class="bg-white shadow-sm border-b border-gray-200">
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
                <h1 class="text-xl font-bold text-gray-900">ProofParcel</h1>
              </div>
            </router-link>
          </div>
          
          <div class="flex items-center space-x-4">
            <router-link 
              to="/" 
              class="text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium"
              active-class="text-primary-600"
            >
              Dashboard
            </router-link>
            <router-link 
              to="/create" 
              class="text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium"
              active-class="text-primary-600"
            >
              Create Delivery
            </router-link>
            <router-link 
              to="/my-deliveries" 
              class="text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium"
              active-class="text-primary-600"
            >
              My Deliveries
            </router-link>
            <router-link 
              to="/my-nfts" 
              class="text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium"
              active-class="text-primary-600"
            >
              My NFTs
            </router-link>
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
    <footer class="bg-white border-t border-gray-200 mt-12">
      <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
        <div class="text-center text-gray-500 text-sm">
          <p>ProofParcel - Secure On-Chain Delivery Confirmation</p>
          <p class="mt-1">Built on Internet Computer Blockchain</p>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
// Component logic will be added here
</script>

<style scoped>
/* Component-specific styles */
</style> 