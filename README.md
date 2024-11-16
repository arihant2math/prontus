# Prontus

An alpha client for Pronto (https://pronto.io), a messaging application used by educational institutions.

**Current Progress:** https://github.com/arihant2math/prontus/issues/1

## Installation

At the moment, there are prebuilt alpha binaries.

To find the latest one, head to the [release page](https://github.com/arihant2math/prontus/releases)

### Windows

> [!NOTE]
> ARM is not built at the moment due to GitHub CI limitations. If you are on ARM, you can build from source.

Download the `prontus_x.y.z_x64_en-US.msi` file.
Using a msi allows prontus notifications to look like they don't come from PowerShell.
If that is a non-issue, there is a portable exe and a .exe setup installer.

### Linux

There are app image, .deb, and .rpm files available for installation.

### MacOS

> [!WARNING]
> At the moment the `prontus_0.1.0_aarch64.dmg` will not install due to the fact is not signed.
> Apple Silicon macOS users should use `prontus_0.1.0_x64.dmg` for the time being.

As the app is not signed, you will have to go through some hoops with gatekeeper to install it.

## Advantages

The official Pronto client is bloated (taking up over 300 mb of RAM), leaks memory, and has poor user interface design.
Prontus aims to fix this by rewriting the Pronto client from the ground up.

### Performance

Prontus aims to be faster and more lightweight than the official Pronto client.
To achieve this, Prontus uses tauri,
which allows it to leverage rust which allows the app to execute background tasks in a performant manner.
The Prontus frontend uses svelte kit, allowing it to keep a relatively low memory footprint.
Tauri also helps reduce the size of the application by using the builtin OS web renderer,
rather than bundling an entire chrome installation like the official Pronto desktop app.

### More Features

Prontus aims to have the following features the official Pronto client does not.
These might not all be implemented before the stable release, but they are the current overarching goals.

- [x] Rich Text (Prontus support markdown and latex, with builtin styling commands in the message box)
- [x] UI Customizability (Pronto has none, so Prontus is already better in many regards, but it aims to go even further,
  including custom themes and rearrangement of UI elements) https://github.com/arihant2math/prontus/issues/51
- [ ] Better search via local message storage. (Already implemented in the backend, but the UI is not completed)
- [ ] Encrypted DMs (Partially implemented, but without any settings or hook to the
  UI) https://github.com/arihant2math/prontus/issues/44
- [ ] Scripting Support (Work has already started with probot, a bot scripting crate)
- [ ] Extensibility via plugins (Using WASM)

## Screenshots

![screenshot.png](screenshots/screenshot.png)
![login.png](screenshots/login.png)
