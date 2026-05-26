<script>
    import { PinInput, Toggle } from "bits-ui";

    // TODO: Use bits-ui
    /** @type {{value?: any}} */
    let {value = $bindable([])} = $props();
    // TODO: onchange prop

</script>

<input bind:value>