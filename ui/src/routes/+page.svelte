<script>
    import "../app.css";
    import {invoke} from "@tauri-apps/api/core";
    import Message from "./message.svelte";
    import Sideitem from "./sideitem.svelte";

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

    async function loadMessages() {
        return await invoke("load_messages");
    }

    async function handleSidebarClick(id) {
        await loadChannel(id);
        await loadMessages();
        let messages = await getMessages();
        document.querySelector("#messages").innerHTML = "";
        for (let message of messages) {
            console.log(message);
            let messageElement = new Message({
                target: document.querySelector("#messages"),
                props: {
                    user: message.user.fullname,
                    message: message.message,
                    timestamp: message.timestamp,
                    pfp_url: message.user.profilepicurl,
                    embed: message.resource,
                }
            });
        }
        let messagesDiv = document.querySelector("#messages");
        messagesDiv.scrollTop = messagesDiv.scrollHeight;
    }

    loadChannelList().then(async () => {
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
    <div id="content" class="h-full w-full overflow-y-auto bg-white dark:bg-gray-900">
        <div id="messages" class="ml-4 mr-4 space-y-10 bg-white dark:bg-gray-900">
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