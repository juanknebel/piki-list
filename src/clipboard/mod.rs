//! Clipboard operations using arboard with platform-specific fallbacks
use arboard::Clipboard;
use std::io;
use std::io::Write;
use std::process::{Command, Stdio};

/// Copy text to the system clipboard
///
/// # Arguments
/// * `clipboard` - Optional persistent clipboard instance
/// * `text` - The text to copy
pub fn copy_to_clipboard(clipboard: Option<&mut Clipboard>, text: &str) -> Result<(), io::Error> {
    let result = match clipboard {
        Some(cb) => cb
            .set_text(text)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to copy: {}", e))),
        None => copy_with_arboard(text),
    };

    if let Err(primary_err) = result {
        // Try platform-specific fallback if arboard is unavailable
        copy_with_platform_tool(text).map_err(|fallback_err| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("{}; fallback failed: {}", primary_err, fallback_err),
            )
        })?;
    }

    Ok(())
}

/// Get text from the system clipboard
///
/// # Arguments
/// * `clipboard` - Optional persistent clipboard instance
pub fn get_from_clipboard(clipboard: Option<&mut Clipboard>) -> Result<String, io::Error> {
    let result = match clipboard {
        Some(cb) => cb
            .get_text()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to paste: {}", e))),
        None => paste_with_arboard(),
    };

    match result {
        Ok(text) => Ok(text),
        Err(primary_err) => paste_with_platform_tool().map_err(|fallback_err| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("{}; fallback failed: {}", primary_err, fallback_err),
            )
        }),
    }
}

fn copy_with_arboard(text: &str) -> Result<(), io::Error> {
    let mut clipboard = Clipboard::new().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to initialize clipboard: {}", e),
        )
    })?;

    clipboard
        .set_text(text)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to copy: {}", e)))?;

    Ok(())
}

fn paste_with_arboard() -> Result<String, io::Error> {
    let mut clipboard = Clipboard::new().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to initialize clipboard: {}", e),
        )
    })?;

    clipboard
        .get_text()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to paste: {}", e)))
}

#[cfg(target_os = "macos")]
fn copy_with_platform_tool(text: &str) -> Result<(), io::Error> {
    run_copy_command("pbcopy", &[], text)
}

#[cfg(target_os = "macos")]
fn paste_with_platform_tool() -> Result<String, io::Error> {
    run_paste_command("pbpaste", &[])
}

#[cfg(target_os = "linux")]
fn copy_with_platform_tool(text: &str) -> Result<(), io::Error> {
    let attempts: &[(&str, &[&str])] = &[
        ("wl-copy", &[]),
        ("xclip", &["-selection", "clipboard"]),
        ("xsel", &["--clipboard"]),
    ];

    let mut last_err: Option<io::Error> = None;
    for (cmd, args) in attempts {
        match run_copy_command(cmd, args, text) {
            Ok(()) => return Ok(()),
            Err(err) => last_err = Some(err),
        }
    }

    Err(last_err.unwrap_or_else(|| {
        io::Error::new(
            io::ErrorKind::Other,
            "No clipboard command available (tried wl-copy, xclip, xsel)",
        )
    }))
}

#[cfg(target_os = "linux")]
fn paste_with_platform_tool() -> Result<String, io::Error> {
    let attempts: &[(&str, &[&str])] = &[
        ("wl-paste", &["-n"]),
        ("xclip", &["-selection", "clipboard", "-o"]),
        ("xsel", &["--clipboard", "--output"]),
    ];

    let mut last_err: Option<io::Error> = None;
    for (cmd, args) in attempts {
        match run_paste_command(cmd, args) {
            Ok(text) => return Ok(text),
            Err(err) => last_err = Some(err),
        }
    }

    Err(last_err.unwrap_or_else(|| {
        io::Error::new(
            io::ErrorKind::Other,
            "No clipboard command available (tried wl-paste, xclip, xsel)",
        )
    }))
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn copy_with_platform_tool(_text: &str) -> Result<(), io::Error> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        "Clipboard fallback not supported on this platform",
    ))
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn paste_with_platform_tool() -> Result<String, io::Error> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        "Clipboard fallback not supported on this platform",
    ))
}

fn run_copy_command(cmd: &str, args: &[&str], text: &str) -> Result<(), io::Error> {
    let mut child = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{} failed: {}", cmd, e)))?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(text.as_bytes()).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("{} stdin failed: {}", cmd, e))
        })?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("{}: stdin not available", cmd),
        ));
    }

    let status = child
        .wait()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{} wait failed: {}", cmd, e)))?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("{} exited with status {}", cmd, status),
        ))
    }
}

fn run_paste_command(cmd: &str, args: &[&str]) -> Result<String, io::Error> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{} failed: {}", cmd, e)))?;

    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("{} output was not UTF-8: {}", cmd, e),
            )
        })
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("{} exited with status {}", cmd, output.status),
        ))
    }
}
