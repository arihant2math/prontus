<script>
    import Sideitem from "./ChannelListItem.svelte";
    import {Collapsible} from "bits-ui";
    import {slide} from "svelte/transition";

    /** @type {{name: any, items: any, buttonClick: any, channelInfo: any}} */
    let {
        name,
        items,
        buttonClick,
        channelInfo
    } = $props();
    let show = $state(true);
    
</script>

<li class="select-none">
    <Collapsible.Root bind:open={show}>
        <Collapsible.Trigger
        class="text-sm flex items-center w-full p-2 text-gray-700 transition duration-75 rounded-lg group dark:text-gray-300 hover:text-gray-900 dark:hover:text-white">
            {#if show}
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-3 w-3">
                    <path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" />
                </svg>
            {:else}
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-3 h-3">
                    <path stroke-linecap="round" stroke-linejoin="round" d="m8.25 4.5 7.5 7.5-7.5 7.5" />
                </svg>
            {/if}
            <span class="ml-1 flex-1 text-left rtl:text-right whitespace-nowrap truncate text-sm font-medium">{name}</span>
        </Collapsible.Trigger>
        <Collapsible.Content
                transition={slide}>
            <ul class="py-2 space-y-2">
                {#each items as item}
                    {#if channelInfo !== null && item[0].id === channelInfo[0].id}
                        <Sideitem info={item[0]} stats={item[1]} membership={item[2]} buttonClick="{buttonClick}" active={true}/>
                    {:else}
                        <Sideitem info={item[0]} stats={item[1]} membership={item[2]} buttonClick="{buttonClick}"/>
                    {/if}
                {/each}
            </ul>
        </Collapsible.Content>
    </Collapsible.Root>
</li>