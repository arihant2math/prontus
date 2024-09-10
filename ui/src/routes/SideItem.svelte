<script>
    import ProfilePicture from "./ProfilePicture.svelte";

    export let info;
    export let stats;
    export let buttonClick;

    function btnClick() {
        buttonClick(bubbleId);
    }
</script>
<li>
    <button on:click={btnClick} class="flex items-start p-2 text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-slate-700 w-full text-ellipsis">
        {#if info.isdm}
            <div class="relative">
                <ProfilePicture user="{info.dmpartner}"/>
                {#if !info.dmpartner.isonline}
                    <span class="bottom-0 left-7 absolute bg-gray-300 dark:bg-gray-600 w-3.5 h-3.5 bg-transparent border-2 border-white dark:border-gray-800 rounded-full"></span>
                {:else}
                    <span class="bottom-0 left-7 absolute  w-3.5 h-3.5 bg-green-400 border-2 border-white dark:border-gray-800 rounded-full"></span>
                {/if}
            </div>
        {/if}
        <span class="text-sm text-left ms-3 flex-1 whitespace-nowrap text-truncate">{info.title}</span>
        { #if stats.unread > 0 }
            {#if stats.unread_mentions > 0}
                <span class="inline-flex items-center justify-center w-3 h-3 p-3 ms-3 text-xs font-medium text-blue-800 bg-blue-100 rounded-full dark:bg-blue-900 dark:text-blue-300">{stats.unread_mentions}</span>
            {:else}
                <span class="inline-flex items-center justify-center px-2 ms-3 text-xs font-medium text-gray-800 bg-gray-100 rounded-full dark:bg-gray-600 dark:text-gray-300">{stats.unread}</span>
            {/if}
        {/if}
    </button>
</li>