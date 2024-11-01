<script lang="ts">
  import Actions from './Actions.svelte';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
	import Action from './Action.svelte';
	import Main from './Main.svelte';

  import {setContext} from 'svelte';
	import { writable, type Writable } from 'svelte/store';

  export const log = writable(new Array<string>());
  setContext<Writable<string[]>>('log',log);

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

  let showedityaml=false;
  function editYAML(){
    //
  }
</script>
{#if config!=-1}
<div class="font-['Space_Grotesk'] grid grid-rows-1 grid-cols-12 gap-2 w-full h-screen bg-[#151515]">  
  <div class="col-span-1 bg-[#191919] rounded-xl h-full">
    <button on:click={()=>editYAML()}>Edit</button>  
  </div>
  <div class="col-span-2 bg-[#191919] rounded-xl h-full">tools</div>
  <div class="col-span-7 bg-[#191919] rounded-xl h-full"><Main/></div>
  <div class="col-span-2 bg-[#191919] rounded-xl h-full">
    <Actions actions={config.actions} baseLocation={config.baseLocation}/>
  </div>
</div>
{:else}
Internal error :P
{/if}