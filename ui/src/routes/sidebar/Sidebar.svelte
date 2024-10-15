<script>
    import SideCategory from "./sidebar/SideCategory.svelte";
    import {listen} from "@tauri-apps/api/event";
    import {getChannelList} from "$lib/api.ts";

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
    <DefaultSidebar bind:currentUser={currentUser} showSettings={showSettings} handleSidebarClick={handleSidebarClick}
             bind:settings={settings}
             on:showAnnouncements on:showTasks on:showDmDialog/>
{/if}
