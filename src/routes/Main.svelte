<script lang="ts">
    import { getContext } from "svelte";
	import { type Writable } from "svelte/store";
    
    let logHandle = getContext<Writable<string[]>>('log');


</script>
<div class="p-2 w-full overflow-hidden text-ellipsis grid grid-flow-rows bg-[#071222]">
   {#if $logHandle.length>0}
    {#each $logHandle as line,i} 
        {#if i%2==1}
            <div class="w-max overflow-hidden text-ellipsis hover:text-[#0A1626] text-[#C2D2F2] font-mono">
                {line}
            </div>
        {/if}
    {/each}
   {/if} 
</div>