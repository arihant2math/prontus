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
    import ThemeLabel from "./settingsComponents/theme/ThemeLabel.svelte";
    import OptionsLabel from "./settingsComponents/options/OptionsLabel.svelte";
    import {getSettings, setSettings} from "$lib/api.js";

    let settings = {
        appearance: {
            "theme": "Auto"
        },
        options: {
            "richText": false,
            "experiments": false,
            "notifications": false
        }
    };

    function loadSettings() {
        loadTheme();
        getSettings().then((newSettings) => {
            console.log(newSettings);
            settings = newSettings;
        });
    }

    function saveSettings() {
        setSettings(settings);
        loadTheme();
    }

    function loadTheme() {
        if (settings.appearance.theme === 'Dark' || (settings.appearance.theme === 'Auto' && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
            document.documentElement.classList.add('dark')
        } else {
            document.documentElement.classList.remove('dark')
        }
    }

    loadTheme();

    loadSettings();
</script>

<div id="settings-modal" tabindex="-1" aria-hidden="true" class="hidden overflow-y-auto overflow-x-hidden fixed top-1/2 right-1/2 z-50 justify-center items-center w-full md:inset-0 h-full">
    <div class="relative p-4 w-full h-full">
        <!-- Modal content -->
        <div class="relative bg-white rounded-lg shadow dark:bg-slate-800">
            <!-- Modal header -->
            <div class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-gray-600">
                <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
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
                    <h4 class="text-lg font-semibold text-gray-900 dark:text-white">
                        Theme
                    </h4>
                    <ul class="space-y-4 mb-4 max-w-lg">
                        <li>
                            <input type="radio" id="settings-theme-automatic" name="theme" value="Auto" class="hidden peer" bind:group={settings.appearance.theme} on:change={saveSettings}/>
                            <ThemeLabel target="settings-theme-automatic">
                                <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9.53 16.122a3 3 0 0 0-5.78 1.128 2.25 2.25 0 0 1-2.4 2.245 4.5 4.5 0 0 0 8.4-2.245c0-.399-.078-.78-.22-1.128Zm0 0a15.998 15.998 0 0 0 3.388-1.62m-5.043-.025a15.994 15.994 0 0 1 1.622-3.395m3.42 3.42a15.995 15.995 0 0 0 4.764-4.648l3.876-5.814a1.151 1.151 0 0 0-1.597-1.597L14.146 6.32a15.996 15.996 0 0 0-4.649 4.763m3.42 3.42a6.776 6.776 0 0 0-3.42-3.42" />
                                </svg>
                                <p slot="text">Automatic</p>
                            </ThemeLabel>
                        </li>
                        <li>
                            <input type="radio" id="settings-theme-light" name="theme" value="Light" class="hidden peer" bind:group={settings.appearance.theme} on:change={saveSettings}>
                            <ThemeLabel target="settings-theme-light">
                                <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 mt-1">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z" />
                                </svg>
                                <p slot="text">Light</p>

                            </ThemeLabel>
                        </li>
                        <li>
                            <input type="radio" id="settings-theme-dark" name="theme" value="Dark" class="hidden peer" bind:group={settings.appearance.theme} on:change={saveSettings}>
                            <ThemeLabel target="settings-theme-dark">
                                <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 mt-1">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z" />
                                </svg>
                                <p slot="text">Dark</p>
                            </ThemeLabel>
                        </li>
                    </ul>
                    <h4 class="text-lg font-semibold text-gray-900 dark:text-white">
                        Options
                    </h4>
                    <ul class="grid w-full gap-6 md:grid-cols-2 lg:grid-cols-3 max-w-xl">
                        <li>
                            <input type="checkbox" id="rich-text-option" value="" class="hidden peer" bind:group={settings.options.richText} on:change={saveSettings}>
                            <OptionsLabel target="rich-text-option">
                                <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mb-2 w-7 h-7 text-blue-500">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
                                </svg>
                                <p slot="title">Rich Text</p>
                                <p slot="body">Enabled rich text, which allows for <b>bold</b>, <i>italics</i>, links,
                                    and more.</p>
                            </OptionsLabel>
                        </li>
                        <li>
                            <input type="checkbox" id="notifications-option" value="" class="hidden peer" bind:group={settings.options.notifications} on:change={saveSettings}>
                            <OptionsLabel target="notifications-option">
                                <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mb-2 w-7 h-7 text-yellow-500">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967 0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714 0" />
                                </svg>
                                <p slot="title">Notifications</p>
                                <p slot="body">Sends OS notifications when there are new messages.</p>
                            </OptionsLabel>
                        </li>
                        <li>
                            <input type="checkbox" id="experiments-option" value="" class="hidden peer" bind:group={settings.options.experiments} on:change={saveSettings}>
                            <OptionsLabel target="experiments-option">
                                <svg slot="svg" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mb-2 w-7 h-7 text-blue-500">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9.75 3.104v5.714a2.25 2.25 0 0 1-.659 1.591L5 14.5M9.75 3.104c-.251.023-.501.05-.75.082m.75-.082a24.301 24.301 0 0 1 4.5 0m0 0v5.714c0 .597.237 1.17.659 1.591L19.8 15.3M14.25 3.104c.251.023.501.05.75.082M19.8 15.3l-1.57.393A9.065 9.065 0 0 1 12 15a9.065 9.065 0 0 0-6.23-.693L5 14.5m14.8.8 1.402 1.402c1.232 1.232.65 3.318-1.067 3.611A48.309 48.309 0 0 1 12 21c-2.773 0-5.491-.235-8.135-.687-1.718-.293-2.3-2.379-1.067-3.61L5 14.5" />
                                </svg>

                                <p slot="title">Experiments</p>
                                <p slot="body">Be the first to try out new features that could be unstable.</p>
                            </OptionsLabel>
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
