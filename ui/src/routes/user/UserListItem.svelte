<script>
    import ProfilePicture from "./ProfilePicture.svelte";
    import { Popover, Separator, Toggle } from "bits-ui";
    import UserInfo from "./UserInfo.svelte";

    /** @type {{user?: any}} */
    let {
        user = {
            id: 0,
            fullname: "User",
            profilepicurl: "https://cdn.discordapp.com/embed/avatars/0.png",
            organizations: [
                {
                    name: "Organization"
                }
            ]
        },
        onCreateDm
    } = $props();
</script>

<Popover.Root class="w-full">
    <Popover.Trigger class="w-full">
        <div class="flex flex-row space-x-4 px-4 py-2 w-full border-b border-gray-500 z-40">
            <ProfilePicture user={user}/>
            <p class="text-sm w-max text-ellipsis">{user.fullname}</p>
        </div>
    </Popover.Trigger>
<!--    For popover content-->
    <!--            transition={flyAndScale}-->
    <Popover.Content
            class="z-50 w-full max-w-max rounded-lg bg-white dark:bg-slate-800 shadow-md p-4"
            sideOffset={8}>
        <UserInfo user={user} onCreateDm={onCreateDm}/>
    </Popover.Content>
</Popover.Root>