<script>
    import Sideitem from "./sideitem.svelte";

    export let name;
    export let items;
    export let buttonClick;

    $: dropdownId = name + "SidebarDropdown";

    function handleDropdownToggle() {
        console.log("Toggling dropdown");
        let dropdown = document.getElementById(dropdownId);
        dropdown.classList.toggle("hidden");
    }
</script>

<li>
    <button on:click={handleDropdownToggle} type="button"
            class="flex items-center w-full p-2 text-base text-gray-900 transition duration-75 rounded-lg group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
            aria-controls="dropdown-example" data-collapse-toggle="dropdown-example">
        <span class="flex-1 ms-3 text-left rtl:text-right whitespace-nowrap truncate">{name}</span>
        <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
        </svg>
    </button>
    <ul id="{dropdownId}" class="hidden py-2 space-y-2">
        {#each items as item}
            <li><Sideitem bubbleId={item.id} name={item.title} notifications={item.notifications} mention={item.mention} buttonClick={buttonClick}/></li>
        {/each}
    </ul>
</li>