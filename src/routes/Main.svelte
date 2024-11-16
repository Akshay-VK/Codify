<script lang="ts">
	import type { TOutputData } from '$lib';
	import { getContext } from 'svelte';
	import { type Writable } from 'svelte/store';
	import TabContentItem from './TabContentItem.svelte';
	import TabHead from './TabHead.svelte';
	import TabHeadItem from './TabHeadItem.svelte';
	import TabWrapper from './TabWrapper.svelte';

	let logHandle = getContext<Writable<TOutputData[]>>('log');

	let activeTabValue = 1;
	const handleClick = (tabValue: number) => () => {
		activeTabValue = tabValue;
	};

    function filterDups(log:TOutputData[]): TOutputData[]{
        return  log.filter((obj1, i, arr) => arr.findIndex((obj2) => obj2.line === obj1.line && obj2.commandCount === obj1.commandCount) === i)
    }

    function sortOutput(log: TOutputData[]){
        let out:TOutputData[][]=[];
        for(let i =0;i<log.length;i++){
            if(out.length-1<log[i].commandCount-1){
                out.push([]);
            }
            out[log[i].commandCount-1].push(log[i]);
        }
        console.log("sorted output",out);
        return out
    }
</script>

{#if $logHandle.length > 0}
<TabWrapper>
	<TabHead fullType={false}>
        {#each {length:(($logHandle)[$logHandle.length-1].commandCount)} as _,j}
            <TabHeadItem id={j+1} on:click={handleClick(j+1)} {activeTabValue}>Output {j+1}</TabHeadItem>
        {/each}
	</TabHead>
        {#each {length:(($logHandle)[$logHandle.length-1].commandCount)} as _,j}
        <TabContentItem id={j+1} on:click={handleClick(j+1)} {activeTabValue}>
            {#each filterDups($logHandle).filter(e=>e.commandCount==(j+1)) as line,i}
                    <div
				class="w-max overflow-hidden text-ellipsis hover:text-[#FFFFFF] text-[#C2D2F2] font-mono"
			>
				{line.line}
				{line.data}
			</div>
                {/each}
            </TabContentItem>
        {/each}
</TabWrapper>
{/if}