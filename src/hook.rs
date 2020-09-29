#[cfg(feature = "example")]
pub(crate) fn install_hook() {
    unsafe {
        winapi::um::shellscalingapi::SetProcessDpiAwareness(
            winapi::um::shellscalingapi::PROCESS_PER_MONITOR_DPI_AWARE,
        );
    }
}

#[cfg(not(feature = "example"))]
pub(crate) fn install_hook() {}
