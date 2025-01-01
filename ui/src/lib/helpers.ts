export function getThemeClass(settings) {
    if (settings === undefined || settings === null) {
        if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
            return 'dark'
        } else {
            return 'light'
        }
    }
    if (settings.appearance.theme === 'Dark' || (settings.appearance.theme === 'Auto' && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
        return 'dark'
    } else {
        return 'light'
    }
}

export function loadTheme(settings) {
    // Auto theme by default
    let theme = 'Auto';
    if (!(settings === undefined || settings === null)) {
        theme = settings.appearance.theme;
    }

    // Add dark mode class if theme is dark or user prefers dark mode
    if (theme === 'Dark' || (theme === 'Auto' && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
        document.documentElement.classList.add('dark')
    } else {
        document.documentElement.classList.remove('dark')
    }

    // Handle color scheme changes iff settings appearance is auto
    if (settings.appearance.theme === 'Auto') {
        window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', event => {
            if (event.matches) {
                document.documentElement.classList.add('dark')
            } else {
                document.documentElement.classList.remove('dark')
            }
        });
    }
}

/// Parse datetimes from Pronto servers
export function parseDatetime(str: string): Date {
    // Split the date and time parts
    const [datePart, timePart] = str.split(' ');
    if (timePart === undefined) {
        return new Date(datePart);
    }
    // Combine the date and time parts with 'T' to conform to ISO 8601 format
    const isoString = `${datePart}T${timePart}Z`;

    // Return the Date object
    return new Date(isoString);
}
