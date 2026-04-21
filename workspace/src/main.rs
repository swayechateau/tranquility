fn main() -> std::process::ExitCode {
    // Reset SIGPIPE to default so piping to `head`, `less`, etc. exits cleanly rather than panicking.
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    tranquility::cli::run()
}
