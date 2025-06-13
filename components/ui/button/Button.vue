<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { Primitive, type PrimitiveProps } from "reka-ui";

import { cn } from "@/lib/utils";

import { type ButtonVariants, buttonVariants } from ".";

interface Props extends PrimitiveProps {
  variant?: ButtonVariants["variant"];
  size?: ButtonVariants["size"];
  class?: HTMLAttributes["class"];
  loading?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  as: "button",
});
</script>

<template>
  <Primitive
    data-slot="button"
    :as="as"
    :as-child="asChild"
    :class="
      cn(
        buttonVariants({ variant, size }),
        props.class,
        loading &&
          'pointer-events-none text-transparent grayscale transition-colors',
      )
    "
  >
    <slot />
    <div
      :class="[
        'absolute top-1/2 left-1/2 aspect-square h-3/5 -translate-x-1/2 -translate-y-1/2 transition-opacity',
        loading ? 'opacity-100' : 'opacity-0',
      ]"
      aria-role="none"
    >
      <div
        class="absolute inset-0 rounded-full border-2 border-black/20"
        arial-role="none"
      />
      <div
        class="absolute inset-0 animate-spin rounded-full border-2 border-white border-r-transparent border-b-transparent"
        aria-role="none"
      />
    </div>
  </Primitive>
</template>
