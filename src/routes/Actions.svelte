<script>
  import { FAB } from 'm3-svelte';
  import iconEdit from "@ktibow/iconset-material-symbols/edit-outline";

  let actions=[
    {'name':'abc'},
    {'name':'Apple'}
  ]; 
  
</script>
  
<div class="p-3">
  <h3 class="text-2xl font-semibold">Actions</h3>
  <div class="grid grid-flow-row gap-3">
    {#each actions as action}
        <FAB color='primary' size='normal' icon={iconEdit} text='{action.name}'/>
    {/each}
  </div>
</div>