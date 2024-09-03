<script>
    import {invoke} from "@tauri-apps/api/core";
    import Message from "./message.svelte";
    import Sidecategory from "./sidecategory.svelte"

    import {load, loadChannel, getChannelList, getMessages, getMoreMessages, loadMessages, sendMessage} from "./api.js";

    let userInfo;

    function spawnMessage(next, message) {
        console.debug(message)
        if (next != null) {
            let messageElement = new Message({
                target: document.querySelector("#messages"),
                props: {
                    message_id: message.id,
                    user: message.user,
                    message: message.message,
                    timestamp: message.timestamp,
                    pfp_url: message.user.profilepicurl,
                    embed: message.resource,
                    media: message.messagemedia,
                    reactions: message.reactionsummary,
                    repeat: next.user.fullname == message.user.fullname && next.systemevent == null,
                    systemMessage: message.systemevent != null,
                    currentUser: userInfo,
                }
            });
        } else {
            let messageElement = new Message({
                target: document.querySelector("#messages"),
                props: {
                    message_id: message.id,
                    user: message.user,
                    message: message.message,
                    timestamp: message.timestamp,
                    pfp_url: message.user.profilepicurl,
                    embed: message.resource,
                    media: message.messagemedia,
                    reactions: message.reactionsummary,
                    repeat: false,
                    systemMessage: message.systemevent != null,
                    currentUser: userInfo,
                }
            });
        }
    }

    async function appendMessages(messages) {
        /// TODO: use previous append message calls as context
        if (messages.length === 0) {
            return
        }
        let previousMessage = null;
        for (let message of messages) {
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
        await appendMessages(messages);
        let messagesDiv = document.querySelector("#messages");
        messagesDiv.scrollTop = 0;
    }

    async function handleMessageKeyDown(event) {
        if (event.keyCode === 13) {
            await sendMessage(document.querySelector("#messageInput").value).then(async (message) => {
                document.querySelector("#messageInput").value = "";
                await appendMessages([message]);
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
            await getMoreMessages(last).then(async (messages) => {
                await appendMessages(messages);
            });
            updating = false;
        }
    }

    load().then(async () => {
        let channels = await getChannelList();
        let categories = {};
        let categoryInfo = {};
        for (let channel of channels) {
            let category = channel.category;
            if (category == null && channel.isdm) {
                category = {
                    "id": -2,
                    "title": "Direct Messages",
                };
            } else if (category == null) {
                category = {
                    "id": -1,
                    "title": "Uncategorized",
                };
            }
            if (categories[category.id] == null) {
                categories[category.id] = [];
                categoryInfo[category.id] = category;
            }
            categories[category.id].push(channel);

        }

        console.log(categoryInfo);
        console.log(categories);

        let sidebarList = document.querySelector("#sidebar-list");
        for (let category in categories) {
            let categoryElement = new Sidecategory({
                target: sidebarList,
                props: {
                    name: categoryInfo[category].title,
                    items: categories[category],
                    buttonClick: handleSidebarClick,
                }
            });
        }
    });
</script>

<div class="flex flex-row font-sans h-dvh bg-white dark:bg-gray-900">
    <aside id="default-sidebar"
           aria-label="Sidebar">
        <div class="h-full w-[375px] overflow-y-auto overflow-x-hidden px-3 py-4 bg-gray-50 dark:bg-gray-800">
            <ul class="space-y-2 font-medium" id="sidebar-list">
            </ul>
        </div>
    </aside>
    <div id="content" class="h-full w-full bg-white dark:bg-gray-900 flex flex-col">
        <div id="messages" class="overflow-y-auto bg-white dark:bg-gray-900 flex flex-col-reverse" on:scroll={messageScroll}>
        </div>
        <div class="w-full border-t border-gray-500 mt-auto bg-white dark:bg-gray-900">
            <input id="messageInput" type="text" class="text-gray-900 dark:text-white bg-white dark:bg-gray-900 outline-0 w-full border-0 h-[50px] text-base border-none px-4" on:keydown={handleMessageKeyDown}>
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
