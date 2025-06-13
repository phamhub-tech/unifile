<template>
  <TooltipProvider :delay-duration="700">
    <Tooltip>
      <TooltipTrigger as-child>
        <slot />
      </TooltipTrigger>

      <TooltipContent
        :side="side ?? 'bottom'"
        class="bg-background text-foreground rounded border shadow-lg"
        arrow-class="bg-background fill-background border-r border-b"
      >
        {{ tooltip }}
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

defineProps<{ tooltip: string; side?: TooltipContentProps["side"] }>();
</script>
