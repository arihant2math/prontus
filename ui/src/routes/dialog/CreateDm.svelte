<script>
    import { run } from 'svelte/legacy';

    import {fade} from "svelte/transition";
    import {Dialog, Separator} from "bits-ui";
    import CloseButton from "../CloseButton.svelte";
    import MemberList from "../MemberList.svelte";
    import {userSearch} from "$lib/api.ts";
    import DialogContent from "../bitsHead/DialogContent.svelte";
    import DialogClose from "../bitsHead/DialogClose.svelte";
    import SeparatorRoot from "../bitsHead/SeparatorRoot.svelte";

    /** @type {{createDmDialogOpen?: boolean}} */
    let { createDmDialogOpen = $bindable(false) } = $props();
    let username = $state("");
    let users = $state([]);

    function updateUsers(username) {
        if (username.length > 0) {
            console.log(username);
            userSearch(username).then((res) => {
                console.log(res);
                users = res;
            });
        }
    }
    run(() => {
        username, updateUsers(username);
    });
</script>

<Dialog.Root bind:open={createDmDialogOpen}>
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
                Create DM
            </Dialog.Title>
            <SeparatorRoot/>
            <div class="flex flex-col items-start gap-1 pb-11 pt-7">
                <input name="username" placeholder="Username" id="username" bind:value={username} class="bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-blue-600 focus:border-blue-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
            </div>
           <div class="overflow-y-auto w-full overflow-x-hidden" style="height: 75vh">
                <MemberList bind:channelUsers={users}/>
           </div>
            <DialogClose/>
        </DialogContent>
    </Dialog.Portal>
</Dialog.Root>