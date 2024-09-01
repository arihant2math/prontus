<script>
    import "../app.css";
    import {invoke} from "@tauri-apps/api/core";
    import Message from "./message.svelte";
    import Sideitem from "./sideitem.svelte";

    async function loadUserInfo() {
        return await invoke("load_user_info");
    }

    async function loadChannel(id) {
        await invoke("load_channel", {id});
    }

    async function getChannelList() {
        return await invoke("get_channel_list");
    }

    async function loadChannelList() {
        await invoke("load_channel_list");
    }

    async function getMessages() {
        return await invoke("get_messages");
    }

    async function getMoreMessages(lastMessageId) {
        return await invoke("get_more_messages", {lastMessageId});
    }

    async function loadMessages() {
        return await invoke("load_messages");
    }

    async function sendMessage(message) {
        return await invoke("send_message", {message});
    }

    function spawnMessage(next, message) {
        if (next != null) {
            let messageElement = new Message({
                target: document.querySelector("#messages"),
                props: {
                    message_id: message.id,
                    user: message.user.fullname,
                    message: message.message,
                    timestamp: message.timestamp,
                    pfp_url: message.user.profilepicurl,
                    embed: message.resource,
                    reactions: message.reactionsummary,
                    repeat: next.user.fullname == message.user.fullname && next.systemevent != null,
                    systemMessage: message.systemevent != null,
                }
            });
        } else {
            let messageElement = new Message({
                target: document.querySelector("#messages"),
                props: {
                    message_id: message.id,
                    user: message.user.fullname,
                    message: message.message,
                    timestamp: message.timestamp,
                    pfp_url: message.user.profilepicurl,
                    embed: message.resource,
                    reactions: message.reactionsummary,
                    repeat: false,
                    systemMessage: message.systemevent != null,
                }
            });
        }
    }

    function appendMessages(messages) {
        let previousMessage = null;
        for (let message of messages) {
            console.debug(message);
            if (previousMessage != null) {
                spawnMessage(message, previousMessage);
            }
            previousMessage = message;
        }
        spawnMessage(null, previousMessage);
    }

    async function handleSidebarClick(id) {
        await loadChannel(id);
        await loadMessages();
        let messages = await getMessages();
        document.querySelector("#messages").innerHTML = "";
        // clear input
        document.querySelector("#messageInput").value = "";
        appendMessages(messages);
        let messagesDiv = document.querySelector("#messages");
        messagesDiv.scrollTop = 0;
    }

    async function handleMessageKeyDown(event) {
        if (event.keyCode === 13) {
            await sendMessage(document.querySelector("#messageInput").value).then((message) => {
                document.querySelector("#messageInput").value = "";
                appendMessages([message]);
            });
        }
    }

    let updating = false;

    async function messageScroll(event) {
        if (event.target.scrollTop + event.target.scrollHeight < 1000) {
            if (updating) {
                return;
            }
            updating = true;
            console.info("Loading more messages");
            let messages = await getMessages();
            let last = messages[messages.length - 1].id;
            await getMoreMessages(last).then(async () => {
                let messages = await getMessages();
                appendMessages(messages);
            });
            updating = false;
        }
    }

    loadChannelList().then(async () => {
        await loadUserInfo();
        let channels = await getChannelList();
        for (let channel of channels) {
            let sideitem = new Sideitem({
                target: document.querySelector("#sidebar-list"),
                props: {
                    bubbleId: channel.id,
                    name: channel.title,
                    notifications: channel.notifications,
                    mention: channel.mention,
                    buttonClick: handleSidebarClick,
                },
            });
        }
    });
</script>

<div class="flex flex-row font-sans h-dvh bg-white dark:bg-gray-900">
    <aside id="default-sidebar"
           aria-label="Sidebar">
        <div class="h-full max-w-[350px] overflow-y-auto px-3 py-4 bg-gray-50 dark:bg-gray-800">
            <ul class="space-y-2 font-medium" id="sidebar-list">
            </ul>
        </div>
    </aside>
    <div id="content" class="h-full w-full bg-white dark:bg-gray-900 flex flex-col">
        <div id="messages" class="overflow-y-auto bg-white dark:bg-gray-900 flex flex-col-reverse" on:scroll={messageScroll}>
        </div>
        <div class="w-full border-t border-gray-500 mt-auto">
            <input id="messageInput" type="text" class="outline-0 w-full border-0 h-[50px] text-base border-none px-4" on:keydown={handleMessageKeyDown}>
        </div>
    </div>
</div>

<style>
    @media (prefers-color-scheme: dark) {
        /* This part changes the scrollbar track (the part behind the scrollbar) */
        ::-webkit-scrollbar-track {
            background: #1E1E1E; /* Dark grey color, you can choose any color you like */
        }

        /* This part changes the scrollbar handle */
        ::-webkit-scrollbar-thumb {
            background: #555; /* Medium grey color, this is the actual scrollbar */
        }

        /* You can also change the scrollbar width and height */
        ::-webkit-scrollbar {
            width: 12px; /* Width of the vertical scrollbar */
            height: 12px; /* Height of the horizontal scrollbar */
        }
    }
</style>