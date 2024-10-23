<script>
    import ProfilePicture from "../user/ProfilePicture.svelte";
    import {readChannel, setChannelMute, setChannelPin} from "$lib/api.ts";
    import {MenuItem} from "@tauri-apps/api/menu/menuItem";
    import {Menu} from "@tauri-apps/api/menu/menu";

    export let info;
    export let stats;
    export let membership;
    export let buttonClick;
    export let active = false;
    let listItem = null;
    $: title = membership.alias === null ? info.title : membership.alias;
    $: textColor = membership.mute ? "text-gray-500 dark:text-gray-400" : "text-gray-900 dark:text-white";
    $: unreadString = stats.unread > 99 ? "99+" : stats.unread;
    $: mentionString = stats.unread_mentions > 99 ? "99+" : stats.unread_mentions;
    $: fontWeight = stats.unread > 0 ? "font-bold" : "font-medium";
    $: bgColor = active ? "bg-blue-600 dark:bg-blue-500 hover:bg-blue-700 dark:hover:bg-blue-600 text-white" : "hover:bg-gray-100 dark:hover:bg-slate-700";

    function btnClick() {
        buttonClick(info.id);
    }

    async function showContextMenu(e) {
        e.preventDefault();

        let menuItemsPromise = [];

        menuItemsPromise.push(MenuItem.new({
            text: 'Open',
            action: () => {
                buttonClick(info.id);
            },
        }));

        if (stats.unread > 0) {
            menuItemsPromise.push(MenuItem.new({
                text: 'Mark as Read',
                action: () => {
                    readChannel(info.id);
                },
            }));
        }
        if (membership.is_pinned) {
            menuItemsPromise.push(MenuItem.new({
                text: 'Unpin',
                action: () => {
                    setChannelPin(info.id, false);
                },
            }));
        } else {
            menuItemsPromise.push(MenuItem.new({
                text: 'Pin',
                action: () => {
                    setChannelPin(info.id, true);
                },
            }));
        }

        if (membership.mute) {

        } else {
            menuItemsPromise.push(MenuItem.new({
                text: 'Unmute',
                action: () => {
                    setChannelMute(false)
                },
            }));
        }

        menuItemsPromise.push(MenuItem.new({
            text: 'Hide',
            action: () => {
                // TODO
            }
        }))
        // PredefinedMenuItem.new({ item: 'Separator' }),

        let menuItems = await Promise.all(menuItemsPromise);

        const menu = await Menu.new({
            items: menuItems,
        });

        await menu.popup();
    }

    async function addListener() {
        while (listItem === null) {
            await new Promise(r => setTimeout(r, 50));
        }
        listItem.addEventListener("contextmenu", (event) => {
            showContextMenu(event)
        });
    }

    addListener();
</script>
<li class="select-none" bind:this={listItem} on:load={addListener}>
    <button on:click={btnClick}
            class="flex-0 flex items-start p-2 {textColor} {bgColor} transition duration-75 rounded-lg pl-4 group w-full text-ellipsis">
        {#if info.isdm}
            <div class="relative">
                <ProfilePicture user="{info.dmpartner}"/>
                {#if !info.dmpartner.isonline}
                    <span class="bottom-0 left-7 absolute bg-gray-500 dark:bg-gray-600 w-3.5 h-3.5 border-2 border-white dark:border-gray-800 rounded-full"></span>
                {:else}
                    <span class="bottom-0 left-7 absolute w-3.5 h-3.5 bg-green-400 border-2 border-white dark:border-gray-800 rounded-full"></span>
                {/if}
            </div>
        {/if}
        <span class="flex-1 text-sm text-left rtl:text-right ms-3 flex-1 whitespace-nowrap text-truncate truncate {fontWeight}">{title}</span>
        {#if stats.unread > 0 && !membership.mute}
            {#if stats.unread_mentions > 0}
                <span class="inline-flex items-center justify-center px-2 ms-3 text-xs font-medium text-white bg-red-600 rounded-full dark:text-white w-fit">{mentionString}</span>
            {:else}
                <span class="inline-flex items-center justify-center px-2 ms-3 text-xs font-medium text-gray-800 bg-gray-100 rounded-full dark:bg-gray-600 dark:text-gray-300 w-fit">{unreadString}</span>
            {/if}
        {/if}
    </button>
</li>