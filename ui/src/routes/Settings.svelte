<!--TODO: Add ability to log out-->
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
<script>
    import RadioLabel from "./settingsComponents/RadioLabel.svelte";
    import OptionsLabel from "./settingsComponents/options/OptionsLabel.svelte";
    import {getSettings, setSettings} from "$lib/api.ts";
    import {loadTheme} from "$lib/helpers.ts";

    export let settings;

    function loadSettings() {
        loadTheme(settings);
        getSettings().then((newSettings) => {
            console.log(newSettings);
            settings = newSettings;
        });
    }

    function saveSettings() {
        console.log(settings);
        setSettings(settings);
        loadTheme(settings);
    }

    function logout() {
        settings.auth.api_key = null;
        saveSettings();
    }

    loadTheme(settings);
</script>

<div id="settings-modal" tabindex="-1" aria-hidden="true" class="hidden overflow-y-auto overflow-x-hidden fixed top-1/2 right-1/2 z-50 justify-center items-center w-full md:inset-0 h-full">
    <div class="relative p-4 w-full h-full">
        <!-- Modal content -->
        <div class="relative bg-white rounded-lg shadow dark:bg-slate-800">
            <!-- Modal header -->
            <div class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-gray-600">
                <h3 class="text-2xl font-semibold text-gray-900 dark:text-white">
                    Settings
                </h3>
                <button type="button" class="end-2.5 text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center dark:hover:bg-slate-600 dark:hover:text-white" on:click={hideSettings}>
                    <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
                    </svg>
                    <span class="sr-only">Close modal</span>
                </button>
            </div>
            <!-- Modal body -->
            <div class="p-4 md:p-5 w-full">
                <form class="space-y-4">
                    <div>
                        <h3 class="text-xl font-semibold text-gray-900 dark:text-white my-2">
                            Appearance
                        </h3>
                        <div>
                            <div>
                                <h4 class="text-lg font-semibold text-gray-900 dark:text-white my-2">
                                    Theme
                                </h4>
                                <ul class="space-y-4 mb-4 max-w-lg">
                                    <li>
                                        <input type="radio" id="settings-theme-automatic" name="theme" value="Auto" class="hidden peer" bind:group={settings.appearance.theme} on:change={saveSettings}/>
                                        <RadioLabel target="settings-theme-automatic">
                                            <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M9.53 16.122a3 3 0 0 0-5.78 1.128 2.25 2.25 0 0 1-2.4 2.245 4.5 4.5 0 0 0 8.4-2.245c0-.399-.078-.78-.22-1.128Zm0 0a15.998 15.998 0 0 0 3.388-1.62m-5.043-.025a15.994 15.994 0 0 1 1.622-3.395m3.42 3.42a15.995 15.995 0 0 0 4.764-4.648l3.876-5.814a1.151 1.151 0 0 0-1.597-1.597L14.146 6.32a15.996 15.996 0 0 0-4.649 4.763m3.42 3.42a6.776 6.776 0 0 0-3.42-3.42" />
                                            </svg>
                                            <p slot="text">Automatic</p>
                                        </RadioLabel>
                                    </li>
                                    <li>
                                        <input type="radio" id="settings-theme-light" name="theme" value="Light" class="hidden peer" bind:group={settings.appearance.theme} on:change={saveSettings}>
                                        <RadioLabel target="settings-theme-light">
                                            <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 mt-1">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z" />
                                            </svg>
                                            <p slot="text">Light</p>

                                        </RadioLabel>
                                    </li>
                                    <li>
                                        <input type="radio" id="settings-theme-dark" name="theme" value="Dark" class="hidden peer" bind:group={settings.appearance.theme} on:change={saveSettings}>
                                        <RadioLabel target="settings-theme-dark">
                                            <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 mt-1">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z" />
                                            </svg>
                                            <p slot="text">Dark</p>
                                        </RadioLabel>
                                    </li>
                                </ul>
                            </div>
                            <div>
                                <h4 class="text-lg font-semibold text-gray-900 dark:text-white my-2">
                                    Sidebar
                                </h4>
                                <div>
                                    <ul class="space-y-4 mb-4 max-w-lg">
                                        <li>
                                            <input type="radio" id="settings-appearance-sidebar-all" name="theme" value="All" class="hidden peer" bind:group={settings.appearance.sidebar.category_display_level} on:change={saveSettings}/>
                                            <RadioLabel target="settings-appearance-sidebar-all">
                                                <p slot="text">All</p>
                                            </RadioLabel>
                                        </li>
                                        <li>
                                            <input type="radio" id="settings-appearance-sidebar-non-singletons" name="theme" value="NonSingleton" class="hidden peer" bind:group={settings.appearance.sidebar.category_display_level} on:change={saveSettings}>
                                            <RadioLabel target="settings-appearance-sidebar-non-singletons">
                                                <p slot="text">Non-singletons</p>

                                            </RadioLabel>
                                        </li>
                                        <li>
                                            <input type="radio" id="settings-appearance-sidebar-none" name="theme" value="None" class="hidden peer" bind:group={settings.appearance.sidebar.category_display_level} on:change={saveSettings}>
                                            <RadioLabel target="settings-appearance-sidebar-none">
                                                <p slot="text">None</p>
                                            </RadioLabel>
                                        </li>
                                    </ul>
                                    <ul class="grid w-full gap-6 md:grid-cols-2 lg:grid-cols-3 max-w-xl">
                                        <li>
                                            <input type="checkbox" id="compact-option" value="" class="hidden peer" bind:checked={settings.appearance.messages.compact} on:change={saveSettings}>
                                            <OptionsLabel target="compact-option">
                                                <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mb-2 w-7 h-7 text-blue-500">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 13.5V3.75m0 9.75a1.5 1.5 0 0 1 0 3m0-3a1.5 1.5 0 0 0 0 3m0 3.75V16.5m12-3V3.75m0 9.75a1.5 1.5 0 0 1 0 3m0-3a1.5 1.5 0 0 0 0 3m0 3.75V16.5m-6-9V3.75m0 3.75a1.5 1.5 0 0 1 0 3m0-3a1.5 1.5 0 0 0 0 3m0 9.75V10.5" />
                                                </svg>

                                                <p slot="title">Compact Mode</p>
                                                <p slot="body">Compacts the messages by hiding profile pictures and removing information.</p>
                                            </OptionsLabel>
                                        </li>
                                        <li>
                                            <input type="checkbox" id="hide-embeds-option" value="" class="hidden peer" bind:checked={settings.appearance.messages.hide_embeds} on:change={saveSettings}>
                                            <OptionsLabel target="hide-embeds-option">
                                                <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mb-2 w-7 h-7 text-blue-500">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.98 8.223A10.477 10.477 0 0 0 1.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.451 10.451 0 0 1 12 4.5c4.756 0 8.773 3.162 10.065 7.498a10.522 10.522 0 0 1-4.293 5.774M6.228 6.228 3 3m3.228 3.228 3.65 3.65m7.894 7.894L21 21m-3.228-3.228-3.65-3.65m0 0a3 3 0 1 0-4.243-4.243m4.242 4.242L9.88 9.88" />
                                                </svg>

                                                <p slot="title">Hide Embeds</p>
                                                <p slot="body">Hides embeds, links will still exist.</p>
                                            </OptionsLabel>
                                        </li>
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div>
                        <h3 class="text-xl font-semibold text-gray-900 dark:text-white my-2">
                            Options
                        </h3>
                        <ul class="grid w-full gap-6 md:grid-cols-2 lg:grid-cols-3 max-w-xl">
                            <li>
                                <input type="checkbox" id="notifications-option" value="" class="hidden peer" bind:checked={settings.options.notifications} on:change={saveSettings}>
                                <OptionsLabel target="notifications-option">
                                    <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mb-2 w-7 h-7 text-yellow-500">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967 0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714 0" />
                                    </svg>
                                    <p slot="title">Notifications</p>
                                    <p slot="body">Sends OS notifications when there are new messages.</p>
                                </OptionsLabel>
                            </li>
                            <li>
                                <input type="checkbox" id="experiments-option" value="" class="hidden peer" bind:checked={settings.options.experiments} on:change={saveSettings}>
                                <OptionsLabel target="experiments-option">
                                    <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mb-2 w-7 h-7 text-blue-500">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M9.75 3.104v5.714a2.25 2.25 0 0 1-.659 1.591L5 14.5M9.75 3.104c-.251.023-.501.05-.75.082m.75-.082a24.301 24.301 0 0 1 4.5 0m0 0v5.714c0 .597.237 1.17.659 1.591L19.8 15.3M14.25 3.104c.251.023.501.05.75.082M19.8 15.3l-1.57.393A9.065 9.065 0 0 1 12 15a9.065 9.065 0 0 0-6.23-.693L5 14.5m14.8.8 1.402 1.402c1.232 1.232.65 3.318-1.067 3.611A48.309 48.309 0 0 1 12 21c-2.773 0-5.491-.235-8.135-.687-1.718-.293-2.3-2.379-1.067-3.61L5 14.5" />
                                    </svg>

                                    <p slot="title">Experiments</p>
                                    <p slot="body">Be the first to try out new features that could be unstable.</p>
                                </OptionsLabel>
                            </li>
                            <li>
                                <input type="checkbox" id="error-reporting-option" value="" class="hidden peer" bind:checked={settings.options.errorReporting} on:change={saveSettings}>
                                <OptionsLabel target="error-reporting-option">
                                    <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mb-2 w-7 h-7 text-red-500">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z" />
                                    </svg>

                                    <p slot="title">Error Reporting</p>
                                    <p slot="body">Automatically report errors via sentry.</p>
                                </OptionsLabel>
                            </li>
                            <li>
                                <input type="checkbox" id="analytics-option" value="" class="hidden peer" bind:checked={settings.options.analytics} on:change={saveSettings}>
                                <OptionsLabel target="analytics-option">
                                    <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mb-2 w-7 h-7 text-green-500">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 14.25v2.25m3-4.5v4.5m3-6.75v6.75m3-9v9M6 20.25h12A2.25 2.25 0 0 0 20.25 18V6A2.25 2.25 0 0 0 18 3.75H6A2.25 2.25 0 0 0 3.75 6v12A2.25 2.25 0 0 0 6 20.25Z" />
                                    </svg>

                                    <p slot="title">Analytics</p>
                                    <p slot="body">Gather analytics to help improve Prontus.</p>
                                </OptionsLabel>
                            </li>
                            <li>
                                <input type="checkbox" id="read-messages-option" value="" class="hidden peer" bind:checked={settings.options.read_messages} on:change={saveSettings}>
                                <OptionsLabel target="read-messages-option">
                                    <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mb-2 w-7 h-7 text-green-500">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 14.25v2.25m3-4.5v4.5m3-6.75v6.75m3-9v9M6 20.25h12A2.25 2.25 0 0 0 20.25 18V6A2.25 2.25 0 0 0 18 3.75H6A2.25 2.25 0 0 0 3.75 6v12A2.25 2.25 0 0 0 6 20.25Z" />
                                    </svg>

                                    <p slot="title">Read Messages</p>
                                    <p slot="body">When channels are opened, mark them as read.</p>
                                </OptionsLabel>
                            </li>
                        </ul>
                    </div>
                    <h3 class="text-xl font-semibold text-gray-900 dark:text-white my-2">
                        Account
                    </h3>
                    <button type="button" class="text-white bg-red-700 hover:bg-red-800 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-red-600 dark:hover:bg-red-700" on:click={logout}>
                        Logout
                    </button>
                </form>
            </div>
        </div>
    </div>
</div>
