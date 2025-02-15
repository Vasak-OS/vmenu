<script lang="ts" setup>
import { defineProps } from 'vue';
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
  <div class="col text-center">
    <WeatherIcon :code="weatherCode" :dayOrNight="dayOrNight" />
    <div>{{ max }}</div>
    <div>{{ min }}</div>
  </div>
</template>
