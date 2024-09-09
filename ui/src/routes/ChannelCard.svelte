<script>
    import {getChannelInfo} from "./api.js";

    let info = null;

    setInterval(async () => {
        info = await getChannelInfo();
    }, 10);
</script>
{#if info !== null}
    <div class="h-[60px] border-b border-gray-500 flex w-full items-center text-gray-900 dark:text-white text-lg my-auto px-5 flex-row">
        <div class="flex flex-col">
            <span>{info[0].title}</span>
            <span class="text-sm text-gray-500 dark:text-gray-400">{info[0].category.title}</span>
        </div>
        <div class="ml-auto hover:bg-gray-300 dark:hover:bg-slate-700 px-1 py-2 rounded-lg">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 12a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM12.75 12a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM18.75 12a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0Z" />
            </svg>
        </div>
    </div>
{/if}