<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted, computed } from 'vue';

import SearchComponent from '@/components/SearchComponent.vue';
import MenuSection from '@/components/section/MenuSection.vue';
import FilterSection from '@/components/section/FilterSection.vue';
import UserInfo from '@/components/UserInfo.vue';
import SessionButton from '@/components/button/SessionButton.vue';
import CategoryPill from '@/components/button/CategoryPill.vue';
import WeatherWidget from '@/components/widget/WeatherWidget.vue';

import shutdown from '@/assets/img/shutdown.svg';
import reboot from '@/assets/img/reboot.svg';
import logout from '@/assets/img/logout.svg';
import suspend from '@/assets/img/suspend.svg';

const menuData = ref({});
const categorySelected = ref('all');
const filter = ref('');

const setMenu = async () => {
  try {
    const items = await invoke('get_menu_items')
    console.log('Menú items recibidos:', items)
    menuData.value = items
  } catch (error) {
    console.error('Error al cargar el menú:', error)
  }
};

const logoutF = async () => {
  await $vsk.logout();
  $vsk.exit();
};

const shutdownF = async () => {
  await $vsk.shutdown();
  $vsk.exit();
};

const rebootF = async () => {
  await $vsk.reboot();
  $vsk.exit();
};

const suspendF = async () => {
  await $vsk.suspend();
  $vsk.exit();
};

const apps = computed(() => {
  const allApps = (menuData.value as any)['all']?.apps;
  console.log(allApps);
  return allApps;
});
const appsOfCategory = computed(() => (menuData.value as any)[categorySelected.value]?.apps);

onMounted(() => {
  setMenu();
});
</script>

<template>
  <div class="min-h-screen bg-white/50 dark:bg-black/50 backdrop-blur-md text-black dark:text-white rounded-xl">
    <!-- Header Section -->
    <div class="flex items-center justify-between px-6 py-3">
      <UserInfo class="w-10 h-10 rounded-full overflow-hidden" />
      
      <SearchComponent 
        v-model:filter="filter"
        class="w-1/3 mx-4" 
      />
      
      <div class="flex items-center space-x-2">
        <SessionButton 
          v-for="(action, index) in [
            { title: 'Shutdown', img: shutdown, handler: shutdownF },
            { title: 'Reboot', img: reboot, handler: rebootF },
            { title: 'Logout', img: logout, handler: logoutF },
            { title: 'Suspend', img: suspend, handler: suspendF }
          ]"
          :key="index"
          :title="action.title"
          :img="action.img"
          @click="action.handler"
          class="w-8 h-8 hover:bg-white/5 rounded-full p-1.5 transition-colors"
        />
      </div>
    </div>

    <!-- Main Content -->
    <template v-if="filter !== ''">
          <FilterSection 
            v-model:apps="apps" 
            v-model:filter="filter"
          />
        </template>
    <template v-else>
    <div class="grid grid-cols-3 gap-6 p-6">

      <!-- Apps -->
      <div class="bg-white/50 dark:bg-black/50 rounded-xl p-4">
        
        
          <MenuSection 
            v-model:apps="appsOfCategory"
          />
      </div>

      <!-- Weather Widget -->
      <div class="space-y-4">
        <WeatherWidget class="bg rounded-xl p-4" />
      </div>

      <!-- Categories -->
      <div>
        <div class="flex flex-wrap flex-row justify-center">
          <CategoryPill
          v-for="(value, key) in menuData"
          :key="key"
          :category="key"
          :image="value.icon"
          :description="value.description"
          v-model:categorySelected="categorySelected"
          :class="{'bg-white/50 dark:bg-black/50': categorySelected === key}"
        />
        </div>
        
      </div>
    </div>

  </template>
  </div>
</template>

<style>
.app-enter-active,
.app-leave-active {
  transition: all 0.2s ease;
}

.app-enter-from,
.app-leave-to {
  opacity: 0;
  transform: translateY(10px);
}

.session-button img {
  @apply brightness-0 invert opacity-75 hover:opacity-100 transition-opacity;
}

/* Estilos globales para mejorar la estética */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.2);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.3);
}
</style>