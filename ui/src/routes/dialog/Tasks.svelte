<script>
    import {fade} from "svelte/transition";
    import {Dialog, Separator} from "bits-ui";
    import {getTasks} from "$lib/api.ts";
    import CloseButton from "../CloseButton.svelte";
    import TaskListItem from "../taskComponents/TaskListItem.svelte";
    import { Tabs } from "bits-ui";

    export let tasksDialogOpen = false;
    let completed = false;
</script>

<Dialog.Root bind:open={tasksDialogOpen}>
    <Dialog.Trigger/>
    <Dialog.Portal>
        <Dialog.Overlay
                transition={fade}
                transitionConfig={{ duration: 150 }}
                class="fixed inset-0 z-50 bg-black/80"
        />
        <Dialog.Content
                class="fixed left-[50%] top-[50%] z-50 w-full max-w-[90%] translate-x-[-50%] translate-y-[-50%] rounded-lg bg-white dark:bg-slate-800 p-5 shadow-2xl outline-none"
        >
            <Dialog.Title
                    class="flex w-full items-center justify-center text-lg font-semibold"
            >
                Tasks
            </Dialog.Title>
            <Separator.Root class="-mx-5 mb-6 mt-5 block h-px bg-gray-500"/>
            <div
                    class="overflow-y-auto w-full overflow-x-hidden"
                    style="height: 75vh"
            >
                <div class="pt-6">
                    <Tabs.Root
                            value="outbound"
                            class="w-[390px] rounded-card border border-muted bg-background-alt p-3 shadow-card"
                    >
                        <Tabs.List
                                class="grid w-full grid-cols-2 gap-1 rounded-9px bg-dark-10 p-1 text-sm font-semibold leading-[0.01em] shadow-mini-inset dark:border dark:border-neutral-600/30 dark:bg-background"
                        >
                            <Tabs.Trigger
                                    value="todo"
                                    class="h-8 rounded-[7px] bg-transparent py-2 data-[state=active]:bg-white data-[state=active]:shadow-mini dark:data-[state=active]:bg-muted"
                            >Todo
                            </Tabs.Trigger
                            >
                            <Tabs.Trigger
                                    value="completed"
                                    class="h-8 rounded-[7px] bg-transparent py-2 data-[state=active]:bg-white data-[state=active]:shadow-mini dark:data-[state=active]:bg-muted"
                            >Completed
                            </Tabs.Trigger>
                        </Tabs.List>
                        <Tabs.Content value="todo" class="pt-3">
                            {#await getTasks(false)}
                                <div role="status">
                                    <svg
                                            aria-hidden="true"
                                            class="w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600"
                                            viewBox="0 0 100 101"
                                            fill="none"
                                            xmlns="http://www.w3.org/2000/svg"
                                    >
                                        <path
                                                d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                                                fill="currentColor"
                                        />
                                        <path
                                                d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                                                fill="currentFill"
                                        />
                                    </svg>
                                    <span class="sr-only">Loading...</span>
                                </div>
                            {:then tasks}
                                {#if tasks.length === 0}
                                    <Dialog.Description class="text-sm">
                                        No tasks.
                                    </Dialog.Description>
                                {:else}
                                    <div class="flex flex-col items-start gap-1 pb-11 pt-7">
                                        {#each tasks as task}
                                            <TaskListItem {task}/>
                                        {/each}
                                    </div>
                                {/if}
                            {/await}
                        </Tabs.Content>
                        <Tabs.Content value="completed" class="pt-3">
                            {#await getTasks(true)}
                                <div role="status">
                                    <svg
                                            aria-hidden="true"
                                            class="w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600"
                                            viewBox="0 0 100 101"
                                            fill="none"
                                            xmlns="http://www.w3.org/2000/svg"
                                    >
                                        <path
                                                d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                                                fill="currentColor"
                                        />
                                        <path
                                                d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                                                fill="currentFill"
                                        />
                                    </svg>
                                    <span class="sr-only">Loading...</span>
                                </div>
                            {:then tasks}
                                {#if tasks.length === 0}
                                    <Dialog.Description class="text-sm">
                                        No tasks.
                                    </Dialog.Description>
                                {:else}
                                    <div class="flex flex-col items-start gap-1 pb-11 pt-7">
                                        {#each tasks as task}
                                            <TaskListItem {task}/>
                                        {/each}
                                    </div>
                                {/if}
                            {/await}
                        </Tabs.Content>
                    </Tabs.Root>
                </div>
            </div>
            <Dialog.Close
                    class="absolute right-5 top-5 rounded-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 focus-visible:ring-offset-background active:scale-98"
            >
                <CloseButton/>
            </Dialog.Close>
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>
