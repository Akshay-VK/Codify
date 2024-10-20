<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';
	import type { TAction } from '$lib';
	import {getContext} from 'svelte';    
	import type { Writable } from 'svelte/store';

	export let action: TAction;
	export let baseLocation: string;

	let switched = false;
	let sc = 'scale-0';
	let tc = 'text-white';

	let formPresent = action.arguments?.length > 0;
	let formelm: HTMLDialogElement;

	async function clicked() {
		console.log(action.name);
		switched = !switched;
		if (switched) {
			sc = 'scale-[8]';
			tc = 'text-black';
			//CALL ACTION
			if (formPresent) {
				formelm.showModal();
			} else {
				let args = new Array<string>(0);
				let output = await invoke('run_command_stream', { baseLocation, action, args });
			}
			//console.log(output);

			sc = 'scale-0';
			tc = 'text-white';
			switched = false;
		} else {
			sc = 'scale-0';
			tc = 'text-white';
		}
	}

	let logged = '';
	let n = 0;
	let logHandle = getContext<Writable<string[]>>('log');
	listen('output_data', (event: any) => {
		n++;
		//if (event.payload.data != logged) {
        console.log(n,event.payload.data);
        logged = event.payload.data as string;
		logHandle.update((logs:string[])=>logs.concat(event.payload.data))
        
		//}
	});

	let parameters: { [k: string]: string } = {};

	function closeFormElm() {
		for (const p in parameters) {
			parameters[p] = '';
		}
		formelm.close();
	}
	async function submitFormElm() {
		let args = [];
		for (let i = 0; i < action.arguments.length; i++) {
			args.push(parameters[action.arguments[i]]);
		}
		//console.log(parameters, args, action);
		let output = await invoke('run_command_stream', { baseLocation, action, args });
		formelm.close();
	}
</script>

<div class="w-full font-['Space Grotesk'] bg-clip-content">
	<div
		class="bg-neutral-700 hover:bg-neutral-800 p-4 grid grid-cols-4 rounded-3xl items-center transition-colors overflow-hidden bg-clip-padding"
	>
		<div class="z-50 col-span-3 text-center align-text-middle h-full {tc} transition-colors">
			{action.name}
		</div>
		<button class="col-span-1 relative" on:click={clicked}>
			<div class="z-50">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="2em"
					height="2em"
					viewBox="0 0 48 48"
					class="bg-neutral-700 rounded-full z-50"
				>
					<defs>
						<mask id="ipSPlay0">
							<g fill="black" stroke-linejoin="round" stroke-width="4">
								<path
									fill="#fff"
									stroke="#fff"
									d="M24 44c11.046 0 20-8.954 20-20S35.046 4 24 4S4 12.954 4 24s8.954 20 20 20Z"
								/>
								<path
									fill="#000"
									stroke="#000"
									d="M20 24v-6.928l6 3.464L32 24l-6 3.464l-6 3.464z"
								/>
							</g>
						</mask>
					</defs>
					<path fill="white" d="M0 0h48v48H0z" mask="url(#ipSPlay0)" />
				</svg>
			</div>
			<div class="z-0 absolute inset-0 col-span-1 bg-white {sc} transition-transform">ab</div>
		</button>
	</div>
</div>
<dialog
	bind:this={formelm}
	class="min-w-64 w-fit h-fit p-4 rounded-xl bg-neutral-800 shadow-lg shadow-neutral-900"
>
	<div class="w-fit py-2">
		<h3>{action.name} parameters</h3>
	</div>
	<div class="grid grid-flow-row gap-4 py-2">
		{#each action.arguments as arg}
			<div class="w-fit p-2 bg-neutral-800 rounded">
				<input
					placeholder={arg}
					bind:value={parameters[arg]}
					class="bg-neutral-700 border-neutral-500 placeholder:text-neutral-500 placeholder:italic border shadow-sm focus:outline-none focus:border-white rounded-md p-2"
				/>
			</div>
		{/each}
	</div>
	<div class="w-full grid grid-flow-col gap-4">
		<button
			on:click={() => submitFormElm()}
			class="w-full p-2 rounded-md bg-green-700 hover:bg-green-500 text-green-200">Run</button
		>
		<button
			on:click={() => closeFormElm()}
			class="w-full p-2 rounded-md bg-neutral-800 hover:bg-neutral-700 text-neutral-500 border-neutral-500 border"
			>Close</button
		>
	</div>
</dialog>
