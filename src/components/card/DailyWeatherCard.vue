<script lang="ts" setup>
import WeatherIcon from '@/components/icon/WeatherIcon.vue';
import { computed } from 'vue';

const props = defineProps({
  date: {
    type: String,
    required: true
  },
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

const formattedDate = computed(() => {
  const dateObj = new Date(props.date);
  dateObj.setDate(dateObj.getDate() + 1);
  return dateObj.toLocaleDateString(undefined, { month: 'numeric', day: 'numeric' });
});
</script>
<template>
  <div class="daily-weather-card-layout">
    <div class="date-display">{{ formattedDate }}</div>
    <WeatherIcon :code="weatherCode" :dayOrNight="dayOrNight" class="weather-icon" />
    <div class="temperatures">
      <span class="temp-max">{{ max }}°</span>
      <span class="temp-min">{{ min }}°</span>
    </div>
  </div>
</template>

<style scoped>
@reference "../../style.css";

.daily-weather-card-layout {
  @apply flex flex-col items-center gap-2 p-2 rounded-vsk bg-vsk-primary/10;
}

.date-display {
  font-size: 0.875rem;
  font-weight: 500;
}

.weather-icon {
  margin-top: 0.25rem;
  margin-bottom: 0.25rem;
}

.temperatures {
  display: flex;
  gap: 0.5rem;
  font-size: 0.875rem;
}

.temp-max {
  font-weight: 600;
}

.temp-min {
  font-weight: 400;
}
</style>
