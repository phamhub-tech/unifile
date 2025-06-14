<template>
  <TooltipProvider v-bind="forwarded" :delay-duration="700">
    <Tooltip>
      <TooltipTrigger as-child>
        <slot />
      </TooltipTrigger>

      <TooltipContent
        :side="side ?? 'bottom'"
        :class="
          cn(
            'bg-background text-foreground max-w-xl rounded border text-sm shadow-lg dark:bg-gray-800',
            $attrs.class as string | undefined,
          )
        "
        arrow-class="bg-background fill-background border-r border-b dark:bg-gray-800 dark:fill-gray-800"
      >
        <slot name="tooltip">
          {{ tooltip }}
        </slot>
      </TooltipContent>
    </Tooltip>
  </TooltipProvider>
</template>

<script lang="ts" setup>
import type { TooltipContentProps } from "reka-ui";
import {
  Tooltip,
  TooltipTrigger,
  TooltipProvider,
  TooltipContent,
} from "./ui/tooltip";
import { cn } from "~/lib/utils";
import { removeFromAttrs } from "~/core/utils";

defineOptions({ inheritAttrs: false})
defineProps<{ tooltip?: string; side?: TooltipContentProps["side"] }>();

const attrs = useAttrs();
const forwarded = removeFromAttrs(attrs, 'class');
</script>
