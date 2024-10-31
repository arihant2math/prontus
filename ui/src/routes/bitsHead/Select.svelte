<script lang="ts">
    import {Select} from "bits-ui";
    import Icon from "@krowten/svelte-heroicons/Icon.svelte";

    let { options = $bindable([]), label = "", selected = $bindable(), onSelectedChange } = $props();
</script>

<Select.Root bind:items={options} bind:selected={selected} onSelectedChange={onSelectedChange}>
    <Select.Trigger
            class="rounded-lg py-2 inline-flex h-input w-[296px] items-center px-[11px] text-sm transition-colors placeholder:text-foreground-alt/50  focus:outline-none border border-gray-100 dark:border-gray-700"
            aria-label={label}
    >
        <Select.Value class="text-sm" placeholder={label}/>
        <Icon name="chevron-up-down" class="w-6 h-6 ml-auto"/>
    </Select.Trigger>
    <Select.Content
            class="w-full rounded-xl bg-white dark:bg-slate-800 px-1 py-3 shadow-md outline-none"
            sideOffset={8}
    >
        {#each options as option}
            <Select.Item
                    class="flex h-10 w-full select-none items-center hover:bg-gray-100 dark:hover:bg-slate-700 rounded-button py-3 pl-5 pr-1.5 text-sm outline-none transition-all duration-75 data-[highlighted]:bg-muted"
                    value={option.value}
                    label={option.label}
            >
                {option.label}
                <Select.ItemIndicator class="ml-auto" asChild={false}>
                    <Icon name="check" class="w-5 h-5"/>
                </Select.ItemIndicator>
            </Select.Item>
        {/each}
    </Select.Content>
    <Select.Input/>
</Select.Root>