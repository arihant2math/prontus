<script lang="ts">
    import {Dialog} from "bits-ui";
</script>
<Dialog.Content class="fixed left-[50%] top-[50%] z-50 w-full max-w-[90%] translate-x-[-50%] translate-y-[-50%] rounded-lg bg-white dark:bg-slate-800 p-5 shadow-2xl outline-none">
    <slot></slot>
</Dialog.Content>