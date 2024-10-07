# Prontus

An alpha client for Pronto (https://pronto.io), a messaging application used by educational institutions.

**Current Progress:** https://github.com/arihant2math/prontus/issues/1

## Advantages
The offical pronto client is bloated (taking up over 300 mb of RAM), leaks memory, and has poor user interface design. Prontus aims to fix this by rewriting the pronto client from the ground up. To accomplish this, Prontus uses tauri, which allows it to leverage rust which allows the app to be fast and performant. The Prontus frontend utilizes sveltekit, allowing it to keep a relatively low memory footprint. Tauri also helps reduce the size of the application by using the builtin OS web renderer, rather than bundling an entire chrome instalation like the offical Pronto desktop app.  

## Screenshots
![screenshot.png](screenshots/screenshot.png)
![login.png](screenshots/login.png)
