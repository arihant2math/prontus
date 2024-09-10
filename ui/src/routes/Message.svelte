<script>
    import Embed from "./messageComponents/Embed.svelte";
    import Media from "./messageComponents/Media.svelte";
    import Reaction from "./messageComponents/Reaction.svelte";
    import {deleteMessage, setReactionState} from "./api.js";
    import ProfilePicture from "./ProfilePicture.svelte";
    import RichTextContainer from "./messageComponents/RichTextContainer.svelte";
    import {positionPopovers} from "./popup.js";

    export let message;
    export let repeat = false;
    export let currentUser;

    $: isCurrentUser = currentUser.id === user.id;
    $: systemMessage = message.systemevent != null;
    $: user = message.user;
    $: timestamp = "";
    $: pfp_url = message.user.profilepicurl;
    $: media = message.messagemedia;
    $: embed = message.resource;
    $: reactions = message.reactionsummary;
    $: repeat = repeat;
    $: ml = repeat ? "ml-10" : "ml-0";
    $: messageId = message.id.toString() + "Message";
    $: actionsId = message.id.toString() + "MessageActions";
    $: reactionsId = message.id.toString() + "MessageReactions";

    function reactionsShow() {
        positionPopovers();
        let reactions = document.getElementById(reactionsId);
        reactions.classList.remove("invisible");
    }

    function reactionsHide() {
        positionPopovers();
        let reactions = document.getElementById(reactionsId);
        reactions.classList.add("invisible");
    }

    function reactionsToggle() {
        positionPopovers();
        let reactions = document.getElementById(reactionsId);
        reactions.classList.toggle("invisible");
    }

    function actionsShow() {
        positionPopovers();
        let dropdown = document.getElementById(actionsId);
        dropdown.classList.remove("hidden");
    }

    function actionsHide() {
        positionPopovers();
        let dropdown = document.getElementById(actionsId);
        dropdown.classList.add("hidden");
    }

    async function remove() {
        positionPopovers();
        console.log("Deleting message " + message.id);
        await deleteMessage(message.id);
    }

    async function react(react_id) {
        reactionsHide();
        // TODO: Allow for unreacting too
        await setReactionState(message.id, react_id, true);
    }

    document.body.addEventListener('click', function (event) {
        let reactions = document.getElementById(reactionsId);
        let actions = document.getElementById(actionsId);
        if (reactions !== null && !reactions.classList.contains("invisible") && !reactions.contains(event.target) && !actions.contains(event.target)) {
            reactionsHide();
        }
    });
</script>
{#if !systemMessage}
    <div id="{messageId}" class="pl-5 py-2 flex items-start gap-2.5 hover:bg-gray-100 dark:hover:bg-slate-800" on:mouseenter={actionsShow} on:mouseleave={actionsHide} role="listitem">
        {#if !repeat}
            <ProfilePicture user={message.user}/>
        {/if}
        <div class="{ml} flex flex-col w-full max-w-[500px] leading-1.5">
            {#if !repeat}
                <div class="flex items-center space-x-2 rtl:space-x-reverse">
                    <span class="text-sm font-semibold text-gray-900 dark:text-white">{user.fullname}</span>
                    <span class="text-sm font-normal text-gray-500 dark:text-gray-400">{timestamp}</span>
                </div>
            {/if}
            <RichTextContainer message="{message.message}"/>
<!--            <p class="text-sm font-normal py-2.5 text-gray-900 dark:text-white">{message.message}</p>-->
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
        <ul data-popover data-popover-target="{messageId}" data-popover-position="right" data-popover-offset="-300" class="invisible fixed flex flex-row text-sm bg-white dark:bg-slate-800 text-gray-700 dark:text-gray-200 rounded-lg shadow-lg" id="{reactionsId}">
            <li>
                <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" on:click={() => react(1)}>
                    👍
                </button>
            </li>
            <li>
                <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" on:click={() => react(2)}>
                    👎
                </button>
            </li>
            <li>
                <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" on:click={() => react(3)}>
                    😂
                </button>
            </li>
            <li>
                <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" on:click={() => react(4)}>
                    💓
                </button>
            </li>
            <li>
                <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" on:click={() => react(5)}>
                    😢
                </button>
            </li>
            <li>
                <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" on:click={() => react(6)}>
                    😲
                </button>
            </li>
        </ul>
        <ul class="ml-auto mr-2 flex flex-row text-sm bg-white dark:bg-slate-900 text-gray-700 dark:text-gray-200 hidden rounded-lg shadow-md" id="{actionsId}">
            <li>
                <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" disabled>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M7.49 12 3.74 8.248m0 0 3.75-3.75m-3.75 3.75h16.5V19.5" />
                    </svg>
                </button>
            </li>
<!--            TODO: Forward button -->
            <li>
                <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" on:click={reactionsToggle}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M15.182 15.182a4.5 4.5 0 0 1-6.364 0M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0ZM9.75 9.75c0 .414-.168.75-.375.75S9 10.164 9 9.75 9.168 9 9.375 9s.375.336.375.75Zm-.375 0h.008v.015h-.008V9.75Zm5.625 0c0 .414-.168.75-.375.75s-.375-.336-.375-.75.168-.75.375-.75.375.336.375.75Zm-.375 0h.008v.015h-.008V9.75Z" />
                    </svg>
                </button>
            </li>
            {#if isCurrentUser}
                <li>
                    <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" disabled>
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
{:else}
    <div class="flex items-start gap-2.5">
        <div class="flex flex-row items-center p-4">
            <span class="text-sm font-semibold text-gray-900 dark:text-white">{user.fullname}</span>
            <p class="ml-2 text-sm font-normal py-2.5 text-gray-900 dark:text-white">{message.message}</p>
        </div>
    </div>
{/if}