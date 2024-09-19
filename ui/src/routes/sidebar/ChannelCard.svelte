<script>
    import ProfilePicture from "../user/ProfilePicture.svelte";
    import {ContextMenu} from "bits-ui";

    export let info;
    export let stats;
    export let buttonClick;
    $: unreadString = stats.unread > 99 ? "99+" : stats.unread;
    $: mentionString = stats.unread_mentions > 99 ? "99+" : stats.unread_mentions;
    $: fontWeight = stats.unread > 0 ? "font-bold" : "font-medium";

    function btnClick() {
        buttonClick(info.id);
    }
</script>
<li>
    <!--TODO: Fix how mentions/unread count works-->
    <ContextMenu.Root>
        <ContextMenu.Trigger class="">
            <button on:click={btnClick}
                    class="flex items-start p-2 text-gray-900 transition duration-75 rounded-lg pl-4 group hover:bg-gray-100 dark:text-white dark:hover:bg-slate-700 w-full text-ellipsis">
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
                <span class="text-sm text-left ms-3 flex-1 whitespace-nowrap text-truncate {fontWeight}">{info.title}</span>
                { #if stats.unread > 0 }
                    {#if stats.unread_mentions > 0}
                        <span class="inline-flex items-center justify-center w-3 h-3 p-3 ms-3 text-xs font-medium text-white bg-red-600 rounded-full dark:text-white w-fit">{mentionString}</span>
                    {:else}
                        <span class="inline-flex items-center justify-center px-2 ms-3 text-xs font-medium text-gray-800 bg-gray-100 rounded-full dark:bg-gray-600 dark:text-gray-300 w-fit">{unreadString}</span>
                    {/if}
                {/if}
            </button>
        </ContextMenu.Trigger>
        <ContextMenu.Content class="z-50 w-full max-w-max rounded-xl bg-white dark:bg-slate-700 px-1 py-1.5 shadow-popover outline-none">
            <ContextMenu.Item
                    class="flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium outline-none !ring-0 !ring-transparent data-[highlighted]:bg-muted hover:bg-gray-100 dark:hover:bg-slate-600">
                <div class="flex items-center space-x-2">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 10.5a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
                        <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 10.5c0 7.142-7.5 11.25-7.5 11.25S4.5 17.642 4.5 10.5a7.5 7.5 0 1 1 15 0Z" />
                    </svg>
                    <p>Pin</p>
                </div>
            </ContextMenu.Item>
            <ContextMenu.Item
                    class="flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium outline-none !ring-0 !ring-transparent data-[highlighted]:bg-muted hover:bg-gray-100 dark:hover:bg-slate-600">
                <div class="flex items-center space-x-2">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M3.98 8.223A10.477 10.477 0 0 0 1.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.451 10.451 0 0 1 12 4.5c4.756 0 8.773 3.162 10.065 7.498a10.522 10.522 0 0 1-4.293 5.774M6.228 6.228 3 3m3.228 3.228 3.65 3.65m7.894 7.894L21 21m-3.228-3.228-3.65-3.65m0 0a3 3 0 1 0-4.243-4.243m4.242 4.242L9.88 9.88" />
                    </svg>
                    <p>Hide</p>
                </div>
            </ContextMenu.Item>
        </ContextMenu.Content>
    </ContextMenu.Root>
</li>