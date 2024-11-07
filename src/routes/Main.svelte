<script lang="ts">
	import type { TOutputData } from "$lib";
    import { getContext } from "svelte";
	import { type Writable } from "svelte/store";
    
    let logHandle = getContext<Writable<TOutputData[]>>('log');


</script>
<div class="p-2 w-full overflow-hidden text-ellipsis grid grid-flow-rows bg-[#071222]">
   {#if $logHandle.length>0}
    {#each (($logHandle).filter((obj1, i, arr) =>arr.findIndex(obj2 => (obj2.line === obj1.line && obj2.commandCount ===obj1.commandCount)) === i)) as line,i}
       
            <div class="w-max overflow-hidden text-ellipsis hover:text-[#FFFFFF] text-[#C2D2F2] font-mono">
                {line.line} {line.data}
            </div>
       
    {/each}
   {/if} 
</div>