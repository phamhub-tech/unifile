<template>
  <div class="flex h-screen overflow-hidden">
    <NavSidenav />

    <main
      id="root"
      :class="[
        'h-screen flex-1 !overflow-y-scroll px-2 pb-20',
        'dark:bg-background border border-black/10 bg-white',
      ]"
      :style="store.mainStyle"
    >
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts">
import { useFSStore } from "~/src/fs/store";

const store = useFSStore();
</script>
