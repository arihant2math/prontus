<script>
    import "../app.css";
    import Main from "./Main.svelte";
    import AuthCode from "./auth/AuthCode.svelte"
    import AuthEmail from "./auth/AuthEmail.svelte";
    import {getCode, sendCode, load} from "$lib/api.ts";
    import {initPopovers} from "$lib/popup.js";

    initPopovers();

    let pages = [Main, AuthEmail, AuthCode]

    let page;
    let savedEmail;

    async function init() {
        await load().then(() => {
            console.log("User is authenticated");
            page = 0
        }, (e) => {
            if (e.toString() === "The user is not authenticated") {
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
        await sendCode(savedEmail, code);
        await load();
        page = 0;
        onCodeLock = false;
    }

    init();
</script>

<svelte:component
        this={pages[page]}
        onEmail={onEmail}
        onCode={onCode}
/>
