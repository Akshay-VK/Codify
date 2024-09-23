<script>
	import { FAB } from 'm3-svelte';
	import iconEdit from '@ktibow/iconset-material-symbols/edit-outline';

	export let action;
</script>

<div class="grid grid-cols-4 bg-neutral-700 hover:bg-neutral-600 font-['Space Grotesk'] p-4 rounded-2xl align-middle align-center items-center transition-colors">
    <div class="col-span-3 w-full h-full align-middle align-center items-center">{action.name}</div>
    <div class="col-span-1">
        <svg xmlns="http://www.w3.org/2000/svg" width="2.5em" height="2.5em" viewBox="0 0 24 24">
            <path fill="white" d="M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2m-2 14.5v-9l6 4.5z"/>
        </svg>
    </div>
</div>

<!-- <FAB color="primary" size="normal" icon={iconEdit} text={action.name} /> -->
<!-- <div class="w-[175px] h-[75px] relative rounded-[25px]">
    <div class="w-[175px] h-[75px] left-0 top-0 absolute bg-[#191919] rounded-[25px]"></div>
    <div class="w-[294px] h-[294px] left-[-7px] top-[-110px] absolute bg-[#e4e0d3] rounded-full"></div>
    <div class="w-10 h-10 p-1 left-[120px] top-[17px] absolute flex-col justify-center items-center gap-2.5 inline-flex">
    <div class="bg-[#e4e0d3] rounded-[100px] justify-center items-center gap-2.5 inline-flex">
        <div class="p-2 justify-center items-center gap-2.5 flex">
        <div class="w-6 h-6 pl-2 pr-[5px] py-[5px] justify-center items-center flex"></div>
        </div>
    </div>
    </div>
    <div class="w-[85px] left-[30px] top-[23px] absolute text-[#191919] text-[22px] font-normal font-['Space Grotesk']">{action.name}</div>
</div>
<div class="w-[175px] h-[75px] relative rounded-[25px]">
    <div class="w-[175px] h-[75px] left-0 top-0 absolute bg-[#3d3b34] rounded-[25px]"></div>
    <div class="w-10 h-10 left-[120px] top-[17px] absolute bg-[#e4e0d3] rounded-full"></div>
    <div class="w-10 h-10 p-1 left-[120px] top-[17px] absolute flex-col justify-center items-center gap-2.5 inline-flex">
    <div class="bg-[#e4e0d3] rounded-[100px] justify-center items-center gap-2.5 inline-flex">
        <div class="p-2 justify-center items-center gap-2.5 flex">
        <div class="w-6 h-6 pl-2 pr-[5px] py-[5px] justify-center items-center flex"></div>
        </div>
    </div>
    </div>
    <div class="w-[85px] left-[30px] top-[23px] absolute text-[#e4e0d3] text-[22px] font-normal font-['Space Grotesk']">{action.name}</div>
</div> -->