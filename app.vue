<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<script lang="ts" setup>
import { useSettingsStore } from '~/src/settings/store';

const settingsStore = useSettingsStore();
await settingsStore.init();
</script>
