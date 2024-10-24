<script>
    import CategorySidebar from "./CategorySidebar.svelte";
    import NoCategorySidebar from "./NoCategorySidebar.svelte";

    /** @type {{currentUser: any, handleSidebarClick: any, settings: any, channelInfo: any}} */
    let {
        currentUser = $bindable(),
        handleSidebarClick,
        settings = $bindable(),
        channelInfo = $bindable()
    } = $props();
</script>
{#if settings !== null && settings.appearance.sidebar.category_display_level === "None"}
    <NoCategorySidebar bind:currentUser={currentUser} handleSidebarClick={handleSidebarClick}
                       bind:channelInfo={channelInfo}
                       on:showAnnouncements on:showTasks on:showDmDialog on:showSettings/>
{:else}
    <CategorySidebar bind:currentUser={currentUser} handleSidebarClick={handleSidebarClick}
                     bind:settings={settings} bind:channelInfo={channelInfo}
                     on:showAnnouncements on:showTasks on:showDmDialog on:showSettings/>
{/if}
