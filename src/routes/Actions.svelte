<script lang='ts'>
  import { FAB } from 'm3-svelte';
  import iconEdit from "@ktibow/iconset-material-symbols/edit-outline";
	import Action from './Action.svelte';
  import type { TAction }  from '$lib/index.ts';

  export let actions: TAction[];
  export let baseLocation: string;
  
</script>
  
<div class="h-full w-full text-center align-text-middle py-4 align-center grid grid-rows-12 items-center divide-[#203359] divide-x divide-y">
  <div class="text-[#C2D2F2] text-2xl font-['Space Grotesk'] w-full h-full">Actions</div>
    {#each actions as action}
        <Action {action} {baseLocation}/>
    {/each}
</div>