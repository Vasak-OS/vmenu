<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue';
import CurrentWeatherCard from '@/components/card/CurrentWeatherCard.vue';
import DailyWeatherCard from '@/components/card/DailyWeatherCard.vue';

const weather = ref({
  latitude: 52.52,
  longitude: 13.419998,
  generationtime_ms: 0.06401538848876953,
  utc_offset_seconds: 0,
  timezone: 'GMT',
  timezone_abbreviation: 'GMT',
  elevation: 38.0,
  current_units: {
    time: 'iso8601',
    interval: 'seconds',
    temperature_2m: '°C',
    is_day: '',
    weather_code: 'wmo code'
  },
  current: {
    time: '2023-12-12T13:45',
    interval: 900,
    temperature_2m: 6.7,
    is_day: 1,
    weather_code: 3
  },
  daily_units: {
    time: 'iso8601',
    weather_code: 'wmo code',
    temperature_2m_max: '°C',
    temperature_2m_min: '°C'
  },
  daily: {
    time: [
      '2023-12-12',
      '2023-12-13',
      '2023-12-14',
      '2023-12-15',
      '2023-12-16',
      '2023-12-17',
      '2023-12-18'
    ],
    weather_code: [80, 63, 61, 3, 61, 61, 3],
    temperature_2m_max: [7.0, 4.3, 3.7, 5.1, 7.8, 9.5, 8.2],
    temperature_2m_min: [3.0, 1.0, 1.1, 0.1, 4.8, 8.0, 6.6]
  }
});

const getLocation = async () => {
  const location = await fetch('http://ip-api.com/json/')
    .then((res) => res.json())
    .then((data) => data);
  return location;
};

const getWeather = async () => {
  const location = await getLocation();
  const weather = await fetch(
    `https://api.open-meteo.com/v1/forecast?latitude=${location.lat}&longitude=${location.lon}&current=temperature_2m,is_day,weather_code&daily=weather_code,temperature_2m_max,temperature_2m_min`
  )
    .then((res) => res.json())
    .then((data) => data);
  return weather;
};

const dayOrNight = computed(() => {
  return weather.value.current.is_day ? 'day' : 'night';
});

onMounted(async () => {
  weather.value = await getWeather();
});
</script>

<template>
  <div class="bg-white/50 dark:bg-black/50 rounded-xl p-4">
    <template v-if="weather">
      <CurrentWeatherCard
        :current="weather.current"
        :units="weather.current_units"
        :dayOrNight="dayOrNight"
      />
      <div class="hydriam-widget-weather-forecast">
        <DailyWeatherCard
          v-for="(value, key) in weather.daily.time"
          :key="key"
          :min="weather.daily.temperature_2m_min[key]"
          :max="weather.daily.temperature_2m_max[key]"
          :units="weather.daily_units"
          :dayOrNight="dayOrNight"
          :weatherCode="weather.daily.weather_code[key]"
        />
      </div>
    </template>
    <template v-else> NO se puede cargar el clima </template>
  </div>
</template>
