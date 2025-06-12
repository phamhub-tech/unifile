<template>
  <div class="flex h-screen">
    <NavSidenav />

    <main
      :class="[
        'flex-1 px-2 pt-2 pb-20',
        'border border-black/10',
        'dark:bg-background bg-white',
      ]"
    >
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts"></script>
