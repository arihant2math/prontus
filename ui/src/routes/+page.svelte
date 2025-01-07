<script>
    import "../app.css";
    import Main from "./Main.svelte";
    import AuthCode from "./auth/AuthCode.svelte"
    import AuthEmail from "./auth/AuthEmail.svelte";
    import {getCode, sendCode, load} from "$lib/api.ts";
    import {initPopovers} from "$lib/popup.js";

    initPopovers();

    let pages = [Main, AuthEmail, AuthCode]

    let page = $state();
    let savedEmail;

    async function init() {
        await load().then(() => {
            console.log("User is authenticated");
            page = 0
        }, (e) => {
            // TODO: smh text matching
            if (e.toString() === "The user is not authenticated" || e.toString() === "Response error: API error: UNAUTHORIZED") {
                page = 1
            } else {
                console.error(e);
            }
        });
    }

    let onEmailLock = false;

    async function onEmail(email) {
        if (onEmailLock) {
            return;
        }
        onEmailLock = true;
        savedEmail = email;
        await getCode(email);
        page = 2;
        onEmailLock = false;
    }

    let onCodeLock = false;

    async function onCode(code) {
        if (onCodeLock) {
            return;
        }
        onCodeLock = true;
        try {
            await sendCode(savedEmail, code);
            await load();
            page = 0;
            onCodeLock = false;
        } catch (e) {
            console.error(e);
            page = 1;
            onCodeLock = false;
        }
    }

    init();

    const SvelteComponent = $derived(pages[page]);
</script>

<SvelteComponent
        onEmail={onEmail}
        onCode={onCode}
/>
