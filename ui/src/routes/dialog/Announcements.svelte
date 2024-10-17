<script>
    import {fade} from "svelte/transition";
    import {Dialog, Separator} from "bits-ui";
    import {getAnnouncements} from "$lib/api.ts";
    import AnnouncementListItem from "../announcementComponents/AnnouncementListItem.svelte";
    import CloseButton from "../CloseButton.svelte";
    import {listen} from "@tauri-apps/api/event";

    export let announcementsDialogOpen = false;
    let announcements = [];

    function fetchAnnouncements() {
        getAnnouncements().then((response) => {
            announcements = response.data;
        });
    }

    fetchAnnouncements();
    listen('announcementListUpdate', async (_event) => {
        fetchAnnouncements();
    });
</script>

<Dialog.Root bind:open={announcementsDialogOpen}>
    <Dialog.Trigger/>
    <Dialog.Portal>
        <Dialog.Overlay
                transition={fade}
                transitionConfig={{ duration: 150 }}
                class="fixed inset-0 z-50 bg-black/80"
        />
        <Dialog.Content
                class="fixed left-[50%] top-[50%] z-50 w-full max-w-[90%] translate-x-[-50%] translate-y-[-50%] rounded-lg bg-white dark:bg-slate-800 p-5 shadow-2xl outline-none">
            <Dialog.Title
                    class="flex w-full items-center justify-center text-lg font-semibold">
                Announcements
            </Dialog.Title>
            <Separator.Root class="-mx-5 mb-6 mt-5 block h-px bg-gray-500"/>
            <div class="overflow-y-auto w-full overflow-x-hidden" style="height: 75vh">
                {#if announcements.length === 0}
                    <Dialog.Description class="text-sm">
                        No announcements.
                    </Dialog.Description>
                {:else}
                    <div class="flex flex-col items-start gap-1 pb-11 pt-7">
                        {#each announcements as announcement}
                            <AnnouncementListItem announcement={announcement}/>
                        {/each}
                    </div>
                {/if}
            </div>
            <Dialog.Close
                    class="absolute right-5 top-5 rounded-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-foreground focus-visible:ring-offset-2 focus-visible:ring-offset-background active:scale-98">
                <CloseButton/>
            </Dialog.Close>
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>
