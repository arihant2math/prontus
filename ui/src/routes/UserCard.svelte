<script>
    import ProfilePicture from "./ProfilePicture.svelte";
    import { Popover, Separator, Toggle } from "bits-ui";
    import UserInfo from "./UserInfo.svelte";

    export let user = {
        id: 0,
        fullname: "User",
        profilepicurl: "https://cdn.discordapp.com/embed/avatars/0.png",
        organizations: [
            {
                name: "Organization"
            }
        ]
    };
</script>

<Popover.Root>
    <Popover.Trigger>
        <div class="flex flex-row space-x-4 px-4 py-2 w-full border-b border-gray-500 z-40">
            <div class="w-max">
                <ProfilePicture user={user}/>
            </div>
            <div class="flex flex-col">
                <p class="text-sm text-nowrap">{user.fullname}</p>
            </div>
        </div>
    </Popover.Trigger>
<!--    For popover content-->
    <!--            transition={flyAndScale}-->
    <Popover.Content
            class="z-50 w-full max-w-[328px] rounded-[12px] border border-dark-10 bg-white dark:bg-slate-800 shadow-popover p-4"
            sideOffset={8}>
        <UserInfo user={user}/>
    </Popover.Content>
</Popover.Root>