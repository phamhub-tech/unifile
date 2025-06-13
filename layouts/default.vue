<template>
  <div class="flex h-screen overflow-hidden">
    <NavSidenav />

    <main
      :class="[
        'flex-1 px-2 pb-20 overflow-y-scroll overscroll-contain',
        'border border-black/10',
        'dark:bg-background bg-white',
      ]"
    >
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts"></script>
