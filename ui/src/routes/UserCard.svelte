<script>
    import ProfilePicture from "./ProfilePicture.svelte";

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

<div class="flex flex-row space-x-4 px-4 py-2 w-full border-b border-gray-500 z-40">
    <div>
        <ProfilePicture user={user} />
    </div>
    <div class="flex flex-col">
        <p class="text-sm text-nowrap">{user.fullname}</p>
    </div>
</div>