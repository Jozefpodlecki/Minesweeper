const dark = localStorage.getItem("dark");
const prefersDarkMode = window.matchMedia("(prefers-color-scheme: dark)").matches;
const useDarkMode = dark === "true" || (dark == null && prefersDarkMode);
document.body.classList.toggle("dark", useDarkMode);

function showFatalError(event) {
    window.removeEventListener("error", showFatalError);

    const error = event.error || event.message;
    console.error("Fatal error:", error);

    const errorNode = document.getElementById("error");
    const defaultBackground = document.getElementById("default-background");
    const rootNode = document.getElementById("root");

    if (rootNode) root.style.display = "none";
    if (errorNode) errorNode.style.display = "flex";
    if (defaultBackground) defaultBackground.style.opacity = "1";
}

window.addEventListener("error", showFatalError);