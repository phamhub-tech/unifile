<template>
  <div class="w-full">
    <p v-if="label" class="mb-1">{{ label }}</p>
    <div
      :class="
        cn(
          'h-5 w-full overflow-hidden rounded-lg border bg-slate-100 dark:bg-slate-900',
          $attrs.class as string | undefined,
        )
      "
      role="none"
    >
      <div
        role="none"
        :class="[
          'h-full',
          'transition-transform duration-500 ease-out',
          barColor,
        ]"
        :style="{
          width: `${value}%`,
          transformOrigin: 'left',
          transform: `scaleX(${scale})`,
        }"
      />
    </div>
    <span v-if="info" class="text-muted-foreground text-sm">
      {{ info }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { cn } from "~/lib/utils";

defineOptions({ inheritAttrs: false });

interface IProps {
  /**
   * The progress value between 0 and 100
   */
  value: number;

  /**
   * The progress label shown before the progress bar
   */
  label?: string;

  /**
   * The progress sub text shown after the progress bar
   */
  info?: string;
}
const props = defineProps<IProps>();

const barColor = computed<string>(() => {
  const value = props.value;
  return value <= 80
    ? "bg-blue-500"
    : value <= 90
      ? "bg-orange-500"
      : "bg-red-500";
});

const scale = ref(0);

let timeout: number | null = null;
onMounted(() => {
  if (timeout) clearTimeout(timeout);
  timeout = window.setTimeout(() => (scale.value = 1), 100);
});
</script>
