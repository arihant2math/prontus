<script>
    import Embed from "./embed.svelte";
    import Reaction from "./reaction.svelte";

    export let message_id = -1;
    export let user = "";
    export let timestamp = "";
    export let pfp_url = "";
    export let message = "";
    export let embed;
    export let reactions;
    export let repeat = false;
    export let systemMessage = false;


    $: user = user;
    $: timestamp = timestamp;
    $: pfp_url = pfp_url;
    $: message = message;
    $: embed = embed;
    $: reactions = reactions;
    $: repeat = repeat;
    $: mt = repeat ? "mt-2" : "mt-10";
    $: ml = repeat ? "ml-10" : "ml-0";
    $: dropdownId = message_id.toString() + "MessageDropdown";

    function handleDropdownToggle() {
        console.log("Toggling dropdown");
        let dropdown = document.getElementById(dropdownId);
        dropdown.classList.toggle("hidden");
    }
</script>
{#if !systemMessage}
    <div class="{mt} flex items-start gap-2.5">
        {#if !repeat}
            <img class="w-8 h-8 rounded-full" src="{pfp_url}" alt="{user} image">
        {/if}
        <div class="{ml} flex flex-col w-full max-w-[500px] leading-1.5 p-4 border-gray-200 bg-gray-100 rounded-e-xl rounded-es-xl dark:bg-gray-700">
            {#if !repeat}
                <div class="flex items-center space-x-2 rtl:space-x-reverse">
                    <span class="text-sm font-semibold text-gray-900 dark:text-white">{user}</span>
                    <span class="text-sm font-normal text-gray-500 dark:text-gray-400">{timestamp}</span>
                </div>
            {/if}
            <p class="text-sm font-normal py-2.5 text-gray-900 dark:text-white">{message}</p>
            {#if embed}
                <Embed title="{embed.title}" shortUrl="{embed.providerurl}" description="{embed.snipped}" image="{embed.thumbnailurl}"/>
            {/if}
            <div class="flex items-center space-x-2">
                {#each reactions as reaction}
                    <Reaction id={reaction.reactiontype_id} count={reaction.count}/>
                {/each}
            </div>
        </div>
        <button on:click={handleDropdownToggle} class="inline-flex self-center items-center p-2 text-sm font-medium text-center text-gray-900 bg-white rounded-lg hover:bg-gray-100 focus:ring-4 focus:outline-none dark:text-white focus:ring-gray-50 dark:bg-gray-900 dark:hover:bg-gray-800 dark:focus:ring-gray-600" type="button">
            <svg class="w-4 h-4 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 4 15">
                <path d="M3.5 1.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Zm0 6.041a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Zm0 5.959a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Z"/>
            </svg>
        </button>
        <div id="{dropdownId}" class="z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-40 dark:bg-gray-700 dark:divide-gray-600">
            <ul class="py-2 text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdownMenuIconButton">
                <li>
                    <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">Reply</a>
                </li>
                <li>
                    <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">Forward</a>
                </li>
                <li>
                    <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">Copy</a>
                </li>
                <li>
                    <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">Report</a>
                </li>
                <li>
                    <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">Delete</a>
                </li>
            </ul>
        </div>
    </div>
{:else}
    <div class="{mt} flex items-start gap-2.5">
        <div class="flex flex-row items-center p-4">
            <span class="text-sm font-semibold text-gray-900 dark:text-white">{user}</span>
            <p class="ml-2 text-sm font-normal py-2.5 text-gray-900 dark:text-white">{message}</p>
        </div>
    </div>
{/if}