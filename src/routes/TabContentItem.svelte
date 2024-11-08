<script lang="ts">
  export let activeTabValue;
  export let id;
  export let contentDivClass = 'p-4 bg-[#071222] text-[#C2D2F2] overflow-hidden text-ellipsis h-full';
</script>

{#if activeTabValue === id}
  <div class={contentDivClass} id="{id}-tabitem" role="tabpanel" aria-labelledby="{id}-tab">
    <slot />
  </div>
{/if}