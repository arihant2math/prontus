<script>
    import CategorySidebar from "./CategorySidebar.svelte";
    import NoCategorySidebar from "./NoCategorySidebar.svelte";

    export let currentUser;
    export let showSettings;
    export let handleSidebarClick;
    export let settings;
</script>
{#if settings !== null && settings.appearance.sidebar.category_display_level === "None"}
    <NoCategorySidebar bind:currentUser={currentUser} showSettings={showSettings} handleSidebarClick={handleSidebarClick}
                       bind:settings={settings}
                       on:showAnnouncements on:showTasks on:showDmDialog/>
{:else}
    <CategorySidebar bind:currentUser={currentUser} showSettings={showSettings} handleSidebarClick={handleSidebarClick}
             bind:settings={settings}
             on:showAnnouncements on:showTasks on:showDmDialog/>
{/if}
