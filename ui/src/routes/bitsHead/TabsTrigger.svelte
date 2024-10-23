<script lang="ts">
    import {Tabs} from "bits-ui";

    export let value: string;
    export let disabled: boolean = false;
</script>
<Tabs.Trigger value={value} disabled={disabled}
              class="h-8 rounded-[7px] py-2 data-[state=active]:bg-white dark:data-[state=active]:bg-slate-700 data-[state=active]:shadow-mini dark:data-[state=active]:bg-muted">
    <slot></slot>
</Tabs.Trigger>