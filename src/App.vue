<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

import SearchComponent from '@/components/SearchComponent.vue';
import MenuSection from '@/components/section/MenuSection.vue';
import FilterSection from '@/components/section/FilterSection.vue';
import UserInfo from '@/components/UserInfo.vue';
import SessionButton from '@/components/button/SessionButton.vue';
import CategoryPill from '@/components/button/CategoryPill.vue';
import WeatherWidget from '@/components/widget/WeatherWidget.vue';

// Importar iconos de sesión
import shutdownImg from '@/assets/img/shutdown.svg';
import rebootImg from '@/assets/img/reboot.svg';
import logoutImg from '@/assets/img/logout.svg';
import suspendImg from '@/assets/img/suspend.svg';

// Estado global
const menuData = ref<Array<any>>([]);
const categorySelected = ref('all');
const filter = ref('');


// Cargar menú
const setMenu = async () => {
  try {
    menuData.value = await invoke('get_menu_items');
  } catch (error) {
    console.error('Error al cargar el menú:', error);
  }
};

const logout = async () => {
  await invoke('logout');
};

const shutdown = async () => {
  await invoke('shutdown');
};

const reboot = async () => {
  await invoke('reboot');
};

const suspend = async () => {
  await invoke('suspend');
};

const apps = computed(() => {
  const allApps = (menuData.value as any)['all']?.apps;
  console.log(allApps);
  return allApps;
});

const appsOfCategory = computed(() => (menuData.value as any)[categorySelected.value]?.apps);

onMounted(setMenu);
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
            { title: 'Shutdown', img: shutdownImg, handler: shutdown },
            { title: 'Reboot', img: rebootImg, handler: reboot },
            { title: 'Logout', img: logoutImg, handler: logout },
            { title: 'Suspend', img: suspendImg, handler: suspend }
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
      <div class="bg-white/50 dark:bg-black/50 rounded-xl p-4 h-[calc(100vh-7rem)] overflow-y-auto">
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