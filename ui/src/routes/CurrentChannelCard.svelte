<script>
    import {DropdownMenu} from "bits-ui";

    export let info = null;

    export let memberListActive = false;

    $: memberListFill = memberListActive ? "currentColor" : "none";

    function toggleMemberList() {
        memberListActive = !memberListActive;
    }
</script>
{#if info !== null && info[0] !== undefined}
    <div class="h-[60px] border-b border-gray-500 flex w-full items-center text-gray-900 dark:text-white text-lg my-auto px-5 flex-row">
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
            <button class="hover:bg-gray-300 dark:hover:bg-slate-700 px-1 py-2 rounded-lg">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                     stroke="currentColor" class="size-6">
                    <path stroke-linecap="round" stroke-linejoin="round"
                          d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967 0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714 0"/>
                </svg>
            </button>
            <button class="hover:bg-gray-300 dark:hover:bg-slate-700 px-1 py-2 rounded-lg">
                <svg xmlns="http://www.w3.org/2000/svg" fill="{memberListFill}" viewBox="0 0 24 24" stroke-width="1.5"
                     stroke="currentColor" class="size-6" on:click={toggleMemberList}>
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
                        class="w-full max-w-[229px] rounded-xl bg-white dark:bg-slate-800 border border-muted px-1 py-1.5 shadow-popover"
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