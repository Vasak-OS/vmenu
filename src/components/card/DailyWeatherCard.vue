<script lang="ts" setup>
import WeatherIcon from '@/components/icon/WeatherIcon.vue';

defineProps({
  min: {
    type: Number,
    required: true
  },
  max: {
    type: Number,
    required: true
  },
  units: {
    type: Object,
    required: true
  },
  dayOrNight: {
    type: String,
    required: true
  },
  weatherCode: {
    type: Number,
    required: true
  }
});
</script>
<template>
  <div class="flex flex-row items-center justify-between p-1">
    <WeatherIcon :code="weatherCode" :dayOrNight="dayOrNight" class="h-8" />
    <div class="text-sm">{{ max }}</div>
    <div class="text-sm">{{ min }}</div>
  </div>
</template>
