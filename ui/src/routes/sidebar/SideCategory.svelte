<script>
    import Sideitem from "./ChannelCard.svelte";

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
            class="text-sm flex items-center w-full p-2 text-base text-gray-900 transition duration-75 rounded-lg group dark:text-white hover:text-gray-700 dark:hover:text-gray-300"
            aria-controls="dropdown-example" data-collapse-toggle="dropdown-example">
        <span class="flex-1 ms-3 text-left rtl:text-right whitespace-nowrap truncate">{name}</span>
        <svg class="w-2 h-2" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
        </svg>
    </button>
    <ul id="{dropdownId}" class="hidden py-2 space-y-2">
        {#each items as item}
            <li><Sideitem info={item[0]} stats={item[1]} buttonClick="{buttonClick}"/></li>
        {/each}
    </ul>
</li>