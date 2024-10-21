# Prontus

An alpha client for Pronto (https://pronto.io), a messaging application used by educational institutions.

**Current Progress:** https://github.com/arihant2math/prontus/issues/1

## Advantages
The offical pronto client is bloated (taking up over 300 mb of RAM), leaks memory, and has poor user interface design. Prontus aims to fix this by rewriting the pronto client from the ground up.

### Performance
Prontus aims to be faster and more lightweight than the offical Pronto client. To accomplish this, Prontus uses tauri, which allows it to leverage rust which allows the app to execute background tasks in a performant manner. The Prontus frontend utilizes sveltekit, allowing it to keep a relatively low memory footprint. Tauri also helps reduce the size of the application by using the builtin OS web renderer, rather than bundling an entire chrome instalation like the offical Pronto desktop app.  

### More Features
Prontus aims to have the following features the offical Pronto client does not. These might not all be implmeneted before the stable release, but they are goals.

- [x] Rich Text (Prontus support markdown and latex, with builtin styling commands in the message box)
- [x] UI Customizability (Pronto has none, so Prontus is already better in many regards, but it aims to go even further, including custom themes and rearrangement of UI elements)
- [ ] Better search via local message storage. (Already implemented in the backend, but the UI is not completed)
- [ ] Encrypted DMs (Partially implemented, but without any settings or hook to the UI)
- [ ] Scripting Support (Work has already started with probot, a bot scripting crate)
- [ ] Extensibility via plugins (Using WASM)

 

## Screenshots
![screenshot.png](screenshots/screenshot.png)
![login.png](screenshots/login.png)
