<script>
    import InteractiveProfilePicture from "../user/InteractiveProfilePicture.svelte";
    import RichTextContainer from "../messageComponents/RichTextContainer.svelte";

    /** @type {{announcement: any}} */
    let { announcement } = $props();
</script>
<!--{JSON.stringify(announcement)}-->
<div class="flex flex-row space-x-2 py-2">
    <div>
        <InteractiveProfilePicture user={announcement.sender} on:createDm/>
    </div>
    <div class="flex flex-col">
        <div class="flex items-center space-x-2 rtl:space-x-reverse">
            <span class="text-sm font-semibold text-gray-900 dark:text-white text-nowrap">{announcement.sender.fullname}</span>
<!--            <span class="text-sm font-normal text-gray-500 dark:text-gray-400 text-nowrap">{announcement}</span>-->
        </div>
        <RichTextContainer message={announcement.announcement}/>
    </div>
</div>