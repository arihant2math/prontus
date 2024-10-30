<script>
    /** @type {{text: any, style?: string, disabled?: boolean, compact?: boolean}} */
    let {
        style = "default",
        disabled = false,
        compact = false,
        ...props
    } = $props();

    function getColorClasses(buttonStyle, disabled) {
        if (disabled) {
            switch (buttonStyle) {
                case "danger":
                    return "bg-red-400 dark:bg-red-500";
                case "success":
                    return "bg-green-400 dark:bg-green-500";
                case "warning":
                    return "bg-yellow-400 dark:bg-yellow-500";
                default:
                    return "bg-blue-400 dark:bg-blue-500";
            }
        } else {
            switch (buttonStyle) {
                case "danger":
                    return "bg-red-700 hover:bg-red-800 dark:bg-red-600 dark:hover:bg-red-700";
                case "success":
                    return "bg-green-700 hover:bg-green-800 dark:bg-green-600 dark:hover:bg-green-700";
                case "warning":
                    return "bg-yellow-700 hover:bg-yellow-800 dark:bg-yellow-600 dark:hover:bg-yellow-700";
                default:
                    return "bg-blue-700 hover:bg-blue-800 dark:bg-blue-600 dark:hover:bg-blue-700";
            }
        }
    }
    let colorCls = $derived(getColorClasses(style, disabled));
    let spacing = $derived(compact ? "p-1" : "px-5 py-2.5 me-2 mb-2");
</script>
<button type="button" class="text-white font-medium rounded-lg text-sm {spacing} {colorCls}" {...props}><slot></slot></button>
