<script>
  // adding active class
  //import classNames from 'classnames';
  export let id;
  export let fullType;
  export let activeTabValue
  export let inactiveClass = 'h-full text-center align-text-middle align-center py-4 px-4 text-center hover:bg-[#C2D2F2] text-[#C2D2F2] hover:text-[#0A1626] text-[#C2D2F2] text-sm'+(fullType?'col-span-1':'')
  export let activeClass = 'h-full text-center align-text-middle align-center py-4 px-4 text-center bg-[#C2D2F2] text-[#0A1626] text-sm'+(fullType?'col-span-1':'')
  const liClass = ' w-full'
</script>

<li role="presentation"

    on:click
    class={activeTabValue === id ? activeClass : inactiveClass}
    id="{id}-tabhead"
    >
    <slot />

</li>