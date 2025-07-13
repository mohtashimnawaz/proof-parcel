<template>
  <div class="flex items-center space-x-4">
    <div v-for="(step, idx) in steps" :key="step.key" class="flex items-center">
      <div class="flex flex-col items-center">
        <div :class="['w-6 h-6 rounded-full flex items-center justify-center font-bold text-xs',
          idx < currentStep ? 'bg-green-500 text-white' : idx === currentStep ? 'bg-primary-600 text-white' : 'bg-gray-300 text-gray-500']">
          {{ idx + 1 }}
        </div>
        <div class="text-xs mt-1 text-center" :class="idx === currentStep ? 'font-semibold text-primary-600' : 'text-gray-500'">
          {{ step.label }}
        </div>
        <div v-if="step.timestamp" class="text-[10px] text-gray-400 mt-0.5">{{ formatTime(step.timestamp) }}</div>
      </div>
      <div v-if="idx < steps.length - 1" class="w-8 h-1 bg-gray-300 mx-1 rounded"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  status: string,
  createdAt?: number,
  confirmedAt?: number,
  escrowReleasedAt?: number,
  deliveredAt?: number,
  inTransitAt?: number,
  cancelledAt?: number,
}>()

const steps = computed(() => [
  { key: 'Pending', label: 'Pending', timestamp: props.createdAt },
  { key: 'InTransit', label: 'In Transit', timestamp: props.inTransitAt },
  { key: 'Delivered', label: 'Delivered', timestamp: props.deliveredAt },
  { key: 'Confirmed', label: 'Confirmed', timestamp: props.confirmedAt },
  { key: 'EscrowReleased', label: 'Escrow Released', timestamp: props.escrowReleasedAt },
  { key: 'Cancelled', label: 'Cancelled', timestamp: props.cancelledAt },
])

const statusOrder = ['Pending', 'InTransit', 'Delivered', 'Confirmed', 'EscrowReleased', 'Cancelled']
const currentStep = computed(() => statusOrder.indexOf(props.status))

function formatTime(ts?: number) {
  if (!ts) return ''
  const d = new Date(ts * 1000)
  return d.toLocaleString()
}
</script> 