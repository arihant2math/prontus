<script>
    import Code from "./Code.svelte";
    import TopLink from "./TopLink.svelte";
    import {loadTheme} from "$lib/helpers.ts";

    /** @type {{onCode: any}} */
    let { onCode } = $props();
    let password = $state([]);

    async function submit() {
        console.log(password.join(""));
        await onCode(password.join(""));
    }

    async function trySubmit() {
        if (password.length === 6) {
            await submit();
        }
    }

    loadTheme();
</script>

<section class="bg-gray-50 dark:bg-gray-900">
    <div class="flex flex-col items-center justify-center px-6 py-8 mx-auto md:h-screen lg:py-0">
        <TopLink/>
        <div class="w-full bg-white rounded-lg shadow dark:border md:mt-0 sm:max-w-md xl:p-0 dark:bg-gray-800 dark:border-gray-700">
            <div class="p-6 space-y-4 md:space-y-6 sm:p-8">
                <h1 class="text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl dark:text-white">
                    Sign in to your account
                </h1>
                <form class="space-y-4 md:space-y-6" action="#">
                    <div>
                        <label for="password" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Code</label>
                        <Code bind:value={password} onchange={trySubmit}/>
                    </div>
                    <button type="button" onclick={submit} class="w-full text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800">Sign in</button>
                    <p class="text-sm font-light text-gray-500 dark:text-gray-400">
                        Pronto has emailed you a one-time 6-digit code to use. It may take a few minutes to arrive.
                    </p>
                </form>
            </div>
        </div>
    </div>
</section>