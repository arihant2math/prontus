<script>
    import {setReactionState} from "$lib/api.ts";

    /** @type {{message_id: any}} */
    let {message_id, reactions} = $props();

    async function react(react_id) {
        await setReactionState(message_id, react_id, !reactions.find(r => r.reaction_id === react_id));
    }
</script>

<ul data-popover data-popover-target-parent data-popover-ref="reaction-panel" data-popover-position="right" data-popover-offset="-300" data-popover-configure class="fixed hidden flex flex-row text-sm bg-white dark:bg-slate-800 text-gray-700 dark:text-gray-200 rounded-lg shadow-lg">
    <li>
        <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" onclick={() => react(1)}>
            👍
        </button>
    </li>
    <li>
        <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" onclick={() => react(2)}>
            👎
        </button>
    </li>
    <li>
        <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" onclick={() => react(3)}>
            😂
        </button>
    </li>
    <li>
        <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" onclick={() => react(4)}>
            ❤️
        </button>
    </li>
    <li>
        <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" onclick={() => react(5)}>
            😢
        </button>
    </li>
    <li>
        <button class="block w-full text-left px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white" onclick={() => react(6)}>
            😲
        </button>
    </li>
</ul>