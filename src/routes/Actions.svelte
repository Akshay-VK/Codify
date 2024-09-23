<script lang='ts'>
  import { FAB } from 'm3-svelte';
  import iconEdit from "@ktibow/iconset-material-symbols/edit-outline";
	import Action from './Action.svelte';
  import type { TAction }  from '$lib/index.ts';

  export let actions: TAction[];
  
</script>
  
<div class="p-3  align-middle align-center items-center">
  <div class="text-[#e4e0d3] text-[32px] font-bold font-['Space Grotesk']">Actions</div>
  <div class="grid grid-flow-row gap-3">
    {#each actions as action}
        <Action {action}/>
    {/each}
  </div>
</div>