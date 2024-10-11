<script>
    import {fade} from "svelte/transition";
    import {Dialog, Separator} from "bits-ui";
    import CloseButton from "../CloseButton.svelte";

    export let createDmDialogOpen = false;
</script>

<Dialog.Root bind:open={createDmDialogOpen}>
    <Dialog.Trigger/>
    <Dialog.Portal>
        <Dialog.Overlay
                transition={fade}
                transitionConfig={{ duration: 150 }}
                class="fixed inset-0 z-50 bg-black/80"
        />
        <Dialog.Content
                class="fixed left-[50%] top-[50%] z-50 w-full max-w-[94%] translate-x-[-50%] translate-y-[-50%] rounded-lg bg-white dark:bg-slate-800 p-5 shadow-2xl outline-none sm:max-w-[490px] md:w-full">
            <Dialog.Title
                    class="flex w-full items-center justify-center text-lg font-semibold">
                Create DM
            </Dialog.Title>
            <Separator.Root class="-mx-5 mb-6 mt-5 block h-px bg-gray-500"/>
            <div class="flex flex-col items-start gap-1 pb-11 pt-7">
                
            </div>
            <div class="flex w-full justify-end">
                <Dialog.Close
                        class="inline-flex items-center justify-center px-4 py-2 text-[15px] rounded-md disabled:bg-gray-100 disabled:dark:bg-slate-700 bg-blue-600 hover:bg-blue-500 font-semibold shadow-sm outline-none" disabled>
                    Create
                </Dialog.Close>
            </div>
            <Dialog.Close
                    class="absolute right-5 top-5 rounded-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-foreground focus-visible:ring-offset-2 focus-visible:ring-offset-background active:scale-98">
                <CloseButton/>
            </Dialog.Close>
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>