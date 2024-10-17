<script>
    import CategorySidebar from "./CategorySidebar.svelte";
    import NoCategorySidebar from "./NoCategorySidebar.svelte";

    export let currentUser;
    export let handleSidebarClick;
    export let settings;
</script>
{#if settings !== null && settings.appearance.sidebar.category_display_level === "None"}
    <NoCategorySidebar bind:currentUser={currentUser} handleSidebarClick={handleSidebarClick}
                       on:showAnnouncements on:showTasks on:showDmDialog on:showSettings/>
{:else}
    <CategorySidebar bind:currentUser={currentUser} handleSidebarClick={handleSidebarClick}
             bind:settings={settings}
             on:showAnnouncements on:showTasks on:showDmDialog on:showSettings/>
{/if}
