<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';
	import type { TAction, TOutputData } from '$lib';
	import { getContext } from 'svelte';
	import type { Writable } from 'svelte/store';

	export let action: TAction;
	export let baseLocation: string;

	let switched = false;

	let formelm: HTMLDialogElement;

	async function clicked() {
		console.log(action.name, ' clicked');
		switched = !switched;
		if (switched) {
			//CALL ACTION
			if (action.arguments?.length > 0) {
				formelm.showModal();
			} else {
				let args = new Array<string>(0);
				let output = await invoke('run_command_stream', { baseLocation, action, args });
			}
			//console.log(output);

			switched = false;
		}
	}

	let logged = '';
	let n = 0;
	let logHandle = getContext<Writable<TOutputData[]>>('log');
	listen('output_data', (event: any) => {
		n++;
		//if (event.payload.data != logged) {
		console.log(n,event.payload);
		//logHandle.update((logs: TOutputData[]) => (logs[logs.length-1].line == event.payload.line ? logs : logs.concat(event.payload)));
		logHandle.update((logs: TOutputData[]) => (logs.concat(event.payload)));
		console.log($logHandle);
		//} "vfvfv"
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

<div
	class="w-full h-full font-['Space Grotesk'] bg-clip-content grid grid-cols-5 place-content-stretch place-items-center divide-x divide-y divide-[#203359] gap-0 hover:bg-[#C2D2F2] text-[#C2D2F2] hover:text-[#0A1626] text-[#C2D2F2] hover:divide-[#C2D2F2]"
>
	<div class="col-span-4 w-full divide-x divide-y divide-[#203359]">
		{action.name}
	</div>
	<button
		class="col-span-1 w-full h-full grid place-items-center divide-x divide-y divide-[#203359]"
		on:click={clicked}
	>
		<svg xmlns="http://www.w3.org/2000/svg" width="2em" height="2em" viewBox="0 0 48 48">
			<path
				fill="currentColor"
				stroke="currentColor"
				stroke-linejoin="round"
				stroke-width="4"
				d="M15 24V11.876l10.5 6.062L36 24l-10.5 6.062L15 36.124z"
			/>
		</svg>
	</button>
</div>
<dialog bind:this={formelm} class="w-128 h-fit p-4 bg-[#0A1626] border border-1 border-[#203359]">
	<div class="w-fit py-2">
		<h3>{action.name} parameters</h3>
	</div>
	<div class="grid grid-flow-row py-2">
		{#each action.arguments as arg}
			<div class="w-fit p-2 bg-[#0A1626]">
				<input
					placeholder={arg}
					bind:value={parameters[arg]}
					class="bg-[#0A1626] border-neutral-500 placeholder:text-neutral-500 placeholder:italic border shadow-sm focus:outline-none focus:border-white rounded-md p-2"
				/>
			</div>
		{/each}
	</div>
	<div class="w-full grid grid-flow-col gap-4">
		<button
			on:click={() => submitFormElm()}
			class="w-full h-full m-2 bg-green-700 hover:bg-green-500 text-green-200">Run</button
		>
		<button
			on:click={() => closeFormElm()}
			class="w-full h-full m-2 bg-[#0A1626] hover:bg-neutral-700 text-neutral-500 border-neutral-500 border"
			>Close</button
		>
	</div>
</dialog>
