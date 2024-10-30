<!--TODO: Add ability to log out-->
<script>
    import { run } from 'svelte/legacy';

    import { open } from '@tauri-apps/plugin-dialog';
    import RadioLabel from "../settingsComponents/RadioLabel.svelte";
    import OptionsLabel from "../settingsComponents/options/OptionsLabel.svelte";
    import {getSettings, setChannelAlias, setSettings} from "$lib/api.ts";
    import {loadTheme} from "$lib/helpers.ts";
    import {fade} from "svelte/transition";
    import {Dialog, Separator, Tabs} from "bits-ui";
    import CloseButton from "../CloseButton.svelte";
    import TabsTrigger from "../bitsHead/TabsTrigger.svelte";
    import DialogContent from "../bitsHead/DialogContent.svelte";
    import DialogClose from "../bitsHead/DialogClose.svelte";
    import SeparatorRoot from "../bitsHead/SeparatorRoot.svelte";
    import ActionButton from "../ActionButton.svelte";

    let { info = $bindable(), stats = $bindable(), membership = $bindable(), showSettings = $bindable(false) } = $props();

    const role = $derived(membership.role);
    let alias = $state();

    function hasPermission(permission) {
        return info[permission] !== null && (info[permission] === "member" || role === "owner");
    }

    function aliasChange() {
        if (alias === "") {
            setChannelAlias(info.id, null);
        } else {
            setChannelAlias(info.id, alias);
        }
    }

    $effect(() => {
        console.log(info)
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
                                    class="grid w-full grid-cols-4 gap-1 rounded-9px bg-dark-10 p-1 text-sm font-semibold leading-[0.01em] shadow-mini-inset dark:border dark:border-neutral-600/30 dark:bg-background"
                            >
                                <TabsTrigger value="general">General</TabsTrigger>
<!--                                TODO: members-->
<!--                                TODO: notifications -->
                            </Tabs.List>
                            <Tabs.Content value="general" class="pt-3">
                                <div class="max-w-lg">
                                    {#if !info.isdm}
                                        <div class="relative z-0 w-full mb-5 group">
                                            <input name="floating_name" id="floating_name" class="block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer" placeholder=" " bind:value={info.title} disabled={!hasPermission("changetitle")}/>
                                            <label for="floating_name" class="peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">Channel Name</label>
                                        </div>
                                        <div class="relative z-0 w-full mb-5 group">
                                            <input name="floating_alias" id="floating_alias" class="block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer" placeholder=" " bind:value={alias} onchange={aliasChange}/>
                                            <label for="floating_alias" class="peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">Alias</label>
                                        </div>
                                    {/if}
                                </div>
                                {#if hasPermission("deletegroup")}
                                    <ActionButton style="danger">Delete Group</ActionButton>
                                {/if}
                                {#if hasPermission("leavegroup")}
                                    <ActionButton style="warning">Leave Group</ActionButton>
                                {/if}
                            </Tabs.Content>
                        </Tabs.Root>
                    </div>
                </div>
            {/if}
            <DialogClose/>
        </DialogContent>
    </Dialog.Portal>
</Dialog.Root>