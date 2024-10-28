<script lang="ts">
    import { getContext } from "svelte";
	import { type Writable } from "svelte/store";
    
    let logHandle = getContext<Writable<string[]>>('log');


</script>
<div class="p-2 w-full overflow-hidden text-ellipsis grid grid-flow-rows rounded-lg rounded-neutral-500 bg-neutral-800">
   {#if $logHandle.length>0}
    {#each $logHandle as line,i} 
        {#if i%2==1}
            <div class="w-max overflow-hidden text-ellipsis bg-neutral-800 text-neutral-400 font-mono">
                {line}
            </div>
        {/if}
    {/each}
   {/if} 
</div>