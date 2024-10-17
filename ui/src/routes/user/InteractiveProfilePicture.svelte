<script>
    import ProfilePicture from "./ProfilePicture.svelte";
    import { Popover, Separator, Toggle } from "bits-ui";
    import UserInfo from "./UserInfo.svelte";

    export let user;
    export let id;
</script>

<Popover.Root>
    <Popover.Trigger class="w-8 h-8">
        <ProfilePicture user="{user}"/>
    </Popover.Trigger>
<!--    For popover content-->
    <!--            transition={flyAndScale}-->
    <Popover.Content
            class="z-50 w-full max-w-max rounded-[12px] bg-white dark:bg-slate-800 shadow-lg p-4"
            sideOffset={8}>
        <UserInfo user={user} on:createDm/>
    </Popover.Content>
</Popover.Root>