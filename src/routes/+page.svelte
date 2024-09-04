<script>
  import { invoke } from '@tauri-apps/api/tauri';
  import { Button, FAB } from "m3-svelte";
  import iconEdit from "@ktibow/iconset-material-symbols/edit-outline";

  let output='';


  async function getConfigData(){
    output=await invoke('file_test')
  }
</script>

<h1 class="text-3xl font-bold underline">
  Hello world!
  <Button type="filled" on:click="{getConfigData}">Hello world! The name of first entry is {output}</Button>
  <FAB color='primary' size='normal' icon={iconEdit} text='Hello' />
</h1>