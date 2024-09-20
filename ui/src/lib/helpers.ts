export function loadTheme(settings) {
    if (settings.appearance.theme === 'Dark' || (settings.appearance.theme === 'Auto' && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
        document.documentElement.classList.add('dark')
    } else {
        document.documentElement.classList.remove('dark')
    }
}