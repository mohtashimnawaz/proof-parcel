<template>
  <div class="flex items-center space-x-4">
    <div v-for="(entry, idx) in statusHistory" :key="idx" class="flex items-center">
      <div class="flex flex-col items-center">
        <div :class="['w-6 h-6 rounded-full flex items-center justify-center font-bold text-xs',
          idx === statusHistory.length - 1 ? 'bg-primary-600 text-white' : 'bg-green-500 text-white']">
          {{ idx + 1 }}
        </div>
        <div class="text-xs mt-1 text-center font-semibold text-primary-600">
          {{ entry.status }}
        </div>
        <div class="text-[10px] text-gray-400 mt-0.5">{{ formatTime(entry.timestamp) }}</div>
      </div>
      <div v-if="idx < statusHistory.length - 1" class="w-8 h-1 bg-gray-300 mx-1 rounded"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  statusHistory: Array<{ status: string; timestamp: number }>
}>()

function formatTime(ts?: number) {
  if (!ts) return ''
  const d = new Date(ts * 1000)
  return d.toLocaleString()
}
</script> 