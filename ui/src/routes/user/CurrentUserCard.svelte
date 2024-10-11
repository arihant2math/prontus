<script>
    import ProfilePicture from "./ProfilePicture.svelte";
    import { createEventDispatcher } from 'svelte';
    import {Dialog, DropdownMenu, Separator} from "bits-ui";
    import { fade } from "svelte/transition";

    export let user = {
        id: 0,
        fullname: "User",
        profilepicurl: "https://cdn.discordapp.com/embed/avatars/0.png",
        organizations: [
            {
                name: "Organization"
            }
        ]
    };

    export let showSettings;
    export let showDmDialog;

    const dispatch = createEventDispatcher();
</script>

<div class="flex flex-row px-4 w-full border-b border-gray-500 z-40 h-[60px]">
    <DropdownMenu.Root>
        <DropdownMenu.Trigger class="flex-auto flex flex-row space-x-4 p-2 rounded-lg hover:bg-gray-300 dark:hover:bg-slate-700 align-middle">
            <div class="align-middle self-center flex-none">
                <ProfilePicture user={user}/>
            </div>
            <div class="flex flex-col rounded-lg px-2 align-left text-left self-center">
                <p class="text-sm">{user.fullname}</p>
                <div>
                    <p class="text-xs text-gray-500 dark:text-gray-400 whitespace-nowrap text-ellipsis overflow-x-hidden">
                        {user.organizations[0].name}
                    </p>
                </div>
            </div>
        </DropdownMenu.Trigger>
        <DropdownMenu.Content
                class="w-full max-w-[229px] rounded-lg bg-gray-100 dark:bg-slate-900 px-1 py-1.5 shadow-md"
                sideOffset={8}
        >
            <DropdownMenu.Item
                    class="flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium !ring-0 !ring-transparent data-[highlighted]:bg-muted"
            >
                <div class="flex items-center">
                    TBD
                </div>
            </DropdownMenu.Item>
        </DropdownMenu.Content>
    </DropdownMenu.Root>
    <button class="px-2 rounded-lg hover:bg-gray-300 dark:hover:bg-slate-700" on:click={() => dispatch('showAnnouncements')}>
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M10.34 15.84c-.688-.06-1.386-.09-2.09-.09H7.5a4.5 4.5 0 1 1 0-9h.75c.704 0 1.402-.03 2.09-.09m0 9.18c.253.962.584 1.892.985 2.783.247.55.06 1.21-.463 1.511l-.657.38c-.551.318-1.26.117-1.527-.461a20.845 20.845 0 0 1-1.44-4.282m3.102.069a18.03 18.03 0 0 1-.59-4.59c0-1.586.205-3.124.59-4.59m0 9.18a23.848 23.848 0 0 1 8.835 2.535M10.34 6.66a23.847 23.847 0 0 0 8.835-2.535m0 0A23.74 23.74 0 0 0 18.795 3m.38 1.125a23.91 23.91 0 0 1 1.014 5.395m-1.014 8.855c-.118.38-.245.754-.38 1.125m.38-1.125a23.91 23.91 0 0 0 1.014-5.395m0-3.46c.495.413.811 1.035.811 1.73 0 .695-.316 1.317-.811 1.73m0-3.46a24.347 24.347 0 0 1 0 3.46" />
        </svg>
    </button>
    <button class="px-2 rounded-lg hover:bg-gray-300 dark:hover:bg-slate-700" on:click={() => dispatch('showTasks')}>
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0ZM3.75 12h.007v.008H3.75V12Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm-.375 5.25h.007v.008H3.75v-.008Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z" />
        </svg>
    </button>
    <button class="px-2 rounded-lg hover:bg-gray-300 dark:hover:bg-slate-700" on:click={showSettings}>
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z" />
            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
        </svg>
    </button>
    <DropdownMenu.Root>
        <DropdownMenu.Trigger class="px-2 rounded-lg hover:bg-gray-300 dark:hover:bg-slate-700">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
            </svg>
        </DropdownMenu.Trigger>
        <DropdownMenu.Content
                class="w-full max-w-[229px] rounded-lg bg-gray-100 dark:bg-slate-900 px-1 py-1.5 shadow-md"
                sideOffset={8}>
            <DropdownMenu.Item
                    class="flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                <button class="flex items-center">
                    New Group
                </button>
            </DropdownMenu.Item>
            <DropdownMenu.Item
                    class="flex h-10 select-none items-center rounded-button py-3 pl-3 pr-1.5 text-sm font-medium !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                <button class="flex items-center" on:click={showDmDialog}>
                    New DM
                </button>
            </DropdownMenu.Item>
        </DropdownMenu.Content>
    </DropdownMenu.Root>
</div>