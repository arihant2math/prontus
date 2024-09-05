<!--TODO: Add ability to log out and set theme-->
<!--TODO: should this be a modal or a page?-->
<script context="module">
    export function hideSettings() {
        let modal = document.getElementById("settings-modal");
        modal.classList.add("hidden");
        modal.setAttribute("aria-hidden", "true");
    }

    export function showSettings() {
        let modal = document.getElementById("settings-modal");
        modal.classList.remove("hidden");
        modal.setAttribute("aria-hidden", "false");
    }
</script>

<div id="settings-modal" tabindex="-1" aria-hidden="true" class="hidden overflow-y-auto overflow-x-hidden fixed top-1/2 right-1/2 z-50 justify-center items-center w-full md:inset-0 h-full">
    <div class="relative p-4 w-full h-full">
        <!-- Modal content -->
        <div class="relative bg-white rounded-lg shadow dark:bg-gray-700">
            <!-- Modal header -->
            <div class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-gray-600">
                <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
                    Settings
                </h3>
                <button type="button" class="end-2.5 text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white" on:click={hideSettings}>
                    <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
                    </svg>
                    <span class="sr-only">Close modal</span>
                </button>
            </div>
            <!-- Modal body -->
            <div class="p-4 md:p-5 max-w-lg">
                <form class="space-y-4">
                    <h4 class="text-lg font-semibold text-gray-900 dark:text-white">
                        Theme
                    </h4>
                    <ul class="space-y-4 mb-4">
                        <li>
                            <input type="radio" id="settings-theme-automatic" name="theme" value="automatic" class="hidden peer" checked/>
                            <label for="settings-theme-automatic"
                                   class="inline-flex items-center justify-between w-full p-5 text-gray-900 bg-white border border-gray-200 rounded-lg cursor-pointer dark:hover:text-gray-300 dark:border-gray-500 dark:peer-checked:text-blue-500 peer-checked:border-blue-600 peer-checked:text-blue-600 hover:text-gray-900 hover:bg-gray-100 dark:text-white dark:bg-gray-600 dark:hover:bg-gray-500">
                                <div class="block">
                                    <div class="w-full text-lg font-semibold">Automatic</div>
                                </div>
                            </label>
                        </li>
                        <li>
                            <input type="radio" id="settings-theme-light" name="theme" value="light" class="hidden peer">
                            <label for="settings-theme-light" class="inline-flex items-center justify-between w-full p-5 text-gray-900 bg-white border border-gray-200 rounded-lg cursor-pointer dark:hover:text-gray-300 dark:border-gray-500 dark:peer-checked:text-blue-500 peer-checked:border-blue-600 peer-checked:text-blue-600 hover:text-gray-900 hover:bg-gray-100 dark:text-white dark:bg-gray-600 dark:hover:bg-gray-500">
                                <div class="block">
                                    <div class="w-full text-lg font-semibold">Light</div>
                                </div>
                            </label>
                        </li>
                        <li>
                            <input type="radio" id="settings-theme-dark" name="theme" value="right" class="hidden peer">
                            <label for="settings-theme-dark" class="inline-flex items-center justify-between w-full p-5 text-gray-900 bg-white border border-gray-200 rounded-lg cursor-pointer dark:hover:text-gray-300 dark:border-gray-500 dark:peer-checked:text-blue-500 peer-checked:border-blue-600 peer-checked:text-blue-600 hover:text-gray-900 hover:bg-gray-100 dark:text-white dark:bg-gray-600 dark:hover:bg-gray-500">
                                <div class="block">
                                    <div class="w-full text-lg font-semibold">Dark</div>
                                </div>
                            </label>
                        </li>
                    </ul>
                    <h4 class="text-lg font-semibold text-gray-900 dark:text-white">
                        Experiments
                    </h4>
                    <ul class="grid w-full gap-6 md:grid-cols-3">
                        <li>
                            <input type="checkbox" id="notifications-option" value="" class="hidden peer">
                            <label for="notifications-option"
                                   class="inline-flex items-center justify-between w-full p-5 text-gray-500 bg-white border-2 border-gray-200 rounded-lg cursor-pointer dark:hover:text-gray-300 dark:border-gray-700 peer-checked:border-blue-600 hover:text-gray-600 dark:peer-checked:text-gray-300 peer-checked:text-gray-600 hover:bg-gray-50 dark:text-gray-400 dark:bg-gray-800 dark:hover:bg-gray-700">
                                <div class="block">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mb-2 w-7 h-7 text-yellow-500">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967 0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714 0" />
                                    </svg>
                                    <div class="w-full text-lg font-semibold">Enable Notifications</div>
                                    <div class="w-full text-sm">Sends OS notifications when there are new messages.</div>
                                </div>
                            </label>
                        </li>
                    </ul>
                    <h4 class="text-lg font-semibold text-gray-900 dark:text-white">
                        Account
                    </h4>
                    <button type="button" class="text-white bg-red-700 hover:bg-red-800 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-red-600 dark:hover:bg-red-700">
                        Logout
                    </button>
                </form>
            </div>
        </div>
    </div>
</div>
