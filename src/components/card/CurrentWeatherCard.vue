<script lang="ts" setup>
import { defineProps, computed } from 'vue';
import WeatherIcon from '@/components/icon/WeatherIcon.vue';

const { units, current } = defineProps({
  current: {
    type: Object,
    required: true
  },
  units: {
    type: Object,
    required: true
  },
  dayOrNight: {
    type: String,
    required: true
  }
});

const getTemUnit = computed(() => units.temperature_2m);
const getTemperature = computed(() => current.temperature_2m);
</script>

<template>
  <div class="hydriam-widget-weather-today">
    <h3>{{ getTemperature }} {{ getTemUnit }}</h3>
    <WeatherIcon :code="current.weather_code" :dayOrNight="dayOrNight" />
  </div>
</template>
