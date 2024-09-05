<script>
    import Embed from "./messageComponents/embed.svelte";
    import Media from "./messageComponents/media.svelte";
    import Reaction from "./messageComponents/reaction.svelte";
    import {deleteMessage} from "./api.js";

    export let message;
    export let repeat = false;
    export let currentUser;

    console.log(message);

    // TODO: use id comparison instead of string comparison
    $: isCurrentUser = currentUser.fullname == user.fullname;
    $: systemMessage = message.systemevent != null;
    $: user = message.user;
    $: timestamp = "";
    $: pfp_url = message.user.profilepicurl;
    $: media = message.messagemedia;
    $: embed = message.resource;
    $: reactions = message.reactionsummary;
    $: repeat = repeat;
    $: mt = repeat ? "mt-2" : "mt-10";
    $: ml = repeat ? "ml-10" : "ml-0";
    $: actionsId = message.id.toString() + "MessageDropdown";

    function actionsShow() {
        console.log("Toggling dropdown");
        let dropdown = document.getElementById(actionsId);
        dropdown.classList.add("hidden");
    }

    function actionsHide() {
        console.log("Toggling dropdown");
        let dropdown = document.getElementById(actionsId);
        dropdown.classList.remove("hidden");
    }

    async function remove() {
        console.log("Deleting message " + message.id);
        await deleteMessage(message.id);
    }
</script>
{#if !systemMessage}
    <div class="pl-5 py-2 flex items-start gap-2.5 hover:bg-gray-100 dark:hover:slate-800" on:mouseleave={actionsShow} on:mouseenter={actionsHide} role="listitem">
        {#if !repeat}
            <img class="w-8 h-8 rounded-full" src="{pfp_url}" alt="{user.fullname} image">
        {/if}
        <div class="{ml} flex flex-col w-full max-w-[500px] leading-1.5">
            {#if !repeat}
                <div class="flex items-center space-x-2 rtl:space-x-reverse">
                    <span class="text-sm font-semibold text-gray-900 dark:text-white">{user.fullname}</span>
                    <span class="text-sm font-normal text-gray-500 dark:text-gray-400">{timestamp}</span>
                </div>
            {/if}
            <p class="text-sm font-normal py-2.5 text-gray-900 dark:text-white">{message.message}</p>
            {#each media as mediaItem}
                <Media url={mediaItem.url} type={mediaItem.mediatype} mimetype="{mediaItem.urlmimetype}"/>
            {/each}
            {#if embed}
                <Embed title="{embed.title}" shortUrl="{embed.providerurl}" description="{embed.snippet}" image="{embed.thumbnailurl}"/>
            {/if}
            <div class="flex items-center space-x-2">
                {#each reactions as reaction}
                    <Reaction id={reaction.reactiontype_id} messageId={message.id} count={reaction.count} users={reaction.users} currentUser={currentUser}/>
                {/each}
            </div>
        </div>
        <ul class="flex flex-row fixed right-5 py-2 text-sm text-gray-700 dark:text-gray-200 hidden" id="{actionsId}">
            <li>
                <button class="block w-full text-left px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" disabled>Reply</button>
            </li>
            <li>
                <button class="block w-full text-left px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" disabled>Forward</button>
            </li>
            <li>
                <button class="block w-full text-left px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" disabled>Copy</button>
            </li>
            {#if isCurrentUser}
                <li>
                    <button class="block w-full text-left px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" disabled>Edit</button>
                </li>
                <li>
                    <button
                            class="block w-full text-left px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" on:click={remove}>Delete</button>
                </li>
            {/if}
        </ul>
    </div>
{:else}
    <div class="{mt} flex items-start gap-2.5">
        <div class="flex flex-row items-center p-4">
            <span class="text-sm font-semibold text-gray-900 dark:text-white">{user.fullname}</span>
            <p class="ml-2 text-sm font-normal py-2.5 text-gray-900 dark:text-white">{message.message}</p>
        </div>
    </div>
{/if}