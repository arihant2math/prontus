<script lang="ts">
    import {Tabs} from "bits-ui";

    interface Props {
        value: string;
        disabled?: boolean;
        children?: import('svelte').Snippet;
    }

    let { value, disabled = false, children }: Props = $props();
</script>
<Tabs.Trigger value={value} disabled={disabled}
              class="h-8 rounded-[7px] py-2 data-[state=active]:bg-gray-100 dark:data-[state=active]:bg-slate-700 data-[state=active]:shadow-mini dark:data-[state=active]:bg-muted">
    {@render children?.()}
</Tabs.Trigger>