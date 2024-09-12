<script>
    import Message from "./Message.svelte";
    import {positionPopovers} from "$lib/popup.js";
    import {getMessages, getMoreMessages} from "$lib/api.js";

    export let messages;
    export let currentUser;

    $: messages = messages;

    let updating = false;

    function appendMessages(newMessages) {
        messages = messages.concat(newMessages);
    }

    async function messageScroll(event) {
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

    setInterval(async () => {
        messages = await getMessages();
    }, 10);
</script>

<div id="messages" class="overflow-y-scroll bg-white dark:bg-slate-900 flex flex-col-reverse" on:scroll={messageScroll}>
    {#each messages as message}
        <!--TODO: Get repeat working-->
        <Message message={message} repeat={false} currentUser={currentUser}/>
    {/each}
</div>