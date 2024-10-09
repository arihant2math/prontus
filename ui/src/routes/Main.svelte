<script>
    import ChannelCard from "./CurrentChannelCard.svelte";
    import Settings, {showSettings} from "./Settings.svelte";
    import MemberList from "./MemberList.svelte";

    import {
        loadChannel,
        getMessages,
        loadMessages,
        sendMessage,
        getChannelUsers,
        loadChannelUsers,
        getCurrentUser, getChannelInfo, getParentMessages, getSettings, readChannel
    } from "$lib/api.ts";
    import {positionPopovers} from "$lib/popup.js";
    import RichTextEdit from "./messageComponents/RichTextEdit.svelte";
    import MessageList from "./MessageList.svelte";
    import {listen} from "@tauri-apps/api/event";
    import Sidebar from "./Sidebar.svelte";
    import {loadTheme} from "$lib/helpers.ts";
    import NoCategorySidebar from "./NoCategorySidebar.svelte";
    import {Dialog, Label, Separator} from "bits-ui";
    import {fade} from "svelte/transition";

    let currentUser;
    let messages = [];
    let parentMessages = [];
    let channelInfo = null;
    let channelUsers = [];
    let showMemberList = false;
    let showThread = false;
    let threadParent = null;
    let messageInput;
    let settings = null;

    let createDmDialogOpen = false;

    const getThreadMessages = () => {
        if (threadParent === null) {
            showThread = false;
            return;
        }
        showMemberList = false;
        showThread = true;

        let msgs = [];
        for (let message of messages) {
            if (message.parentmessage_id === threadParent) {
                console.log(message.parentmessage_id === threadParent)
                msgs.push(message);
            } else if (message.id === threadParent) {
                msgs.push(message);
            }
        }
        return msgs;
    }

    $: threadMessages = getThreadMessages(messages, threadParent);


    // TODO: progress bar on the top or smth
    async function handleSidebarClick(id) {
        if (id === await getChannelInfo().id) {
            return;
        }
        threadParent = null;
        showMemberList = true;
        await loadChannel(id);
        let messagesPromise = loadMessages().then(async () => {
            messages = await getMessages();
            parentMessages = await getParentMessages();
            positionPopovers();
        });
        channelUsers = [];
        let usersPromise = loadChannelUsers(id).then(async () => {
            channelUsers = await getChannelUsers(id);
            positionPopovers();
        });
        let channelPromise = getChannelInfo().then((info) => {
            channelInfo = info;
            if (settings.options.read_messages) {
                readChannel(channelInfo[0].id);
            }
        });
        await Promise.all([messagesPromise, usersPromise, channelPromise]);
        // clear input
        messageInput.clear();
        // TODO: Below doesn't work for dms
        positionPopovers();
    }

    async function init() {
        getSettings().then((result) => {
            settings = result;
            loadTheme(settings);
        });
        currentUser = await getCurrentUser();
    }

    function viewThread(parentId) {
        threadParent = parentId;
        showThread = true;
    }

    async function queuedSendMessage(message, threadId) {
        sendMessage(message, threadId).then(async () => {
            messages = await getMessages();
            parentMessages = await getParentMessages();
        });
        // TODO: implement
    }

    init().then(() => {
        console.log("Main init complete");
    });

    listen('messageListUpdate', async (_event) => {
        messages = await getMessages();
        parentMessages = await getParentMessages();
    });
</script>
{#if settings !== null}
    <Settings bind:settings={settings}/>
{/if}

<div class="flex flex-row font-sans h-dvh bg-white dark:bg-slate-900 text-gray-900 dark:text-white overflow-x-hidden overflow-y-hidden">
    {#if settings !== null && settings.appearance.sidebar.category_display_level === "None"}
        <NoCategorySidebar bind:currentUser={currentUser} showSettings={showSettings}
                           bind:settings={settings}
                           showDmDialog={() => {createDmDialogOpen = true}}
                           handleSidebarClick={handleSidebarClick}/>
    {:else}
        <Sidebar bind:currentUser={currentUser} showSettings={showSettings} handleSidebarClick={handleSidebarClick}
                 showDmDialog={() => {createDmDialogOpen = true}}
                 bind:settings={settings}/>
    {/if}
    <div id="content"
         class="h-full w-full bg-white dark:bg-slate-950 flex flex-col overflow-x-hidden overflow-y-hidden">
        <div>
            <ChannelCard bind:info={channelInfo} bind:memberListActive={showMemberList}/>
        </div>
        <div class="flex flex-row overflow-x-hidden overflow-y-hidden h-full bg-white dark:bg-slate-900">
            <div class="flex flex-col w-full overflow-x-hidden overflow-y-hidden ml-4">
                <MessageList id="messagesDiv" bind:messages={messages} bind:parentMessages={parentMessages}
                             bind:currentUser={currentUser} viewThread={viewThread} bind:settings={settings}/>
                <div class="w-full mt-auto bg-white dark:bg-slate-900 z-40 p-5">
                    <RichTextEdit bind:this={messageInput}
                                  sendMessage={async (text) => {queuedSendMessage(text, null)}}/>
                </div>
            </div>
            {#if showMemberList && !showThread}
                <MemberList bind:channelUsers={channelUsers}/>
            {/if}
            {#if showThread}
                <div class="w-max h-full overflow-x-hidden overflow-y-hidden border border-gray-500">
                    <button class="fixed top-16 right-4 bg-white dark:bg-slate-800 hover:bg-gray-100 dark:hover:bg-slate-700 p-1 rounded-lg"
                            on:click={() => {showThread = false}}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                             stroke="currentColor" class="size-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12"/>
                        </svg>
                    </button>
                    <div class="flex flex-col w-full h-full overflow-x-hidden overflow-y-hidden ml-4">
                        <MessageList id="threadMessagesDiv" bind:messages={threadMessages}
                                     bind:parentMessages={parentMessages} bind:currentUser={currentUser} inThread={true}
                                     bind:settings={settings}/>
                        <div class="w-full mt-auto bg-white dark:bg-slate-900 z-40 p-5">
                            <RichTextEdit sendMessage={async (text) => {queuedSendMessage(text, threadParent)}}/>
                        </div>
                    </div>
                </div>
            {/if}
        </div>
    </div>
</div>


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
            <Dialog.Description class="text-sm">
                Create a direct message to a user.
            </Dialog.Description>
            <div class="flex flex-col items-start gap-1 pb-11 pt-7">
                <!-- TODO: Input -->
            </div>
            <div class="flex w-full justify-end">
                <Dialog.Close
                        class="inline-flex items-center justify-center px-4 py-2 text-[15px] rounded-md disabled:bg-gray-100 disabled:dark:bg-slate-700 bg-blue-600 hover:bg-blue-500 font-semibold shadow-sm outline-none" disabled>
                    Create
                </Dialog.Close>
            </div>
            <Dialog.Close
                    class="absolute right-5 top-5 rounded-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-foreground focus-visible:ring-offset-2 focus-visible:ring-offset-background active:scale-98">
                <div>
                    <!--            TODO: replace-->
                    Close
                    <span class="sr-only">Close</span>
                </div>
            </Dialog.Close>
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>
