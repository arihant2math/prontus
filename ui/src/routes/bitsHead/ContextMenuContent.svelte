<script lang="ts">
    import {ContextMenu} from "bits-ui";

    interface Props {
        children?: import('svelte').Snippet;
    }

    let {children}: Props = $props();
</script>
<ContextMenu.Content
        class="z-50 w-full max-w-[229px] rounded-xl bg-white dark:bg-slate-800 px-1 py-1.5 shadow-lg">
    {@render children?.()}
</ContextMenu.Content>