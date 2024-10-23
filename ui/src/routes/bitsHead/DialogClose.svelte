<script lang="ts">
    import {Dialog} from "bits-ui";
    import CloseButton from "../CloseButton.svelte";
</script>
<Dialog.Close
        class="absolute right-5 top-5 rounded-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-foreground focus-visible:ring-offset-2 focus-visible:ring-offset-background active:scale-98">
    <CloseButton/>
</Dialog.Close>