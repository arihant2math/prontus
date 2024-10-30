<script lang="ts">
    import Message from "./messageComponents/MessageListItem.svelte";
    import {positionPopovers} from "$lib/popup.js";
    import {getCurrentChannelId, getMessages, getMoreMessages} from "$lib/api.ts";
    import { flip } from 'svelte/animate';
    import { quintOut } from 'svelte/easing';

    let {
        messages = $bindable(),
        parentMessages = $bindable(),
        channelInfo,
        currentUser,
        inThread = false,
        viewThread,
        settings,
        createDm,
        pulsing = false
    } = $props();

    let memberships = [];
    $effect(() => {
        getMemberships()
    });
    let updating = false;
    const pulsingClass = $derived(pulsing ? "animate-pulse" : "");

    function getMemberships() {
        getCurrentChannelId().then((info) => {
            memberships = info.memberships;
        });
    }

    function appendMessages(newMessages) {
        messages = messages.concat(newMessages);
    }

    async function messageScroll(event) {
        if (inThread) {
            return;
        }
        // TODO: Fix hack, this should be global
        positionPopovers();
        console.log(event.target.scrollTop, event.target.scrollHeight);
        if (event.target.scrollTop + event.target.scrollHeight < 2500) {
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

<div class="overflow-y-scroll bg-white dark:bg-slate-900 flex flex-col-reverse h-full w-full {pulsingClass}" onscroll={messageScroll} onload={() => {this.scrollTop=0}}>
    {#each messages as message, i (message.id)}
        <div animate:flip={{ delay: 200, duration: 250, easing: quintOut }}>
            {#if message !== undefined && memberships !== undefined}
                {#if i < messages.length - 1 && i > 0}
                    <Message message={message} memberships={memberships} previousMessage={messages[i+1]} nextMessage={messages[i-1]} currentUser={currentUser} viewThread={viewThread} inThread={inThread} messages={parentMessages} settings={settings} createDm={createDm}/>
                {:else if i === 0}
                    <Message message={message} memberships={memberships} previousMessage={messages[i+1]} currentUser={currentUser} viewThread={viewThread} inThread={inThread} messages={parentMessages} settings={settings} createDm={createDm}/>
                {:else if i === message.length - 1}
                    <Message message={message} memberships={memberships} nextMessage={messages[i-1]} currentUser={currentUser} viewThread={viewThread} inThread={inThread} messages={parentMessages} settings={settings} createDm={createDm}/>
                {:else}
                    <Message message={message} memberships={memberships} currentUser={currentUser} viewThread={viewThread} inThread={inThread} messages={parentMessages} settings={settings} createDm={createDm}/>
                {/if}
            {/if}
            {#if channelInfo !== null && message.id === channelInfo[2].mark && i !== 0}
                <div class="relative flex py-5 items-center">
                    <div class="flex-grow border-t border-red-500 dark:border-red-400"></div>
                    <span class="flex-shrink mx-4 text-red-500 dark:text-red-400 select-none">Unread</span>
                    <div class="flex-grow border-t border-red-500 dark:border-red-400"></div>
                </div>
            {/if}
        </div>
    {/each}
</div>