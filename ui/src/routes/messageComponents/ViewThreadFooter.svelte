<script>
    /** @type {{onClick: any}} */
    let { onClick } = $props();
</script>

<button onclick={onClick} type="button" class="max-w-fit text-blue-500 dark:text-blue-400 bg-white hover:bg-gray-100 font-medium
                rounded-lg text-xs mx-2 px-2 py-2 me-2 mb-2 dark:bg-slate-900 dark:border-gray-600
                dark:hover:bg-gray-700 dark:hover:border-gray-600">View Thread</button>