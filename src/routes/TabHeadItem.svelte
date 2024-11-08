<script>
  // adding active class
  //import classNames from 'classnames';
  export let id;
  export let activeTabValue
  export let inactiveClass = 'h-full text-center align-text-middle align-center inline-block py-4 px-4 text-sm text-center hover:bg-[#C2D2F2] text-[#C2D2F2] hover:text-[#0A1626] text-[#C2D2F2]'
  export let activeClass = 'h-full text-center align-text-middle align-center inline-block py-4 px-4 text-sm text-center bg-[#C2D2F2] text-[#0A1626]'
  const liClass = ''
</script>

<li class={liClass} role="presentation">
  <button
    on:click
    class={activeTabValue === id ? activeClass : inactiveClass}
    id="{id}-tabhead"
    type="button"
    role="tab">
    <slot />
  </button>
</li>