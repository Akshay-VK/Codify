<script>
export let fullType;
</script>

<div class="row-span-1">
  <ul class='flex flex-wrap -mb-px divide-x divide-[#203359] h-full place-content-stretch {fullType?'grid grid-cols-2':''}'  role="tablist">
    <slot {fullType}/>
  </ul>
</div>