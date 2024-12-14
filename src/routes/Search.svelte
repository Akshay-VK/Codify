<script lang="ts">

    type SearchForm={
        name: string,
        extension: string,
        type: string,
        beginfrom: string
    };
    let form: SearchForm={
        name:"",
        extension:"",
        type:"",
        beginfrom:""
    }
    function search(e: Event){
        console.log(e);
        console.log(form);
    }
</script>

<div class="h-full w-full place-content-stretch place-items-center text-center align-text-middle align-center grid grid-rows-12 items-center divide-[#203359] divide-y">
    <div class="text-[#C2D2F2] text-lg font-['Space Grotesk'] place-content-center text-center place-items-center align-text-middle w-full h-full row-span-1">
        Search
    </div>
    <div class="w-full h-full place-content-stretch row-span-11">
        <form on:submit|preventDefault={search} class="w-full h-5/6 grid grid-rows-14 place-items-left align-text-middle align-center text-left divide-[#203359] divide-y">

            <label for="name" class="text-sm row-span-1 align-text-middle place-content-center px-2">Name to search</label>
            <div class="row-span-2 grid place-content-stretch">
                <input type="text" id="name" name="name" bind:value={form.name} required class="bg-[#051020] w-full h-full"/>
            </div>

            <label for="extension" class="text-sm row-span-1 align-text-middle place-content-center px-2">File extension <i>(if applicable)</i></label>
            <div class="row-span-2 grid place-content-stretch">
                <input type="text" id="extension" name="extension" bind:value={form.extension} class="bg-[#051020] w-full h-full"/>
            </div>

            <legend class="text-sm row-span-1 place-content-center px-2">Type</legend>
            <div class="row-span-3 grid grid-rows-2 grid-cols-2 grid-flow-row">
                <div class="w-full h-full place-content-center">
                    <input type="radio" id="any" name="type" value="any" bind:group={form.type} checked class="bg-[#0A1626]"/>
                    <label for="any">Any</label>
                </div>
                <div class="w-full h-full place-content-center">
                    <input type="radio" id="anyfile" name="type" value="anyfile" bind:group={form.type} class="bg-[#0A1626]"/>
                    <label for="anyfile">Any file</label>
                </div>
                <div class="w-full h-full place-content-center">
                    <input type="radio" id="dotfile" name="type" value="dotfile" bind:group={form.type} class="bg-[#0A1626]"/>
                    <label for="dotfile"> Dot file</label>
                </div>
                <div class="w-full h-full place-content-center">
                    <input type="radio" id="folder" name="type" value="folder" bind:group={form.type} class="bg-[#0A1626]"/>
                    <label for="folder">Folder</label>
                </div>
            </div>

            <legend class="text-sm row-span-1 place-content-center px-2">Begin search from path</legend>
            <div class="row-span-2"><input type="text" id="beginfrom" name="beginfrom" placeholder="C:/" bind:value={form.beginfrom} class="bg-[#051020] w-full h-full"/></div>

            <input type="submit" value="Search" class="row-span-1 text-center w-full py-2 hover:bg-[#C2D2F2] text-[#C2D2F2] hover:text-[#0A1626]"/>
            <div></div>
        </form>
    </div>
</div>