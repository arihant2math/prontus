<script>
    import Message from "./Message.svelte";
    import {positionPopovers} from "$lib/popup.js";
    import {getMessages, getMoreMessages} from "$lib/api.js";

    export let messages;
    export let viewThread;
    export let currentUser;
    export let inThread = false;
    export let id;

    let updating = false;

    function appendMessages(newMessages) {
        messages = messages.concat(newMessages);
    }

    async function messageScroll(event) {
        if (inThread) {
            return;
        }
        // TODO: Fix hack, this should be global
        positionPopovers();
        if (event.target.scrollTop + event.target.scrollHeight < 1000) {
            if (updating) {
                return;
            }
            updating = true;
            console.info("Loading more messages");
            let messages = await getMessages();
            let last = messages[messages.length - 1].id;
            await getMoreMessages(last).then(async (messages) => {
                appendMessages(messages);
            });
            updating = false;
        }
    }

    if (!inThread) {
        setInterval(async () => {
            messages = await getMessages();
        }, 10);
    }
    positionPopovers();
</script>

<div id="{id}" class="overflow-y-scroll bg-white dark:bg-slate-900 flex flex-col-reverse h-full w-full" on:scroll={messageScroll} onload="this.scrollTop=0">
    {#each messages as message}
        <!--TODO: Get repeat working-->
        <Message message={message} repeat={false} currentUser={currentUser} viewThread={viewThread} inThread={inThread}/>
    {/each}
</div>