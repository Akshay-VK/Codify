<script lang="ts">
    import {onMount} from "svelte";
    import { invoke } from '@tauri-apps/api/tauri';
	import type { TFileData, TFolderData, TSubfolder } from "$lib";

    export let path="";

    let dir: TFolderData;
    async function loadDir(){
        dir = await invoke('dir_data',{path});
        console.log(dir);
    }

    type DirElem={
        path:string,
        name:string,
    }
    function pathToObj(p: TSubfolder[]|TFileData[]){
        let res:DirElem[]=[]
        for(let i=0;i<p.length;i++){
            let path=p[i].path.replaceAll('\\','/')
            let name=path.split('/').reverse()[0]
            res.push({path,name});
        }
        return res;
    }

    onMount(()=>{
        loadDir();
    });
</script>
{#if dir}
{dir.path}
<ul>
    {#each pathToObj(dir.folders) as f}
        <li>
            <div class="p-2">>>{f.name}</div> 
        </li>
    {/each}

    {#each pathToObj(dir.files) as f}
        <li>
            <div class="p-2">>{f.name}</div> 
        </li>
    {/each}
</ul>
{/if}