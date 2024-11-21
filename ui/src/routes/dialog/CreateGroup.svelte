<script>
    import {fade} from "svelte/transition";
    import {Dialog, Separator} from "bits-ui";
    import CloseButton from "../CloseButton.svelte";
    import UserList from "../UserList.svelte";
    import {userSearch} from "$lib/api.ts";
    import DialogContent from "../bitsHead/DialogContent.svelte";
    import DialogClose from "../bitsHead/DialogClose.svelte";
    import SeparatorRoot from "../bitsHead/SeparatorRoot.svelte";
    import ActionButton from "../ActionButton.svelte";

    /** @type {{createDmDialogOpen?: boolean}} */
    let {createGroupDialogOpen = $bindable(false), onCreateGroup} = $props();
    let channelName = $state("");
</script>

<Dialog.Root bind:open={createGroupDialogOpen}>
    <Dialog.Trigger/>
    <Dialog.Portal>
        <Dialog.Overlay
                transition={fade}
                transitionConfig={{ duration: 150 }}
                class="fixed inset-0 z-50 bg-black/80"
        />
        <DialogContent>
            <Dialog.Title
                    class="flex w-full items-center justify-center text-lg font-semibold">
                Create Group
            </Dialog.Title>
            <SeparatorRoot/>
            <div class="flex flex-col items-start gap-1 pb-11 pt-7 max-w-lg">
                <input name="channelName" placeholder="Channel Name" id="channelName" bind:value={channelName}
                       class="bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-blue-600 focus:border-blue-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
            </div>
            <ActionButton disabled>Create Group</ActionButton>
            <DialogClose/>
        </DialogContent>
    </Dialog.Portal>
</Dialog.Root>