<script>
    import { invoke } from '@tauri-apps/api/tauri';
    export let path="";
    async function c(){
        let res = await invoke('dir_data',{path});
        console.log(res);
    }
</script>

Files..
<button on:click={c}>Click</button>