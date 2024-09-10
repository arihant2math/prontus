<script>
    import "../app.css";
    import Main from "./main.svelte";
    import AuthCode from "./auth_code.svelte"
    import AuthEmail from "./auth_email.svelte";
    import {getCode, sendCode, load} from "./api.js";
    import {initPopovers} from "./popup.js";

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
