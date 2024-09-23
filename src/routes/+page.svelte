<script>
  import { invoke } from '@tauri-apps/api/tauri';
  import { Button, FAB } from "m3-svelte";
  import iconEdit from "@ktibow/iconset-material-symbols/edit-outline";

  import App from './App.svelte'

</script>

<!--
<h1 class="text-3xl font-bold underline">
  Hello world!
  <Button type="filled" on:click="{getConfigData}">Hello world!</Button>
  <br>
  <FAB color='primary' size='normal' icon={iconEdit} text='Hello' />
  <p>
    {#each output as cmd}
      <i>{cmd}</i>
    {/each}
  </p>
</h1>-->
<App/>