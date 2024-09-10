<script>
    import ProfilePicture from "../ProfilePicture.svelte";

    export let user;
    export let id;

    $: user = user;
    $: targetId = id + "MessageProfilePicture";
    $: popoutId = id + "MessageUserInfo";

    function handleClick() {
        let popover = document.querySelector(`[data-popover-target="${targetId}"]`);
        if (!popover.classList.contains("collapse")) {
            // TODO: update user details
        }
        popover.classList.toggle("opacity-0");
        popover.classList.toggle("collapse");
    }

    function hide() {
        let popover = document.querySelector(`[data-popover-target="${targetId}"]`);
        popover.classList.add("opacity-0");
        popover.classList.add("collapse");
    }

    document.body.addEventListener('click', function (event) {
        let popout = document.getElementById(popoutId);
        let actions = document.getElementById(targetId);
        if (popout !== null && !popout.classList.contains("opacity-0") && !popout.contains(event.target) && !actions.contains(event.target)) {
            hide();
        }
    });

    console.log(user);
</script>

<button id="{targetId}" on:click={handleClick}>
    <ProfilePicture user="{user}"/>
</button>

<div id="{popoutId}" data-popover data-popover-target="{targetId}" class="collapse absolute z-10 inline-block w-64 text-sm text-gray-500 transition-opacity duration-300 bg-white border border-gray-200 rounded-lg shadow-sm opacity-0 dark:text-gray-400 dark:bg-gray-800 dark:border-gray-600">
    <div class="p-3">
        <div class="flex items-center justify-between mb-2">
            <a href="#">
                <ProfilePicture user="{user}"/>
            </a>
            <div>
                <button type="button" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-xs px-3 py-1.5 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800" disabled>Message</button>
            </div>
        </div>
        <p class="text-base font-semibold leading-none text-gray-900 dark:text-white">
            {user.fullname}
        </p>
        <p class="mb-3 text-sm font-normal">
            {user.pronouns}
        </p>
        <ul class="flex text-sm">
            <li class="me-2">
                <a href="#" class="hover:underline">
                    <span class="font-semibold text-gray-900 dark:text-white">799</span>
                    <span>Following</span>
                </a>
            </li>
            <li>
                <a href="#" class="hover:underline">
                    <span class="font-semibold text-gray-900 dark:text-white">3,758</span>
                    <span>Followers</span>
                </a>
            </li>
        </ul>
    </div>
    <div data-popper-arrow></div>
</div>