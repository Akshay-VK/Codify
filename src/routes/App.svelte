<script>
  import Actions from './Actions.svelte';
</script>

<div class="font-['Space_Grotesk'] grid grid-rows-1 grid-cols-12 gap-2 w-screen h-screen bg-[#151515] p-2">  
  <div class="col-span-1 bg-[#191919] rounded-lg">side</div>
  <div class="col-span-2 bg-[#191919] rounded-lg">tools</div>
  <div class="col-span-7 bg-[#191919] rounded-lg">main</div>
  <div class="col-span-2 bg-[#191919] rounded-lg">
    <Actions/>
  </div>
</div>