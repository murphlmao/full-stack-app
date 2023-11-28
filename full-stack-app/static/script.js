// Function to toggle the theme
function toggleTheme() {
    if (document.documentElement.classList.contains('dark')) {
        document.documentElement.classList.remove('dark');
        localStorage.setItem('color-theme', 'light'); // Storing the theme choice in localStorage
    } else {
        document.documentElement.classList.add('dark');
        localStorage.setItem('color-theme', 'dark'); // Storing the theme choice in localStorage
    }
}

// Check for a saved theme in localStorage and apply it
if (localStorage.getItem('color-theme') === 'dark' ||
    (!('color-theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
    document.documentElement.classList.add('dark');
} else {
    document.documentElement.classList.remove('dark');
}
