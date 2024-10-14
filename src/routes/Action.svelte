<script lang="ts">
	import { FAB } from 'm3-svelte';
	import iconEdit from '@ktibow/iconset-material-symbols/edit-outline';
    import { invoke } from '@tauri-apps/api/tauri';
    import { listen } from '@tauri-apps/api/event'
	import type { TAction } from '$lib';

	export let action: TAction;
    export let baseLocation: string;

    let switched=false;
    let sc='scale-0'
    let  tc='text-white';


    let formPresent=action.arguments?.length>0;
    let formVisible= false;

    async function clicked(){
        console.log(action.name);
        switched=!switched;
        if(switched){
            sc='scale-[8]';
            tc='text-black';
            //CALL ACTION
            if(formPresent){
                formVisible=true;
            }else{
                let output=await invoke('run_command_stream',{baseLocation,action})
            }
            //console.log(output);

            sc='scale-0';
            tc='text-white'
            switched=false;
        }else{
            sc='scale-0';
            tc='text-white'
        }
    }

    let logged="";
    let n=0;
    listen("output_data",(event: any)=>{
        n++;
        if(event.payload.data!=logged){
            console.log(event.payload.data);
            console.log(n);
            logged=event.payload.data as string;
        }
    })
</script>
<div class="w-full font-['Space Grotesk']  bg-clip-content">
    <div class="bg-neutral-700 hover:bg-neutral-800 p-4 grid grid-cols-4 rounded-3xl items-center transition-colors  overflow-hidden bg-clip-padding">
        <div class="z-50 col-span-3 text-center align-text-middle h-full {tc} transition-colors">{action.name}</div>
        <button class="col-span-1 relative" on:click={clicked}>
            <div class="z-50">
                <svg xmlns="http://www.w3.org/2000/svg" width="2em" height="2em" viewBox="0 0 48 48" class="bg-neutral-700 rounded-full z-50">
                    <defs>
                        <mask id="ipSPlay0">
                            <g fill="black" stroke-linejoin="round" stroke-width="4">
                                <path fill="#fff" stroke="#fff" d="M24 44c11.046 0 20-8.954 20-20S35.046 4 24 4S4 12.954 4 24s8.954 20 20 20Z" />
                                <path fill="#000" stroke="#000" d="M20 24v-6.928l6 3.464L32 24l-6 3.464l-6 3.464z" />
                            </g>
                        </mask>
                    </defs>
                    <path fill="white" d="M0 0h48v48H0z" mask="url(#ipSPlay0)" />
                </svg>
            </div>
            <div class="z-0 absolute inset-0 col-span-1 bg-white {sc} transition-transform">
                ab
            </div>
        </button>
    </div>
</div>
{#if formVisible}
<div class="relative bg-red">
    <div class="absolute -left-48 -top-16 w-64 bg-white">
        Form
    </div>
</div>
{/if}