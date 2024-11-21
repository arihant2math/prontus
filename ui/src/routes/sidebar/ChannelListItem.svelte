<script>
    import ProfilePicture from "../user/ProfilePicture.svelte";
    import {readChannel, setChannelAlias, setChannelMute, setChannelPin} from "$lib/api.ts";
    import { ContextMenu } from "bits-ui";
    import ContextMenuContent from "../bitsHead/ContextMenuContent.svelte";

    /** @type {{info: any, stats: any, membership: any, buttonClick: any, active?: boolean}} */
    let {
        info,
        stats,
        membership,
        buttonClick,
        active = false
    } = $props();
    let listItem = $state(null);
    let title = $derived(membership.alias === null ? info.title : membership.alias);
    let textColor = $derived(membership.mute ? "text-gray-500 dark:text-gray-400" : "text-gray-900 dark:text-white");
    let unreadString = $derived(stats.unread > 99 ? "99+" : stats.unread);
    let mentionString = $derived(stats.unread_mentions > 99 ? "99+" : stats.unread_mentions);
    let fontWeight = $derived(stats.unread > 0 ? "font-bold" : "font-medium");
    let bgColor = $derived(active ? "bg-blue-600 dark:bg-blue-500 hover:bg-blue-700 dark:hover:bg-blue-600 text-white" : "hover:bg-gray-100 dark:hover:bg-slate-700");
    let contextMenuOpen = $state(false);

    function btnClick() {
        buttonClick(info.id);
    }

    async function addListener() {
        while (listItem === null) {
            await new Promise(r => setTimeout(r, 50));
        }
        window.addEventListener("auxclick", (e) => {
            if (e !== null && listItem !== null) {
                if (!listItem.contains(e.target)) {
                    contextMenuOpen = false;
                }
            }
        });
        // TODO: listener
    }

    addListener();
</script>
<ContextMenu.Root bind:open={contextMenuOpen}>
    <ContextMenu.Trigger>
        <li class="select-none" bind:this={listItem} onload={addListener}>
            <button onclick={() => {btnClick()}}
                    class="flex-0 flex items-start p-2 {textColor} {bgColor} transition duration-75 rounded-lg pl-4 group w-full text-ellipsis">
                {#if info.isdm}
                    <div class="relative">
                        <ProfilePicture user={info.dmpartner}/>
                        {#if !info.dmpartner.isonline}
                            <span class="bottom-0 left-7 absolute bg-gray-500 dark:bg-gray-600 w-3.5 h-3.5 border-2 border-white dark:border-gray-800 rounded-full"></span>
                        {:else}
                            <span class="bottom-0 left-7 absolute w-3.5 h-3.5 bg-green-400 border-2 border-white dark:border-gray-800 rounded-full"></span>
                        {/if}
                    </div>
                {/if}
                <span class="flex-1 text-sm text-left rtl:text-right ms-3 whitespace-nowrap text-truncate truncate {fontWeight}">{title}</span>
                {#if stats.unread > 0 && !membership.mute}
                    {#if stats.unread_mentions > 0}
                        <span class="inline-flex items-center justify-center px-2 ms-3 text-xs font-medium text-white bg-red-600 rounded-full dark:text-white w-fit">{mentionString}</span>
                    {:else}
                        <span class="inline-flex items-center justify-center px-2 ms-3 text-xs font-medium text-gray-800 bg-gray-100 rounded-full dark:bg-gray-600 dark:text-gray-300 w-fit">{unreadString}</span>
                    {/if}
                {/if}
            </button>
        </li>
    </ContextMenu.Trigger>
    <ContextMenuContent>
        <ContextMenu.Item onclick={() => {btnClick()}} class="hover:bg-gray-100 dark:hover:bg-slate-700 flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium outline-none !ring-0 !ring-transparent data-[highlighted]:bg-muted">
            Open
        </ContextMenu.Item>
        <ContextMenu.Item onclick={() => {readChannel(info.id)}} class="hover:bg-gray-100 dark:hover:bg-slate-700 flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium outline-none !ring-0 !ring-transparent data-[highlighted]:bg-muted">
            Mark as read
        </ContextMenu.Item>
        <ContextMenu.Item onclick={() => {}} class="hover:bg-gray-100 dark:hover:bg-slate-700 flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium outline-none !ring-0 !ring-transparent data-[highlighted]:bg-muted">
            Set Alias
        </ContextMenu.Item>
        {#if membership.alias !== null}
            <ContextMenu.Item onclick={() => {setChannelAlias(info.id, null)}} class="hover:bg-gray-100 dark:hover:bg-slate-700 flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium outline-none !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                Remove alias
            </ContextMenu.Item>
        {/if}
        <ContextMenu.Item onclick={() => {setChannelMute(info.id, !membership.mute)}} class="hover:bg-gray-100 dark:hover:bg-slate-700 flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium outline-none !ring-0 !ring-transparent data-[highlighted]:bg-muted">
            {membership.mute ? "Unmute" : "Mute"}
        </ContextMenu.Item>
        <ContextMenu.Item onclick={() => {setChannelPin(info.id, !membership.pin)}} class="hover:bg-gray-100 dark:hover:bg-slate-700 flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium outline-none !ring-0 !ring-transparent data-[highlighted]:bg-muted">
            {membership.pin ? "Unpin" : "Pin"}
        </ContextMenu.Item>
        <!-- TODO: -->
        <ContextMenu.Item onclick={() => {}} class="hover:bg-gray-100 dark:hover:bg-slate-700 flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium outline-none !ring-0 !ring-transparent data-[highlighted]:bg-muted">
            Hide
        </ContextMenu.Item>
    </ContextMenuContent>
</ContextMenu.Root>
