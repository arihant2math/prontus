<!--TODO: Add ability to log out-->
<script>
    import {
        getSettings,
        modifyChannelPermission,
        setChannelAlias,
        setChannelMute,
        setChannelNotifications, setChannelTitle,
        setSettings
    } from "$lib/api.ts";
    import {fade} from "svelte/transition";
    import {Dialog, Separator, Tabs} from "bits-ui";
    import TabsTrigger from "../bitsHead/TabsTrigger.svelte";
    import DialogContent from "../bitsHead/DialogContent.svelte";
    import DialogClose from "../bitsHead/DialogClose.svelte";
    import SeparatorRoot from "../bitsHead/SeparatorRoot.svelte";
    import ActionButton from "../ActionButton.svelte";
    import Select from "../bitsHead/Select.svelte";
    import { Label } from "bits-ui";

    let { info = $bindable(), stats = $bindable(), membership = $bindable(), showSettings = $bindable(false) } = $props();

    const role = $derived(membership.role);
    let alias = $state();
    let title = $state();
    let permissionOptions = [
        {value: "member", label: "Member"},
        {value: "owner", label: "Owner"}
    ];

    function hasPermission(permission) {
        return info[permission] !== null && (info[permission] === "member" || role === "owner");
    }

    function titleChange(ev) {
        if (ev.key !== "Enter") return;
        setChannelTitle(info.id, title);
    }

    function aliasChange() {
        if (alias === "") {
            setChannelAlias(info.id, null);
        } else {
            setChannelAlias(info.id, alias);
        }
    }

    function getSelected(value) {
        return permissionOptions[value === "owner" | 0];
    }

    async function updateMute() {
        await setChannelMute(info.id, membership.mute);
    }

    async function updateNotificationPreference() {
        await setChannelNotifications(info.id, membership.notificationpreference);
    }

    $effect(() => {
        alias = info.alias;
        title = info.title;
    })
</script>
<Dialog.Root bind:open={showSettings}>
    <Dialog.Trigger/>
    <Dialog.Portal>
        <Dialog.Overlay
                transition={fade}
                transitionConfig={{ duration: 150 }}
                class="fixed inset-0 z-50 bg-black/80"
        />
        <DialogContent>
            <Dialog.Title
                    class="flex w-full h-max items-center justify-items-start text-2xl px-2 font-semibold">
                Channel Settings
            </Dialog.Title>
            <SeparatorRoot/>
            {#if info !== null && info !== undefined}
                <div class="p-4 md:p-5 overflow-y-auto" style="height: 75vh">
                    <div>
                        <Tabs.Root
                                value="general"
                                class="w-full rounded-card"
                        >
                            <Tabs.List
                                    class="grid w-full grid-cols-5 gap-1 rounded-9px bg-dark-10 p-1 text-sm font-semibold leading-[0.01em] shadow-mini-inset dark:border dark:border-neutral-600/30 dark:bg-background"
                            >
                                <TabsTrigger value="general">General</TabsTrigger>
                                <TabsTrigger value="members">Members</TabsTrigger>
                                <TabsTrigger value="invites">Invites</TabsTrigger>
                                <TabsTrigger value="notifications">Notifications</TabsTrigger>
                                <TabsTrigger value="media">Media</TabsTrigger>
                            </Tabs.List>
                            <Tabs.Content value="general" class="pt-3">
                                <div class="max-w-xxl">
                                    {#if !info.isdm}
                                        <div class="relative z-0 w-full mb-5 group">
                                            <input name="floating_name" id="floating_name" class="block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer" placeholder=" " bind:value={title} disabled={!hasPermission("changetitle")} onkeydown={titleChange}/>
                                            <label for="floating_name" class="peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">Channel Name</label>
                                        </div>
                                        <div class="relative z-0 w-full mb-5 group">
                                            <input name="floating_alias" id="floating_alias" class="block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer" placeholder=" " bind:value={alias} onchange={aliasChange}/>
                                            <label for="floating_alias" class="peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">Alias</label>
                                        </div>

                                            <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 my-8">
                                                <div>
                                                    <Label.Root>
                                                        Change group name
                                                    </Label.Root>
                                                    <Select options={permissionOptions} selected={getSelected(info.changetitle)} onSelectedChange={(value) => {modifyChannelPermission(info.id, "changetitle", value.value)}} disabled={role === "member"}/>
                                                </div>
                                                <div>
                                                    <Label.Root>
                                                        Change group category
                                                    </Label.Root>
                                                    <Select options={permissionOptions} selected={getSelected(info.changecategory)} onSelectedChange={(value) => {modifyChannelPermission(info.id, "changecategory", value.value)}} disabled={role === "member"}/>
                                                </div>
                                                <div>
                                                    <Label.Root>
                                                        Add New Members
                                                    </Label.Root>
                                                    <Select options={permissionOptions} selected={getSelected(info.addmember)} onSelectedChange={(value) => {modifyChannelPermission(info.id, "addmember", value.value)}} disabled={role === "member"}/>
                                                </div>
                                                <div>
                                                    <Label.Root>
                                                        Remove Members
                                                    </Label.Root>
                                                    <Select options={permissionOptions} selected={getSelected(info.removemember)} onSelectedChange={(value) => {modifyChannelPermission(info.id, "removemember", value.value)}} disabled={role === "member"}/>
                                                </div>
                                                <div>
                                                    <Label.Root>
                                                        Leave Group
                                                    </Label.Root>
                                                    <Select options={permissionOptions} selected={getSelected(info.leavegroup)} onSelectedChange={(value) => {modifyChannelPermission(info.id, "leavegroup", value.value)}} disabled={role === "member"}/>
                                                </div>
                                                <div>
                                                    <Label.Root>
                                                        Start Meetings
                                                    </Label.Root>
                                                    <Select options={permissionOptions} selected={getSelected(info.create_videosession)} onSelectedChange={(value) => {modifyChannelPermission(info.id, "create_videosession", value.value)}} disabled={role === "member"}/>
                                                </div>
                                                <div>
                                                    <Label.Root>
                                                        Send Messages
                                                    </Label.Root>
                                                    <Select options={permissionOptions} selected={getSelected(info.create_message)} onSelectedChange={(value) => {modifyChannelPermission(info.id, "create_message", value.value)}} disabled={role === "member"}/>
                                                </div>
                                               <div>
                                                    <Label.Root>
                                                        <!-- TODO: do cloud recording later -->
                                                        Record Meetings Locally
                                                    </Label.Root>
                                                    <Select options={permissionOptions} selected={getSelected(info.grantvideosessionrecordlocal)} onSelectedChange={(value) => {modifyChannelPermission(info.id, "grantvideosessionrecordlocal", value.value)}} disabled={role === "member"}/>
                                                </div>
                                                <div>
                                                    <Label.Root>
                                                        Assign Tasks
                                                    </Label.Root>
                                                    <Select options={permissionOptions} selected={getSelected(info.assign_task)} onSelectedChange={(value) => {modifyChannelPermission(info.id, "assign_task", value.value)}} disabled={role === "member"}/>
                                                </div>
                                                <div>
                                                    <Label.Root>
                                                        Send Announcements
                                                    </Label.Root>
                                                    <Select options={permissionOptions} selected={getSelected(info.create_announcement)} onSelectedChange={(value) => {modifyChannelPermission(info.id, "create_announcement", value.value)}} disabled={role === "member"}/>
                                                </div>
                                                <div>
                                                    <Label.Root>
                                                        Pin a message
                                                    </Label.Root>
                                                    <Select options={permissionOptions} selected={getSelected(info.pin_message)} onSelectedChange={(value) => {modifyChannelPermission(info.id, "pin_message", value.value)}} disabled={role === "member"}/>
                                                </div>
                                            </div>
                                    {/if}
                                </div>
                                <ActionButton>Hide Group</ActionButton>
                                {#if hasPermission("leavegroup")}
                                    <ActionButton style="warning">Leave Group</ActionButton>
                                {/if}
                                {#if hasPermission("deletegroup")}
                                    <ActionButton style="danger">Delete Group</ActionButton>
                                {/if}
                            </Tabs.Content>
                            <Tabs.Content value="members" class="pt-3">
<!--                                TODO:-->
                            </Tabs.Content>
                            <Tabs.Content value="invites" class="pt-3">
<!--                                TODO: -->
                            </Tabs.Content>
                            <Tabs.Content value="notifications" class="pt-3">
                                <label class="inline-flex items-center cursor-pointer p-3">
                                    <input type="checkbox" value={membership.mute} class="sr-only peer" onclick={updateMute}>
                                    <div class="relative w-11 h-6 bg-gray-200 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:w-5 after:h-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
                                    <span class="ms-3 text-sm font-medium text-gray-900 dark:text-gray-300">Mute Channel</span>
                                </label>
                                <ul class="p-3 space-y-1 text-sm text-gray-700 dark:text-gray-200">
                                    <li>
                                        <div class="flex p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                                            <div class="flex items-center h-5">
                                                <input id="helper-radio-4" name="helper-radio" type="radio" value="ALL" bind:group={membership.notificationpreference} onclick={updateNotificationPreference} class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500">
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
                                                <input id="helper-radio-5" name="helper-radio" type="radio" value="MENTIONS" bind:group={membership.notificationpreference} onclick={updateNotificationPreference} class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500">
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
                                                <input id="helper-radio-6" name="helper-radio" type="radio" value="NONE" bind:group={membership.notificationpreference} onclick={updateNotificationPreference} class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500">
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
                            </Tabs.Content>
                            <Tabs.Content value="media" class="pt-3">
<!--                                TODO: -->
                            </Tabs.Content>
                        </Tabs.Root>
                    </div>
                </div>
            {/if}
            <DialogClose/>
        </DialogContent>
    </Dialog.Portal>
</Dialog.Root>