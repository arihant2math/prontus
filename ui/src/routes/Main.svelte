<script>
    import {run} from 'svelte/legacy';
    import {PaneGroup, Pane, PaneResizer} from "paneforge";

    import ChannelCard from "./CurrentChannelCard.svelte";
    import Settings from "./dialog/Settings.svelte";
    import MemberList from "./MemberList.svelte";

    import {
        loadChannel,
        getMessages,
        loadMessages,
        sendMessage,
        getChannelUsers,
        loadChannelUsers,
        getCurrentUser, getChannelInfo, getParentMessages, getSettings, readChannel, createDm
    } from "$lib/api.ts";
    import {positionPopovers} from "$lib/popup.js";
    import RichTextEdit from "./messageComponents/RichTextEdit.svelte";
    import MessageList from "./MessageList.svelte";
    import {listen} from "@tauri-apps/api/event";
    import Sidebar from "./sidebar/Sidebar.svelte";
    import {loadTheme} from "$lib/helpers.ts";
    import {Dialog, Label, Separator} from "bits-ui";
    import {fade} from "svelte/transition";
    import CreateDm from "./dialog/CreateDm.svelte";
    import Announcements from "./dialog/Announcements.svelte";
    import Tasks from "./dialog/Tasks.svelte";
    import {toast, Toaster} from "svelte-sonner";
    import MessagePlaceholder from "./MessagePlaceholder.svelte";

    let currentUser = $state();
    let messages = $state([]);
    let parentMessages = $state([]);
    let channelInfo = $state(null);
    let channelUsers = $state([]);
    let showMemberList = $state(false);
    let showThread = $state(false);
    let threadParent = $state(null);
    let messageInput = $state();
    let settings = $state(null);
    let loadingMessages = $state(-1);

    let createDmDialogOpen = $state(false);
    let settingsDialogOpen = $state(false);
    let announcementsDialogOpen = $state(false);
    let tasksDialogOpen = $state(false);

    function getThreadMessages(messages, threadParent) {
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

    let threadMessages = $state();
    $effect(() => {
        threadMessages = getThreadMessages(messages, threadParent);
    });


    async function handleSidebarClick(id) {
        loadingMessages = 0;
        if (id === await getChannelInfo().id) {
            return;
        }
        loadingMessages = 10;
        threadParent = null;
        showMemberList = true;
        await loadChannel(id);
        let messagesPromise = loadMessages().then(async () => {
            loadingMessages += 10;
            messages = await getMessages();
            loadingMessages += 3;
            parentMessages = await getParentMessages();
            loadingMessages += 2;
            positionPopovers();
        });
        channelUsers = [];
        let usersPromise = loadChannelUsers(id).then(async () => {
            loadingMessages += 10;
            channelUsers = await getChannelUsers(id);
            loadingMessages += 10;
            positionPopovers();
        });
        let channelPromise = getChannelInfo().then((info) => {
            channelInfo = info;
            loadingMessages += 10;
            if (settings.options.read_messages) {
                readChannel(channelInfo[0].id);
            }
        });
        await Promise.all([messagesPromise, usersPromise, channelPromise]);
        // clear input
        messageInput.clear();
        // TODO: Below doesn't work for dms
        positionPopovers();
        console.log(channelInfo);
        loadingMessages = -1;
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

    async function createDmForUser(user) {
        createDmDialogOpen = false;
        await createDm(user.id);
        toast.success("Direct message created");
        // TODO: focus onto that dm
    }

    init().then(() => {
        console.log("Main init complete");
    });

    listen('messageListUpdate', async (_event) => {
        messages = await getMessages();
        parentMessages = await getParentMessages();
    });
</script>
<div style="height: 100vh">
    <PaneGroup direction="horizontal"
               class="w-full flex flex-row font-sans h-dvh bg-white dark:bg-slate-900 text-gray-900 dark:text-white overflow-x-hidden overflow-y-hidden">
        <Pane defaultSize={25}>
            <Sidebar bind:currentUser={currentUser}
                     bind:channelInfo={channelInfo}
                     bind:settings={settings}
                     onSidebarClick={async (id) => {await handleSidebarClick(id)}}
                     onShowDmDialog={() => {createDmDialogOpen = true}}
                     onShowSettings={() => {settingsDialogOpen = true}}
                     onShowAnnouncements={() => {announcementsDialogOpen=true}}
                     onShowTasks={() => {tasksDialogOpen = true}}/>
        </Pane>
        <PaneResizer class="relative flex w-2 items-center justify-center bg-background">
            <!--		<div class="z-10 flex h-7 w-5 items-center justify-center rounded-sm border bg-brand">-->
            <!--			<DotsSixVertical class="size-4 text-black" weight="bold" />-->
            <!--		</div>-->
        </PaneResizer>
        <Pane defaultSize={75}>
            <div id="content" style="height: 100vh"
                 class="w-full bg-white dark:bg-slate-950 flex flex-col overflow-x-hidden overflow-y-hidden">
                {#if loadingMessages !== -1}
                    <div class="w-full h-1">
                        <div class="bg-blue-600 h-1" style="width: {loadingMessages}%"></div>
                    </div>
                {/if}
                <div>
                    <ChannelCard bind:info={channelInfo} bind:memberListActive={showMemberList}/>
                </div>
                <div class="flex flex-row overflow-x-hidden overflow-y-hidden h-full bg-white dark:bg-slate-900">
                    <div class="flex flex-col w-full overflow-x-hidden overflow-y-hidden ml-4">
                        <MessageList id="messagesDiv" bind:messages={messages} bind:parentMessages={parentMessages}
                                     channelInfo={channelInfo} currentUser={currentUser} viewThread={viewThread}
                                     settings={settings} onCreateDm={createDmForUser} pulsing={loadingMessages !== -1}/>
                        <div class="w-full mt-auto bg-white dark:bg-slate-900 z-40 p-5">
                            {#if channelInfo !== null && channelInfo[0].grant_create_message && loadingMessages === -1}
                                <RichTextEdit bind:this={messageInput}
                                              sendMessage={async (text) => {queuedSendMessage(text, null)}}
                                              disabled={false}/>
                            {:else}
                                <RichTextEdit bind:this={messageInput}
                                              sendMessage={async (text) => {queuedSendMessage(text, null)}}
                                              disabled={true}/>
                            {/if}
                        </div>
                    </div>
                    {#if showMemberList && !showThread}
                        <MemberList bind:channelUsers={channelUsers} on:createDm={createDmForUser}/>
                    {/if}
                    {#if showThread}
                        <div class="w-max h-full overflow-x-hidden overflow-y-hidden border border-gray-500">
                            <button class="fixed top-16 right-4 bg-white dark:bg-slate-800 hover:bg-gray-100 dark:hover:bg-slate-700 p-1 rounded-lg"
                                    onclick={() => {showThread = false}} aria-label="Close Thread">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                                     stroke-width="1.5"
                                     stroke="currentColor" class="size-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12"/>
                                </svg>
                            </button>
                            <div class="flex flex-col w-full h-full overflow-x-hidden overflow-y-hidden ml-4">
                                <MessageList id="threadMessagesDiv" bind:messages={threadMessages}
                                             channelInfo={channelInfo} viewThread={(id) => {}}
                                             bind:parentMessages={parentMessages} currentUser={currentUser}
                                             inThread={true}
                                             settings={settings} on:createDm={createDmForUser}/>
                                <div class="w-full mt-auto bg-white dark:bg-slate-900 z-40 p-5">
                                    <RichTextEdit
                                            sendMessage={async (text) => {queuedSendMessage(text, threadParent)}}/>
                                </div>
                            </div>
                        </div>
                    {/if}
                </div>
            </div>
        </Pane>
    </PaneGroup>

    <Settings bind:settings={settings} bind:showSettings={settingsDialogOpen}/>
    <CreateDm bind:createDmDialogOpen={createDmDialogOpen}/>
    <Announcements bind:announcementsDialogOpen={announcementsDialogOpen}/>
    <Tasks bind:tasksDialogOpen={tasksDialogOpen}/>
    <Toaster/>
</div>
