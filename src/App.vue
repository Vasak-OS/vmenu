<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useConfigStore } from "@vasakgroup/plugin-config-manager";

import SearchComponent from "@/components/SearchComponent.vue";
import MenuSection from "@/components/section/MenuSection.vue";
import FilterSection from "@/components/section/FilterSection.vue";
import UserInfo from "@/components/UserInfo.vue";
import SessionButton from "@/components/button/SessionButton.vue";
import CategoryPill from "@/components/button/CategoryPill.vue";
import WeatherWidget from "@/components/widget/WeatherWidget.vue";

// Importar iconos de sesión
import shutdownImg from "@/assets/img/shutdown.svg";
import rebootImg from "@/assets/img/reboot.svg";
import logoutImg from "@/assets/img/logout.svg";
import suspendImg from "@/assets/img/suspend.svg";

// Estado global
const menuData = ref<Array<any>>([]);
const categorySelected = ref<any>("all");
const filter = ref<string>("");
const configStore = useConfigStore();
let unlistenConfig: Function | null = null;

// Cargar menú
const setMenu = async () => {
  try {
    menuData.value = await invoke("get_menu_items");
  } catch (error) {
    console.error("Error al cargar el menú:", error);
  }
};

const logout = async () => {
  await invoke("logout");
};

const shutdown = async () => {
  await invoke("shutdown");
};

const reboot = async () => {
  await invoke("reboot");
};

const suspend = async () => {
  await invoke("suspend");
};

const apps = computed(() => {
  const allApps = (menuData.value as any)["all"]?.apps;
  return allApps;
});

const appsOfCategory = computed(
  () => (menuData.value as any)[categorySelected.value]?.apps
);

onMounted(async () => {
  setMenu();
  configStore.loadConfig();
  unlistenConfig = await listen("config-changed", async () => {
    configStore.loadConfig();
  });
});

onUnmounted(() => {
  if (unlistenConfig !== null) {
    unlistenConfig();
  }
});
</script>

<template>
  <div
    class="vmenu background"
  >
    <!-- Header Section -->
    <div class="flex items-center justify-between animate-fadeIn mb-4 header-section">
      <UserInfo />

      <SearchComponent
        v-model:filter="filter"
        class="search-component"
      />

      <div class="flex items-center space-x-2">
        <SessionButton
          v-for="(action, index) in [
            { title: 'Shutdown', img: shutdownImg, handler: shutdown },
            { title: 'Reboot', img: rebootImg, handler: reboot },
            { title: 'Logout', img: logoutImg, handler: logout },
            { title: 'Suspend', img: suspendImg, handler: suspend },
          ]"
          :key="index"
          :title="action.title"
          :img="action.img"
          @click="action.handler"
          class="session-button"
        />
      </div>
    </div>

    <!-- Main Content -->
    <transition name="fade" mode="out-in">
      <div v-if="filter !== ''" key="filter-view" class="animate-fadeIn">
        <FilterSection v-model:apps="apps" v-model:filter="filter" />
      </div>
      <div v-else key="main-view" class="grid grid-cols-3 gap-4 animate-slideUpPlus h-[calc(100vh-88px)]">
        <!-- Apps -->
        <div
          class="background rounded-vsk p-4 h-full overflow-y-auto apps-container"
        >
          <MenuSection v-model:apps="appsOfCategory" />
        </div>

        <!-- Weather Widget -->
        <div class="rounded-vsk background p-4 space-y-4 h-full overflow-y-auto weather-container">
          <WeatherWidget />
        </div>

        <!-- Categories -->
        <div class="categories-container">
          <transition-group
            tag="div"
            name="list-stagger"
            appear
            class="flex flex-wrap flex-row justify-center category-pills-wrapper"
          >
            <CategoryPill
              v-for="(value, key, index) in menuData"
              :key="key"
              :data-index="index"
              :category="key"
              :image="value.icon"
              :description="value.description"
              v-model:categorySelected="categorySelected"
              class="category-pill"
            />
          </transition-group>
        </div>
      </div>
    </transition>
  </div>
</template>

<style>
@reference "./style.css";

.vmenu {
  @apply h-screen p-4;
  border-radius: calc(var(--border-radius) + 16px);
}

.header-section {
  /* Puedes ajustar la duración si es necesario */
  animation-duration: 0.5s;
}

.search-component {
  @apply w-2/5 border-b-2 border-vsk-primary;
}

.session-button {
  @apply w-10 h-10 hover:bg-vsk-primary/30 rounded-vsk p-1 transform transition-all duration-200 ease-out hover:scale-110 hover:rotate-3;
}
/* Animaciones personalizadas */
@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes slideUpPlus {
  from {
    opacity: 0;
    transform: translateY(30px); /* Un poco más de desplazamiento */
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-fadeIn {
  animation: fadeIn 0.4s ease-out; /* Duración ligeramente más corta */
}

.animate-slideUpPlus {
  animation: slideUpPlus 0.5s ease-out forwards;
}

.session-button img {
  @apply brightness-0 invert opacity-75 group-hover:opacity-100 transition-opacity duration-200;
}

/* Transiciones de Vue */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* Animación de lista escalonada para píldoras de categoría */
.list-stagger-enter-active,
.list-stagger-leave-active {
  transition: all 0.4s ease;
}
.list-stagger-enter-from,
.list-stagger-leave-to {
  opacity: 0;
  transform: translateY(20px) scale(0.9);
}
.list-stagger-move {
  transition: transform 0.4s ease;
}

.category-pills-wrapper {
  gap: 0.75rem; /* Espacio entre píldoras */
}

.category-pill {
  transition: transform 0.2s ease-out, box-shadow 0.2s ease-out;
}

.category-pill:hover {
  transform: translateY(-4px) scale(1.03); /* Efecto de elevación y ligero zoom */
  box-shadow: 0 4px 15px rgba(0,0,0,0.1);
}

/* Estilos para contenedores de la cuadrícula principal para posibles animaciones internas */
.apps-container, .weather-container, .categories-container {
  animation: fadeIn 0.5s ease-out 0.2s backwards; /* Retraso para que aparezcan después del slideUpPlus */
}


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
