export function loadTheme(settings) {
    if (settings === undefined || settings === null) {
        if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
            document.documentElement.classList.add('dark')
        } else {
            document.documentElement.classList.remove('dark')
        }
        return;
    }
    if (settings.appearance.theme === 'Dark' || (settings.appearance.theme === 'Auto' && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
        document.documentElement.classList.add('dark')
    } else {
        document.documentElement.classList.remove('dark')
    }
}

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
