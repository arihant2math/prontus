<script>
    import {DropdownMenu} from "bits-ui";
    import {Popover} from "bits-ui";
    import {setChannelMute, setChannelNotifications} from "$lib/api.ts";

    /** @type {{info?: any, memberListActive?: boolean}} */
    let { info = $bindable(null), memberListActive = $bindable(false) } = $props();

    let memberListFill = $derived(memberListActive ? "currentColor" : "none");

    function toggleMemberList() {
        memberListActive = !memberListActive;
    }

    async function updateMute() {
        await setChannelMute(info[0].id, info[2].mute);
    }

    async function updateNotificationPreference() {
        await setChannelNotifications(info[0].id, info[2].notificationpreference);
    }
</script>
{#if info !== null && info[0] !== undefined}
    <div class="h-[60px] border-b border-gray-500 flex w-full items-center text-gray-900 dark:text-white text-lg my-auto px-5 flex-row dark:bg-slate-900">
        <div class="flex flex-col">
            <span class="text-nowrap">{info[0].title}</span>
            {#if info[0].category !== null}
                <span class="text-sm text-gray-500 dark:text-gray-400">{info[0].category.title}</span>
            {:else if info[0].isdm}
                <span class="text-sm text-gray-500 dark:text-gray-400">Direct Messages</span>
            {:else}
                <span class="text-sm text-gray-500 dark:text-gray-400">Uncategorized</span>
            {/if}
        </div>
        <div class="ml-auto">
            <Popover.Root>
                <Popover.Trigger>
                    <button class="hover:bg-gray-300 dark:hover:bg-slate-700 px-1 py-2 rounded-lg">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                             stroke="currentColor" class="size-6">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                  d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967 0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714 0"/>
                        </svg>
                    </button>
                </Popover.Trigger>
                <Popover.Content
                        class="z-30 w-full max-w-[328px] rounded-lg bg-white dark:bg-slate-800 p-4 shadow-lg flex-col"
                        sideOffset={8}>
                    <label class="inline-flex items-center cursor-pointer p-3">
                        <input type="checkbox" value={info[2].mute} class="sr-only peer" onclick={updateMute}>
                        <div class="relative w-11 h-6 bg-gray-200 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:w-5 after:h-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
                        <span class="ms-3 text-sm font-medium text-gray-900 dark:text-gray-300">Mute Channel</span>
                    </label>
                    <ul class="p-3 space-y-1 text-sm text-gray-700 dark:text-gray-200">
                        <li>
                            <div class="flex p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                                <div class="flex items-center h-5">
                                    <input id="helper-radio-4" name="helper-radio" type="radio" value="ALL" bind:group={info[2].notificationpreference} onclick={updateNotificationPreference} class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500">
                                </div>
                                <div class="ms-2 text-sm">
                                    <label for="helper-radio-4" class="font-medium text-gray-900 dark:text-gray-300">
                                        <div>All Messages</div>
<!--                                        <p id="helper-radio-text-4" class="text-xs font-normal text-gray-500 dark:text-gray-300">Some helpful instruction goes over here.</p>-->
                                    </label>
                                </div>
                            </div>
                        </li>
                        <li>
                            <div class="flex p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                                <div class="flex items-center h-5">
                                    <input id="helper-radio-5" name="helper-radio" type="radio" value="MENTIONS" bind:group={info[2].notificationpreference} onclick={updateNotificationPreference} class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500">
                                </div>
                                <div class="ms-2 text-sm">
                                    <label for="helper-radio-5" class="font-medium text-gray-900 dark:text-gray-300">
                                        <div>Only mentions</div>
                                        <p id="helper-radio-text-5" class="text-xs font-normal text-gray-500 dark:text-gray-300">Including @everyone and @here.</p>
                                    </label>
                                </div>
                            </div>
                        </li>
                        <li>
                            <div class="flex p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                                <div class="flex items-center h-5">
                                    <input id="helper-radio-6" name="helper-radio" type="radio" value="NONE" bind:group={info[2].notificationpreference} onclick={updateNotificationPreference} class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500">
                                </div>
                                <div class="ms-2 text-sm">
                                    <label for="helper-radio-6" class="font-medium text-gray-900 dark:text-gray-300">
                                        <div>Nothing</div>
<!--                                        <p id="helper-radio-text-6" class="text-xs font-normal text-gray-500 dark:text-gray-300">Some helpful instruction goes over here.</p>-->
                                    </label>
                                </div>
                            </div>
                        </li>
                    </ul>
                </Popover.Content>
            </Popover.Root>
            <button class="hover:bg-gray-300 dark:hover:bg-slate-700 px-1 py-2 rounded-lg">
                <svg xmlns="http://www.w3.org/2000/svg" fill="{memberListFill}" viewBox="0 0 24 24" stroke-width="1.5"
                     stroke="currentColor" class="size-6" onclick={toggleMemberList}>
                    <path stroke-linecap="round" stroke-linejoin="round"
                          d="M15 19.128a9.38 9.38 0 0 0 2.625.372 9.337 9.337 0 0 0 4.121-.952 4.125 4.125 0 0 0-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 0 1 8.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0 1 11.964-3.07M12 6.375a3.375 3.375 0 1 1-6.75 0 3.375 3.375 0 0 1 6.75 0Zm8.25 2.25a2.625 2.625 0 1 1-5.25 0 2.625 2.625 0 0 1 5.25 0Z"/>
                </svg>
            </button>
            <button class="hover:bg-gray-300 dark:hover:bg-slate-700 px-1 py-2 rounded-lg" disabled>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                     stroke="currentColor" class="size-6">
                    <path stroke-linecap="round" stroke-linejoin="round"
                          d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z"/>
                </svg>
            </button>
            <DropdownMenu.Root>
                <DropdownMenu.Trigger
                        class="hover:bg-gray-300 dark:hover:bg-slate-700 px-1 py-2 rounded-lg">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                         stroke="currentColor" class="size-6">
                        <path stroke-linecap="round" stroke-linejoin="round"
                              d="M12 6.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 12.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 18.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5Z"/>
                    </svg>
                </DropdownMenu.Trigger>
                <DropdownMenu.Content
                        class="w-full max-w-[229px] rounded-xl bg-white dark:bg-slate-800 px-1 py-1.5 shadow-popover"
                        sideOffset={8}>
                    {#if !info[0].isdm}
                        <DropdownMenu.Item
                                class="flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                            <div class="flex items-center">
                                Invites
                            </div>
                        </DropdownMenu.Item>
                    {/if}
                    <DropdownMenu.Item
                            class="flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                        <div class="flex items-center">
                            Files
                        </div>
                    </DropdownMenu.Item>
                    <DropdownMenu.Item
                            class="flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                        <div class="flex items-center">
                            Photos & Videos
                        </div>
                    </DropdownMenu.Item>
                    {#if !info[0].isdm}
                        <DropdownMenu.Item
                                class="flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                        >
                            <div class="flex items-center">
                                Settings
                            </div>
                        </DropdownMenu.Item>
                    {/if}
                </DropdownMenu.Content>
            </DropdownMenu.Root>
        </div>
    </div>
{/if}