<script>
    import { PinInput, Toggle } from "bits-ui";

    /** @type {{value?: any}} */
    let { value = $bindable([]) } = $props();
    // TODO: onchange prop

</script>

<PinInput.Root
        bind:value
        class="flex items-center gap-2"
        type="text"
        placeholder="0"
>
    <PinInput.Input
            class="flex h-input w-10 select-none rounded-input border border-foreground bg-grey-100 dark:bg-slate-800 px-2 py-3 text-center font-alt text-[17px] tracking-[0.01em] text-foreground placeholder-shown:border-border-input focus-within:border-border-input-hover focus-within:shadow-date-field-focus hover:border-border-input-hover"
    />
    <PinInput.Input
            class="flex h-input w-10 select-none rounded-input border border-foreground bg-grey-100 dark:bg-slate-800 px-2 py-3 text-center font-alt text-[17px] tracking-[0.01em] text-foreground placeholder-shown:border-border-input focus-within:border-border-input-hover focus-within:shadow-date-field-focus hover:border-border-input-hover"
    />
    <PinInput.Input
            class="flex h-input w-10 select-none rounded-input border border-foreground bg-grey-100 dark:bg-slate-800 px-2 py-3 text-center font-alt text-[17px] tracking-[0.01em] text-foreground placeholder-shown:border-border-input focus-within:border-border-input-hover focus-within:shadow-date-field-focus hover:border-border-input-hover"
    />
    <PinInput.Input
            class="flex h-input w-10 select-none rounded-input border border-foreground bg-grey-100 dark:bg-slate-800 px-2 py-3 text-center font-alt text-[17px] tracking-[0.01em] text-foreground placeholder-shown:border-border-input focus-within:border-border-input-hover focus-within:shadow-date-field-focus hover:border-border-input-hover"
    />
    <PinInput.Input
            class="flex h-input w-10 select-none rounded-input border border-foreground bg-grey-100 dark:bg-slate-800 px-2 py-3 text-center font-alt text-[17px] tracking-[0.01em] text-foreground placeholder-shown:border-border-input focus-within:border-border-input-hover focus-within:shadow-date-field-focus hover:border-border-input-hover"
    />
    <PinInput.Input
            class="flex h-input w-10 select-none rounded-input border border-foreground bg-grey-100 dark:bg-slate-800 px-2 py-3 text-center font-alt text-[17px] tracking-[0.01em] text-foreground placeholder-shown:border-border-input focus-within:border-border-input-hover focus-within:shadow-date-field-focus hover:border-border-input-hover"
    />
    <PinInput.HiddenInput />
</PinInput.Root>