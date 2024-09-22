<script>
    import Message from "./Message.svelte";
    import {positionPopovers} from "$lib/popup.js";
    import {getMessages, getMoreMessages} from "$lib/api.ts";

    export let messages;
    export let parentMessages;
    export let viewThread;
    export let currentUser;
    export let inThread = false;
    export let id;
    export let settings;

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

    positionPopovers();
</script>

<div id="{id}" class="overflow-y-scroll bg-white dark:bg-slate-900 flex flex-col-reverse h-full w-full" on:scroll={messageScroll} onload="this.scrollTop=0">
    {#each messages as message, i}
        {#if i < messages.length - 1 && i > 0}
            <Message message={message} previousMessage={messages[i+1]} nextMessage={messages[i-1]} currentUser={currentUser} viewThread={viewThread} inThread={inThread} messages={parentMessages} bind:settings={settings}/>
        {:else if i < message.length - 1}
            <Message message={message} previousMessage={messages[i+1]} currentUser={currentUser} viewThread={viewThread} inThread={inThread} messages={parentMessages} bind:settings={settings}/>
        {:else if i > 0}
            <Message message={message} nextMessage={messages[i-1]} currentUser={currentUser} viewThread={viewThread} inThread={inThread} messages={parentMessages} bind:settings={settings}/>
        {:else}
            <Message message={message} currentUser={currentUser} viewThread={viewThread} inThread={inThread} messages={parentMessages} bind:settings={settings}/>
        {/if}
    {/each}
</div>