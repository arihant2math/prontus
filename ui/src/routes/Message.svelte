<script>
    import Embed from "./messageComponents/Embed.svelte";
    import Media from "./messageComponents/Media.svelte";
    import Reaction from "./messageComponents/Reaction.svelte";
    import {getMessage, deleteMessage, editMessage} from "$lib/api.ts";
    import RichTextContainer from "./messageComponents/RichTextContainer.svelte";
    import {positionPopovers} from "$lib/popup.js";
    import {parseDatetime} from "$lib/helpers.ts";
    import ReactionPanel from "./messageComponents/ReactionPanel.svelte";
    import InteractiveProfilePicture from "./user/InteractiveProfilePicture.svelte";
    import RichTextEdit from "./messageComponents/RichTextEdit.svelte";
    import ViewTheadFooter from "./messageComponents/ViewThreadFooter.svelte";

    export let message;
    export let previousMessage = null;
    export let nextMessage = null;
    export let currentUser;
    export let viewThread;
    export let inThread;
    export let messages;
    export let settings;

    let editing = false;

    function strft(date) {
        return date.toLocaleString('en-US', {
            month: 'numeric',
            day: 'numeric',
            year: 'numeric',
            hour: 'numeric',
            minute: 'numeric',
            second: 'numeric',
            hour12: true
        });
    }

    $: unsent = message.hasOwnProperty("unsent");
    $: dateSpan = spanDate(message, previousMessage);
    $: repeat = isRepeat(message, previousMessage);
    $: firstThreadMessage = isFirstThreadMessage(message, previousMessage);
    $: lastThreadMessage = isLastThreadMessage(message, nextMessage);
    $: isCurrentUser = currentUser.id === message.user.id;
    $: systemMessage = message.systemevent != null;
    $: user = message.user;
    $: media = message.messagemedia;
    $: embed = message.resource;
    $: reactions = message.reactionsummary;
    $: messageCreatedAtDatetime = strft(parseDatetime(message.created_at));
    $: messageCreatedatDate = parseDatetime(message.created_at).toDateString();
    $: ml = repeat ? "ml-10" : "ml-0";
    $: py = repeat ? "py-1" : "py-3";
    $: border = parentMessage === undefined ? "" : "border-l border-blue-500 dark:border-blue-400";


    function formatTime(date) {
        let datetime = parseDatetime(date);
        // TODO: AM/PM (and make this configurable)
        return datetime.getMonth() + "/" + datetime.getDay() + "/" + datetime.getFullYear() + " " + (datetime.getHours() % 12) + ":" + datetime.getMinutes() + ":" + datetime.getSeconds();
    }

    function isRepeat() {
        if (previousMessage === null) {
            return false;
        }
        let currentDatetime = new Date(message.created_at);
        let previousDatetime = new Date(previousMessage.created_at);
        if (currentDatetime.getDay() != previousDatetime.getDay()) {
            return false;
        }
        return previousMessage.user.id === message.user.id && previousMessage.systemevent == null;
    }


    function isFirstThreadMessage() {
        if (previousMessage === null) {
            return true;
        }
        return !(previousMessage.parentmessage_id === message.parentmessage_id);
    }

    function isLastThreadMessage() {
        if (nextMessage === null) {
            return true;
        }

        return !(nextMessage.parentmessage_id === message.parentmessage_id);
    }

    function spanDate() {
        if (previousMessage === null) {
            return true;
        }
        let currentDatetime = parseDatetime(message.created_at);
        let previousDatetime = parseDatetime(previousMessage.created_at);
        return currentDatetime.getDay() !== previousDatetime.getDay();
    }

    async function edit() {
        editing = true;
    }

    async function sendEditMessage(newMessage) {
        editing = false;
        editMessage(message.id, newMessage)
    }

    async function remove() {
        positionPopovers();
        console.log("Deleting message " + message.id);
        await deleteMessage(message.id);
    }

    $: parentMessage = getParentMessage(message, messages);

    function getParentMessage(message, messages) {
        if (message.parentmessage_id === null) {
            return undefined;
        }
        for (let i = 0; i < messages.length; i++) {
            if (messages[i].id === message.parentmessage_id) {
                return messages[i];
            }
        }
        return null;
    }
</script>
{#if editing}
    <RichTextEdit bind:text={message.message} sendMessage={sendEditMessage}/>
{:else}
    {#if !systemMessage && !settings.appearance.messages.compact}
        <div class="flex flex-col">
            {#if dateSpan}
                <!--    TODO: Debug and then put a proper date span here-->
                <p class="text-gray-500 text-center">{messageCreatedatDate}</p>
            {/if}
            {#if !inThread && parentMessage !== undefined && firstThreadMessage}
                <button on:click={() => {viewThread(parentMessage.id)}} class="max-w-[500px] p-2 rounded-xl bg-gray-50 dark:bg-slate-700 w-max">
                    {#if parentMessage !== null}
                        <p class="text-xs line-clamp-1"><b>{parentMessage.user.fullname}</b> {parentMessage.message}</p>
                    {:else}
                        <p class="text-xs"><b>Loading</b> loading</p>
                    {/if}
                </button>
            {/if}
            <div class="pl-5 {py} flex items-start gap-2.5 hover:bg-gray-100 dark:hover:bg-slate-800 {border}" role="listitem">
                {#if !repeat}
                    <InteractiveProfilePicture user={message.user}/>
                {/if}
                <div class="{ml} flex flex-col w-full max-w-[500px] leading-1.5 space-y-2">
                    {#if !repeat}
                        <div class="flex items-center space-x-2 rtl:space-x-reverse">
                            <span class="text-sm font-semibold text-gray-900 dark:text-white text-nowrap">{user.fullname}</span>
                            <span class="text-sm font-normal text-gray-500 dark:text-gray-400 text-nowrap">{messageCreatedAtDatetime}</span>
                        </div>
                    {/if}
                    <RichTextContainer message="{message.message}"/>
                    {#each media as mediaItem}
                        <Media url={mediaItem.url} type={mediaItem.mediatype} mimetype="{mediaItem.urlmimetype}"/>
                    {/each}
                    {#if embed && !settings.appearance.messages.hide_embeds}
                        <Embed title="{embed.title}" shortUrl="{embed.providerurl}" description="{embed.snippet}" image="{embed.thumbnailurl}"/>
                    {/if}
                    <div class="flex items-center space-x-2">
                        {#each reactions as reaction}
                            <Reaction id={reaction.reactiontype_id} messageId={message.id} count={reaction.count} users={reaction.users} currentUser={currentUser}/>
                        {/each}
                    </div>
                </div>
                <ReactionPanel message_id={message.id}/>
                <ul class="fixed hidden flex flex-row text-sm bg-white dark:bg-slate-900 text-gray-700 dark:text-gray-200 rounded-lg shadow-md" data-popover data-popover-target-parent data-popover-configure data-popover-show-method="hover" data-popover-position="right" data-popover-offset="-150">
                    {#if !inThread}
                        <li>
                            <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" on:click={() => {viewThread(message.id)}}>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M7.49 12 3.74 8.248m0 0 3.75-3.75m-3.75 3.75h16.5V19.5" />
                                </svg>
                            </button>
                        </li>
                    {/if}
                    <!--            TODO: Forward button -->
                    <li>
                        <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" data-popover-ref-target="reaction-panel">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                              <path stroke-linecap="round" stroke-linejoin="round" d="M15.182 15.182a4.5 4.5 0 0 1-6.364 0M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0ZM9.75 9.75c0 .414-.168.75-.375.75S9 10.164 9 9.75 9.168 9 9.375 9s.375.336.375.75Zm-.375 0h.008v.015h-.008V9.75Zm5.625 0c0 .414-.168.75-.375.75s-.375-.336-.375-.75.168-.75.375-.75.375.336.375.75Zm-.375 0h.008v.015h-.008V9.75Z" />
                            </svg>
                        </button>
                    </li>
                    {#if isCurrentUser}
                        <li>
                            <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" on:click={edit}>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L6.832 19.82a4.5 4.5 0 0 1-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 0 1 1.13-1.897L16.863 4.487Zm0 0L19.5 7.125" />
                                </svg>
                            </button>
                        </li>
                        <li>
                            <button
                                    class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" on:click={remove}>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0" />
                                </svg>
                            </button>
                        </li>
                    {/if}
                </ul>
            </div>
            {#if lastThreadMessage && parentMessage !== undefined && !inThread}
                <ViewTheadFooter onClick={() => {viewThread(parentMessage.id)}}/>
            {/if}
        </div>
    {:else}
        <div class="flex items-start gap-2.5">
            <div class="flex flex-row items-center p-4">
                <span class="text-sm font-semibold text-gray-900 dark:text-white text-nowrap">{user.fullname}</span>
                <p class="ml-2 text-sm font-normal py-2.5 text-gray-900 dark:text-white">{message.message}</p>
            </div>
        </div>
    {/if}
{/if}