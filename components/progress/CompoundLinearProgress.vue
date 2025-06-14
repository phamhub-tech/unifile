<template>
  <div class="@container">
    <p v-if="label" class="mb-1">{{ label }}</p>
    <div
      class="flex h-5 w-full overflow-hidden rounded-lg border bg-slate-100 dark:bg-slate-900"
      role="none"
    >
      <div
        ref="bar"
        class="h-full w-full"
        :style="{
          backgroundImage: backgroundColor,
          transform: `scaleX(${scale})`,
          transformOrigin: 'left',
        }"
      />
    </div>
    <div
      :class="[
        'text-muted-foreground mt-1 grid gap-x-8 gap-y-3 text-sm',
        '@sm:grid-cols-2 @md:grid-cols-3',
      ]"
    >
      <div
        v-for="(_, i) of values"
        :key="`progress-info-${i}`"
        class="flex gap-x-2"
      >
        <div
          class="mt-1.5 h-2 w-4 shrink-0 rounded-full"
          role="none"
          :style="{ backgroundColor: colors ? colors[i] : undefined }"
        />
        <slot name="legend" :index="i" :percentage="toReadable(values[i])">
          <p>{{ labels ? (labels[i] ?? "undefined") : "undefined" }}</p>
          <p class="ml-auto">{{ toReadable(values[i]) }}%</p>
        </slot>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";

import { Curves } from "~/core/curves";
import { tween } from "~/core/utils";

interface IProps {
  /**
   * Each value representing a fraction of the total
   *
   * The sum of values should be 1
   */
  values: number[];

  /**
   * The colors per value
   *
   * The length of colors should be equal to the length of values
   */
  colors?: string[];

  /**
   * The labels per value
   *
   * The length of labels should be equal to the length of values
   */
  labels?: string[];

  /**
   * The progress label shown before the progress bar
   */
  label?: string;

  /**
   * How long it takes for the progress animation in milliseconds
   */
  duration?: number;
}
const props = withDefaults(defineProps<IProps>(), { duration: 2000 });

const backgroundColor = computed<string>(() => {
  const colors: string[] = [];
  let cummulativePercent = 0;
  for (let i = 0; i < props.values.length; i++) {
    cummulativePercent += props.values[i];
    const color = props.colors ? (props.colors[i] ?? "hotpink") : "hotpink";
    colors.push(`${color} 0 ${cummulativePercent}%`);
  }

  const colorsString = colors.join(", ");
  return `linear-gradient(to right, ${colorsString})`;
});

const scale = ref(0.78);

function toReadable(percent: number): string {
  let value = percent.toString();

  const valueSplit = value.split(".");
  if (valueSplit.length === 2) value = percent.toFixed(2);

  return value;
}

let cancelAnimation: (() => void) | null = null;
function scaleHandler() {
  if (cancelAnimation) cancelAnimation();

  const el = bar.value;
  if (!el) return;

  const parent = el.parentElement!;
  const parentWidth = parent.getBoundingClientRect().width;

  cancelAnimation = tween({
    duration: props.duration,
    onUpdate: (value, { to }) => {
      value = Curves.easeOutCubic(value, to);

      const width = parentWidth * value;
      const count = parentWidth / width;
      const percentage = count * 100;

      el.style.backgroundSize = `${percentage}%`;
      scale.value = value;
    },
  });
}

const bar = ref<HTMLDivElement | null>(null);
onMounted(scaleHandler);
</script>
