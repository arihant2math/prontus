<script>
    import ProfilePicture from "./ProfilePicture.svelte";
    import { Popover, Separator, Toggle } from "bits-ui";
    import UserInfo from "./UserInfo.svelte";

    export let user;
    export let id;
</script>

<Popover.Root>
    <Popover.Trigger>
        <ProfilePicture user="{user}"/>
    </Popover.Trigger>
<!--    For popover content-->
    <!--            transition={flyAndScale}-->
    <Popover.Content
            class="z-50 w-full max-w-max rounded-[12px] border border-dark-10 bg-white dark:bg-slate-800 shadow-popover p-4"
            sideOffset={8}>
        <UserInfo user={user}/>
    </Popover.Content>
</Popover.Root>