fn main() -> std::process::ExitCode {
    // Reset SIGPIPE to default so piping to `head`, `less`, etc. exits cleanly rather than panicking.
    // This is the crate's only unsafe code; the release build otherwise denies `unsafe_code`.
    #[cfg(unix)]
    #[allow(unsafe_code)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    tranquility::cli::run()
}
