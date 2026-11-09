//! A *real* shell for the Terminal app: the user's `$SHELL` on a pseudo-terminal, rendered through a vt100 screen.
//! Native only. On the web there are no processes, so the Terminal keeps its virtual-file-system shell there.
use egui::{Event, Key, Modifiers};

/// Translates one key press into the bytes a terminal program expects (xterm conventions).
pub fn key_bytes(key: Key, m: Modifiers, app_cursor: bool) -> Option<Vec<u8>> {
    let md = 1 + (m.shift as u8) + 2 * (m.alt as u8) + 4 * (m.ctrl as u8);
    let csi = |c: char| -> Vec<u8> {
        if md > 1 {
            format!("\x1b[1;{md}{c}").into_bytes()
        } else if app_cursor {
            format!("\x1bO{c}").into_bytes()
        } else {
            format!("\x1b[{c}").into_bytes()
        }
    };
    let tilde = |n: u8| -> Vec<u8> {
        if md > 1 {
            format!("\x1b[{n};{md}~").into_bytes()
        } else {
            format!("\x1b[{n}~").into_bytes()
        }
    };
    let ss3 = |c: char| -> Vec<u8> {
        if md > 1 {
            format!("\x1b[1;{md}{c}").into_bytes()
        } else {
            format!("\x1bO{c}").into_bytes()
        }
    };
    Some(match key {
        Key::Enter => {
            if m.alt {
                b"\x1b\r".to_vec()
            } else {
                b"\r".to_vec()
            }
        }
        Key::Backspace => {
            if m.ctrl {
                vec![0x08]
            } else if m.alt {
                b"\x1b\x7f".to_vec()
            } else {
                vec![0x7f]
            }
        }
        Key::Tab => {
            if m.shift {
                b"\x1b[Z".to_vec()
            } else {
                b"\t".to_vec()
            }
        }
        Key::Escape => vec![0x1b],
        Key::ArrowUp => csi('A'),
        Key::ArrowDown => csi('B'),
        Key::ArrowRight => csi('C'),
        Key::ArrowLeft => csi('D'),
        Key::Home => csi('H'),
        Key::End => csi('F'),
        Key::Insert => tilde(2),
        Key::Delete => tilde(3),
        Key::PageUp => tilde(5),
        Key::PageDown => tilde(6),
        Key::F1 => ss3('P'),
        Key::F2 => ss3('Q'),
        Key::F3 => ss3('R'),
        Key::F4 => ss3('S'),
        Key::F5 => tilde(15),
        Key::F6 => tilde(17),
        Key::F7 => tilde(18),
        Key::F8 => tilde(19),
        Key::F9 => tilde(20),
        Key::F10 => tilde(21),
        Key::F11 => tilde(23),
        Key::F12 => tilde(24),
        Key::Space if m.ctrl => vec![0],
        _ => {
            // Ctrl+letter → C0 control code, Alt+letter → ESC prefix
            let name = key.name();
            let mut it = name.chars();
            let (Some(ch), None) = (it.next(), it.next()) else { return None };
            let lower = ch.to_ascii_lowercase();
            if m.ctrl && lower.is_ascii_lowercase() {
                let code = lower as u8 - b'a' + 1;
                return Some(if m.alt { vec![0x1b, code] } else { vec![code] });
            }
            if m.ctrl {
                return match lower {
                    '[' => Some(vec![0x1b]),
                    '\\' => Some(vec![0x1c]),
                    ']' => Some(vec![0x1d]),
                    '/' => Some(vec![0x1f]),
                    _ => None,
                };
            }
            if m.alt && !m.command {
                let c = if m.shift { ch } else { lower };
                let mut v = vec![0x1b];
                v.extend(c.to_string().as_bytes());
                return Some(v);
            }
            return None;
        }
    })
}

/// Bytes for everything the user did this frame that should reach the shell.
pub fn input_bytes(events: &[Event], app_cursor: bool, bracketed: bool) -> Vec<u8> {
    let mut out = vec![];
    for e in events {
        match e {
            Event::Text(t) => out.extend(t.as_bytes()),
            Event::Paste(t) => {
                let clean = t.replace("\r\n", "\r").replace('\n', "\r");
                if bracketed {
                    out.extend(b"\x1b[200~");
                    out.extend(clean.replace("\x1b[201~", "").as_bytes());
                    out.extend(b"\x1b[201~");
                } else {
                    out.extend(clean.as_bytes());
                }
            }
            Event::Key { key, pressed: true, modifiers, .. } => {
                // clipboard chords are handled by the UI (Ctrl+Shift+C / V)
                if modifiers.ctrl && modifiers.shift && matches!(key, Key::C | Key::V) {
                    continue;
                }
                if modifiers.ctrl && *key == Key::V && !modifiers.shift {
                    continue;
                }
                if let Some(b) = key_bytes(*key, *modifiers, app_cursor) {
                    out.extend(b);
                }
            }
            _ => {}
        }
    }
    out
}

pub fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::Live;

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
    use std::io::{Read, Write};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};

    pub struct Live {
        pub parser: Arc<Mutex<vt100::Parser>>,
        writer: Box<dyn Write + Send>,
        master: Box<dyn MasterPty + Send>,
        child: Box<dyn Child + Send + Sync>,
        alive: Arc<AtomicBool>,
        pub size: (u16, u16),
        pub shell: String,
        pub pid: Option<u32>,
        pub sel: Option<((u16, u16), (u16, u16))>,
        pub scroll: usize,
    }

    impl Live {
        pub fn default_shell() -> String {
            std::env::var("SHELL").ok().filter(|s| std::path::Path::new(s).exists()).unwrap_or_else(|| if std::path::Path::new("/bin/bash").exists() { "/bin/bash".into() } else { "/bin/sh".into() })
        }

        pub fn spawn(ctx: egui::Context, cwd: Option<&std::path::Path>, cols: u16, rows: u16) -> Result<Live, String> {
            let (cols, rows) = (cols.max(10), rows.max(3));
            let pair = native_pty_system().openpty(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 }).map_err(|e| format!("cannot open a pseudo-terminal: {e}"))?;
            let shell = Self::default_shell();
            let mut cmd = CommandBuilder::new(&shell);
            cmd.env("TERM", "xterm-256color");
            cmd.env("COLORTERM", "truecolor");
            cmd.env("TERM_PROGRAM", "SmartTerminal");
            if let Some(d) = cwd.filter(|d| d.is_dir()).map(|d| d.to_path_buf()).or_else(dirs_home) {
                cmd.cwd(d);
            }
            let child = pair.slave.spawn_command(cmd).map_err(|e| format!("cannot start {shell}: {e}"))?;
            drop(pair.slave);
            let pid = child.process_id();
            let mut reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;
            let writer = pair.master.take_writer().map_err(|e| e.to_string())?;
            let parser = Arc::new(Mutex::new(vt100::Parser::new(rows, cols, 5000)));
            let alive = Arc::new(AtomicBool::new(true));
            let (p2, a2) = (parser.clone(), alive.clone());
            let reply = Arc::new(Mutex::new(None::<Box<dyn Write + Send>>));
            let _ = reply; // replies go through a second writer below
            let mut answer = pair.master.take_writer().ok();
            std::thread::Builder::new()
                .name("pty-reader".into())
                .spawn(move || {
                    let mut buf = [0u8; 16 * 1024];
                    loop {
                        match reader.read(&mut buf) {
                            Ok(0) | Err(_) => break,
                            Ok(n) => {
                                let chunk = &buf[..n];
                                let mut replies: Vec<u8> = vec![];
                                {
                                    let mut p = match p2.lock() {
                                        Ok(g) => g,
                                        Err(poisoned) => poisoned.into_inner(),
                                    };
                                    p.process(chunk);
                                    // programs that ask "where is the cursor?" / "what are you?" must get an answer or they stall
                                    if find(chunk, b"\x1b[6n") {
                                        let (r, c) = p.screen().cursor_position();
                                        replies.extend(format!("\x1b[{};{}R", r + 1, c + 1).as_bytes());
                                    }
                                    if find(chunk, b"\x1b[c") || find(chunk, b"\x1b[0c") {
                                        replies.extend(b"\x1b[?62;c");
                                    }
                                }
                                if !replies.is_empty() {
                                    if let Some(w) = answer.as_mut() {
                                        let _ = w.write_all(&replies);
                                        let _ = w.flush();
                                    }
                                }
                                ctx.request_repaint();
                            }
                        }
                    }
                    a2.store(false, Ordering::SeqCst);
                    ctx.request_repaint();
                })
                .map_err(|e| e.to_string())?;
            Ok(Live { parser, writer, master: pair.master, child, alive, size: (cols, rows), shell, pid, sel: None, scroll: 0 })
        }

        pub fn write(&mut self, bytes: &[u8]) {
            if bytes.is_empty() {
                return;
            }
            let _ = self.writer.write_all(bytes);
            let _ = self.writer.flush();
        }

        pub fn resize(&mut self, cols: u16, rows: u16) {
            let (cols, rows) = (cols.max(10), rows.max(3));
            if (cols, rows) == self.size {
                return;
            }
            self.size = (cols, rows);
            let _ = self.master.resize(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 });
            if let Ok(mut p) = self.parser.lock() {
                p.set_size(rows, cols);
            }
        }

        pub fn is_alive(&mut self) -> bool {
            if !self.alive.load(Ordering::SeqCst) {
                return false;
            }
            !matches!(self.child.try_wait(), Ok(Some(_)))
        }

        /// The shell's current directory, read from the kernel (Linux). Lets the sidebar follow `cd`.
        pub fn cwd(&self) -> Option<std::path::PathBuf> {
            #[cfg(target_os = "linux")]
            {
                let pid = self.pid?;
                std::fs::read_link(format!("/proc/{pid}/cwd")).ok()
            }
            #[cfg(not(target_os = "linux"))]
            {
                None
            }
        }

        pub fn kill(&mut self) {
            let _ = self.child.kill();
        }
    }

    impl Drop for Live {
        fn drop(&mut self) {
            let _ = self.child.kill();
        }
    }

    fn find(hay: &[u8], needle: &[u8]) -> bool {
        hay.windows(needle.len()).any(|w| w == needle)
    }

    fn dirs_home() -> Option<std::path::PathBuf> {
        std::env::var_os("HOME").map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn k(key: Key) -> Option<Vec<u8>> {
        key_bytes(key, Modifiers::NONE, false)
    }

    #[test]
    fn plain_keys() {
        assert_eq!(k(Key::Enter), Some(b"\r".to_vec()));
        assert_eq!(k(Key::Backspace), Some(vec![0x7f]));
        assert_eq!(k(Key::Tab), Some(b"\t".to_vec()));
        assert_eq!(k(Key::ArrowUp), Some(b"\x1b[A".to_vec()));
        assert_eq!(k(Key::Delete), Some(b"\x1b[3~".to_vec()));
        assert_eq!(k(Key::F5), Some(b"\x1b[15~".to_vec()));
        assert_eq!(k(Key::F1), Some(b"\x1bOP".to_vec()));
        assert_eq!(k(Key::A), None, "printable keys arrive as Text events");
    }

    #[test]
    fn application_cursor_mode() {
        assert_eq!(key_bytes(Key::ArrowUp, Modifiers::NONE, true), Some(b"\x1bOA".to_vec()));
        let ctrl = Modifiers { ctrl: true, ..Modifiers::NONE };
        assert_eq!(key_bytes(Key::ArrowRight, ctrl, true), Some(b"\x1b[1;5C".to_vec()), "modifiers win over application mode");
    }

    #[test]
    fn control_and_alt_chords() {
        let ctrl = Modifiers { ctrl: true, ..Modifiers::NONE };
        assert_eq!(key_bytes(Key::C, ctrl, false), Some(vec![3]));
        assert_eq!(key_bytes(Key::D, ctrl, false), Some(vec![4]));
        assert_eq!(key_bytes(Key::Z, ctrl, false), Some(vec![26]));
        assert_eq!(key_bytes(Key::Space, ctrl, false), Some(vec![0]));
        let alt = Modifiers { alt: true, ..Modifiers::NONE };
        assert_eq!(key_bytes(Key::B, alt, false), Some(b"\x1bb".to_vec()));
        assert_eq!(key_bytes(Key::Backspace, alt, false), Some(b"\x1b\x7f".to_vec()));
        let shift = Modifiers { shift: true, ..Modifiers::NONE };
        assert_eq!(key_bytes(Key::Tab, shift, false), Some(b"\x1b[Z".to_vec()));
        assert_eq!(key_bytes(Key::Home, Modifiers { shift: true, ctrl: true, ..Modifiers::NONE }, false), Some(b"\x1b[1;6H".to_vec()));
    }

    #[test]
    fn paste_is_bracketed_and_sanitised() {
        let ev = vec![Event::Paste("a\nb\x1b[201~c".into())];
        assert_eq!(input_bytes(&ev, false, true), b"\x1b[200~a\rbc\x1b[201~".to_vec());
        assert_eq!(input_bytes(&ev, false, false), b"a\rbc".to_vec());
    }

    #[test]
    fn clipboard_chords_do_not_leak_into_the_shell() {
        let ev = vec![Event::Key { key: Key::C, physical_key: None, pressed: true, repeat: false, modifiers: Modifiers { ctrl: true, shift: true, ..Modifiers::NONE } }];
        assert!(input_bytes(&ev, false, false).is_empty());
        let ev = vec![Event::Key { key: Key::C, physical_key: None, pressed: true, repeat: false, modifiers: Modifiers { ctrl: true, ..Modifiers::NONE } }];
        assert_eq!(input_bytes(&ev, false, false), vec![3], "plain Ctrl+C is SIGINT");
        let ev = vec![Event::Text("héllo".into())];
        assert_eq!(input_bytes(&ev, false, false), "héllo".as_bytes().to_vec());
    }

    #[test]
    fn quoting() {
        assert_eq!(shell_quote("a b"), "'a b'");
        assert_eq!(shell_quote("it's"), "'it'\\''s'");
    }

    #[cfg(all(unix, not(target_arch = "wasm32")))]
    #[test]
    fn a_real_shell_runs_commands() {
        let ctx = egui::Context::default();
        let mut live = Live::spawn(ctx, Some(std::path::Path::new("/tmp")), 80, 24).expect("spawn");
        live.write(b"echo smart-$((20+22))\r");
        let t0 = std::time::Instant::now();
        let mut ok = false;
        while t0.elapsed().as_secs() < 5 {
            let text = live.parser.lock().unwrap().screen().contents();
            if text.contains("smart-42") {
                ok = true;
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        assert!(ok, "shell output never appeared");
        live.resize(100, 30);
        assert_eq!(live.parser.lock().unwrap().screen().size(), (30, 100));
        assert_eq!(live.cwd().as_deref(), Some(std::path::Path::new("/tmp")));
        live.write(b"exit\r");
        let t1 = std::time::Instant::now();
        while live.is_alive() && t1.elapsed().as_secs() < 5 {
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        assert!(!live.is_alive());
    }
}
