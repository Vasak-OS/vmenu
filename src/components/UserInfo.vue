<script lang="ts" setup>
import { inject, computed, ref, onMounted } from 'vue';

const profilePicture = computed(() => `https://yt3.ggpht.com/ytc/AIdro_lLcLKB8s-7GHJBqNoj0vZNTD9AmGtFlH0pVkSTmt4uOS0=s48-c-k-c0x00ffffff-no-rj`);

</script>

<template>
  <div id="user-info" class="hydriam-user">
    <img :src="profilePicture" alt="user" />
  </div>
</template>
