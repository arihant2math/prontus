<script>
    import CategorySidebar from "./CategorySidebar.svelte";
    import NoCategorySidebar from "./NoCategorySidebar.svelte";

    /** @type {{currentUser: any, handleSidebarClick: any, settings: any, channelInfo: any}} */
    let {
        currentUser = $bindable(),
        settings = $bindable(),
        channelInfo = $bindable(),
        ...on
    } = $props();
</script>
{#if settings !== null && settings.appearance.sidebar.category_display_level === "None"}
    <NoCategorySidebar bind:currentUser={currentUser}
                       bind:channelInfo={channelInfo}
                       {...on}/>
{:else}
    <CategorySidebar bind:currentUser={currentUser}
                     bind:settings={settings}
                     bind:channelInfo={channelInfo}
                     {...on}/>
{/if}
