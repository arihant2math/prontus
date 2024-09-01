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
            let messageElement = new Message({
                target: document.querySelector("#messages"),
                props: {
                    user: message.user.fullname,
                    message: message.message,
                    timestamp: message.timestamp,
                    pfp_url: message.user.profilepicurl
                }
            });
        }
    }

    loadChannelList().then(async () => {
        console.log("Channel list loaded");
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

<div class="flex flex-col font-sans h-dvh">
    <aside id="default-sidebar"
           class="fixed top-0 left-0 z-40 w-64 h-screen transition-transform -translate-x-full sm:translate-x-0"
           aria-label="Sidebar">
        <div class="h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-800">
            <ul class="space-y-2 font-medium" id="sidebar-list">
            </ul>
        </div>
    </aside>
    <div id="content" class="ml-64 bg-white dark:bg-gray-900 h-full">
        <div id="messages" class="ml-4 mr-4 space-y-10">
            <Message user="User" message="test" timestamp="11:10"
                     pfp_url="https://flowbite.com/docs/images/people/profile-picture-3.jpg"/>
        </div>
    </div>
</div>