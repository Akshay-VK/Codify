<script>
  import Actions from './Actions.svelte';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  let config=-1;
  onMount(()=>{
    loadConfig();
  });

  async function loadConfig(){
    config=await invoke('get_config');
    console.log(config);
  }
</script>
{#if config!=-1}
<div class="font-['Space_Grotesk'] grid grid-rows-1 grid-cols-12 gap-2 w-full h-screen bg-[#151515]">  
  <div class="col-span-1 bg-[#191919] rounded-xl h-full">side</div>
  <div class="col-span-2 bg-[#191919] rounded-xl h-full">tools</div>
  <div class="col-span-7 bg-[#191919] rounded-xl h-full">main</div>
  <div class="col-span-2 bg-[#191919] rounded-xl h-full">
    <Actions actions={config.actions}/>
  </div>
</div>
{:else}
Internal error :P
{/if}