<script lang="ts">
  import Actions from './Actions.svelte';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
	import Action from './Action.svelte';
	import Main from './Main.svelte';

  import {setContext} from 'svelte';
	import { writable, type Writable } from 'svelte/store';
	import type { TOutputData, TYAMLOutputData } from '$lib';

  import { open } from '@tauri-apps/api/dialog';

  import TabContentItem from './TabContentItem.svelte';
	import TabHead from './TabHead.svelte';
	import TabHeadItem from './TabHeadItem.svelte';
	import TabWrapper from './TabWrapper.svelte';
	import Files from './Files.svelte';
	import Search from './Search.svelte';

	let activeTabValue = 1;
	const handleClick = (tabValue: number) => () => {
		activeTabValue = tabValue;
	};
  export const log = writable(new Array<TOutputData>());
  setContext<Writable<TOutputData[]>>('log',log);

  let config: any=-1;
  let yamlpath:any=-1;
  onMount(()=>{
    loadConfig();
  });

  async function loadConfig(){
    config=await invoke('get_config');
    yamlpath = await invoke('state_test');
    console.log(yamlpath);
    console.log(config);
  }

  function editYAML(){
    //
  }
  async function swapYAML(){
    const selected = await open({
      multiple:false,
      filters:[{name:'config',extensions:['yaml','yml']}]
    });
    if (selected==null){
      console.log('selected=null')
    }else{
      console.log(selected);
      let changesuccess:TYAMLOutputData=await invoke('change_yaml',{path:selected});
      console.log(changesuccess);
      if(changesuccess.success){
        config=changesuccess.config
      }
    }
  }
</script>
{#if config!=-1}
<div class="font-['Space_Grotesk'] grid grid-rows-12 grid-cols-12 w-full h-svh bg-[#0A1626] divide-[#203359] divide-x divide-y overflow-hidden">  
  <div class="col-span-3 bg-[#0A1626]  h-screen relative text-[#C2D2F2]">
     
    <TabWrapper>
    	<TabHead fullType={true}>
            <TabHeadItem fullType={true} id={1} on:click={handleClick(1)} {activeTabValue}><div class="">Files</div></TabHeadItem>
            <TabHeadItem fullType={true} id={2} on:click={handleClick(2)} {activeTabValue}><div class="">Search</div></TabHeadItem>
    	</TabHead>            
      <TabContentItem id={1} on:click={handleClick(1)} {activeTabValue}>
        <Files/>
      </TabContentItem>
      <TabContentItem id={2} on:click={handleClick(2)} {activeTabValue}>
        <Search/>
      </TabContentItem>
      
    </TabWrapper>
  </div>
  <div class="col-span-7 bg-[#0A1626]  h-screen"><Main/></div>
  <div class="col-span-2 bg-[#0A1626]  h-screen relative divide-y divide-[#203359]">
    <Actions actions={config.actions} baseLocation={config.baseLocation}/>
    <button on:click={()=>swapYAML()} class="text-center w-full py-2 hover:bg-[#C2D2F2] text-[#C2D2F2] hover:text-[#0A1626] absolute bottom-0">Change YAML </button>  </div>
</div>
{:else}
Internal error :P
{/if}