const isTauriAvailable = function(){
    return typeof window.__TAURI_IPC__ == 'function'
}

// Tauri window label ("main" = overlay, "control" = control panel).
// In a plain browser (vite dev without Tauri) use ?view=control to preview the control panel.
const getWindowLabel = function(){
    if(isTauriAvailable()){
        return window.__TAURI_METADATA__.__currentWindow.label
    }
    return new URLSearchParams(window.location.search).get('view') == 'control' ? 'control' : 'main'
}

export { isTauriAvailable, getWindowLabel }
