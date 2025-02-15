<script lang="ts" setup>
import { defineProps } from 'vue';

defineProps({
  title: String,
  img: String
});

</script>

<template>
  <a :title="title" href="#">
    <img :src="img" :alt="title" />
  </a>
</template>
