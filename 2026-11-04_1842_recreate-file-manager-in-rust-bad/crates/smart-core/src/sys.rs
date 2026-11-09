//! Host integration: disks, mounting, default apps, phones. Native implementations + wasm stubs.
use crate::model::*;

#[derive(Clone, Debug)]
pub struct AppInfo {
    pub id: String,
    pub name: String,
    pub exec: Option<String>,
    pub icon: Icon,
    pub default: bool,
}

#[derive(Clone, Debug)]
pub struct HostDisk {
    pub label: String,
    pub mount_point: String,
    pub device: String,
    pub removable: bool,
    pub total: u64,
    pub free: u64,
    pub mounted: bool,
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
#[cfg(target_arch = "wasm32")]
pub use web::*;

#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    pub fn home() -> Option<String> {
        None
    }
    pub fn user_dir(_: &str) -> Option<String> {
        None
    }
    pub fn host_disks() -> Vec<HostDisk> {
        vec![]
    }
    pub fn unmount(_: &HostDisk) -> R<()> {
        Err("Unmounting needs a native host".into())
    }
    pub fn mount(_: &HostDisk) -> R<String> {
        Err("Mounting needs a native host".into())
    }
    pub fn apps_for(_: Kind, _: &str) -> Vec<AppInfo> {
        vec![]
    }
    pub fn open_default(_: &str) -> R<()> {
        Err("No host shell in the browser".into())
    }
    pub fn launch(_: &AppInfo, _: &str) -> R<()> {
        Err("No host shell in the browser".into())
    }
    pub fn terminal_at(_: &str) -> R<()> {
        Err("No host shell in the browser".into())
    }
    pub fn adb_devices() -> Vec<(String, String)> {
        vec![]
    }
    pub const NATIVE: bool = false;
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::*;
    use crate::vfs::*;
    use std::process::Command;
    pub const NATIVE: bool = true;

    pub fn home() -> Option<String> {
        dirs::home_dir().map(|p| from_real(&p))
    }
    pub fn user_dir(which: &str) -> Option<String> {
        match which {
            "Desktop" => dirs::desktop_dir(),
            "Documents" => dirs::document_dir(),
            "Downloads" => dirs::download_dir(),
            "Pictures" => dirs::picture_dir(),
            "Music" => dirs::audio_dir(),
            "Movies" => dirs::video_dir(),
            _ => None,
        }
        .map(|p| from_real(&p))
    }

    /// Mounted disks from sysinfo plus (Linux) unmounted removable partitions from lsblk.
    pub fn host_disks() -> Vec<HostDisk> {
        let mut out: Vec<HostDisk> = vec![];
        let disks = sysinfo::Disks::new_with_refreshed_list();
        for d in disks.list() {
            let mp = from_real(d.mount_point());
            let fs = d.file_system().to_string_lossy().to_lowercase();
            if matches!(fs.as_str(), "tmpfs" | "devtmpfs" | "squashfs" | "overlay" | "proc" | "sysfs" | "efivarfs" | "cgroup2") && mp != "/" {
                continue;
            }
            if ["/boot", "/snap", "/sys", "/run", "/proc", "/dev", "/var/lib"].iter().any(|b| mp.starts_with(b)) {
                continue;
            }
            if out.iter().any(|o| o.mount_point == mp) {
                continue;
            }
            let name = d.name().to_string_lossy().to_string();
            out.push(HostDisk {
                label: if mp == "/" { "Computer".into() } else if !name.is_empty() && !name.starts_with("/dev") { name.clone() } else { name_of(&mp).to_string() },
                mount_point: mp,
                device: name,
                removable: d.is_removable(),
                total: d.total_space(),
                free: d.available_space(),
                mounted: true,
            });
        }
        #[cfg(target_os = "linux")]
        if let Ok(o) = Command::new("lsblk").args(["-P", "-b", "-o", "NAME,LABEL,MOUNTPOINT,RM,SIZE,TYPE,FSTYPE"]).output() {
            for l in String::from_utf8_lossy(&o.stdout).lines() {
                let g = |k: &str| -> String {
                    l.split(&format!("{k}=\"")).nth(1).and_then(|r| r.split('"').next()).unwrap_or("").to_string()
                };
                if g("RM") == "1" && g("TYPE") == "part" && g("MOUNTPOINT").is_empty() && !g("FSTYPE").is_empty() {
                    out.push(HostDisk {
                        label: if g("LABEL").is_empty() { g("NAME") } else { g("LABEL") },
                        mount_point: String::new(),
                        device: format!("/dev/{}", g("NAME")),
                        removable: true,
                        total: g("SIZE").parse().unwrap_or(0),
                        free: 0,
                        mounted: false,
                    });
                }
            }
        }
        out
    }

    pub fn unmount(d: &HostDisk) -> R<()> {
        #[cfg(target_os = "linux")]
        let r = Command::new("udisksctl").args(["unmount", "-b", &d.device]).output().or_else(|_| Command::new("umount").arg(&d.mount_point).output());
        #[cfg(target_os = "macos")]
        let r = Command::new("diskutil").args(["unmount", &d.mount_point]).output();
        #[cfg(target_os = "windows")]
        let r = Command::new("powershell").args(["-NoProfile", "-Command", &format!("(New-Object -comObject Shell.Application).Namespace(17).ParseName('{}:').InvokeVerb('Eject')", d.mount_point.trim_start_matches('/'))]).output();
        let o = r.map_err(|e| e.to_string())?;
        if o.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(if o.stderr.is_empty() { &o.stdout } else { &o.stderr }).trim().to_string())
        }
    }
    pub fn mount(d: &HostDisk) -> R<String> {
        #[cfg(target_os = "linux")]
        {
            let o = Command::new("udisksctl").args(["mount", "-b", &d.device]).output().map_err(|e| e.to_string())?;
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            if o.status.success() {
                // "Mounted /dev/sdb1 at /media/user/LABEL."
                return Ok(s.rsplit(" at ").next().unwrap_or("").trim().trim_end_matches('.').to_string());
            }
            return Err(String::from_utf8_lossy(&o.stderr).trim().to_string());
        }
        #[allow(unreachable_code)]
        Err("Mounting is handled by the OS on this platform".into())
    }

    pub fn open_default(p: &str) -> R<()> {
        open::that_detached(to_real(p)).map_err(|e| e.to_string())
    }
    pub fn terminal_at(p: &str) -> R<()> {
        let dir = to_real(p);
        for t in ["x-terminal-emulator", "gnome-terminal", "konsole", "kitty", "alacritty", "xterm", "wezterm"] {
            let r = match t {
                "gnome-terminal" => Command::new(t).arg(format!("--working-directory={}", dir.display())).spawn(),
                "konsole" => Command::new(t).arg("--workdir").arg(&dir).spawn(),
                _ => Command::new(t).current_dir(&dir).spawn(),
            };
            if r.is_ok() {
                return Ok(());
            }
        }
        #[cfg(target_os = "macos")]
        if Command::new("open").args(["-a", "Terminal"]).arg(&dir).spawn().is_ok() {
            return Ok(());
        }
        Err("No terminal emulator found".into())
    }
    pub fn launch(app: &AppInfo, p: &str) -> R<()> {
        let real = to_real(p).to_string_lossy().to_string();
        match &app.exec {
            Some(exec) => {
                let mut parts: Vec<String> = exec.split_whitespace().map(|s| s.to_string()).collect();
                let mut used = false;
                for s in parts.iter_mut() {
                    if matches!(s.as_str(), "%f" | "%F" | "%u" | "%U") {
                        *s = real.clone();
                        used = true;
                    }
                }
                parts.retain(|s| !s.starts_with('%'));
                if !used {
                    parts.push(real);
                }
                Command::new(&parts[0]).args(&parts[1..]).spawn().map(|_| ()).map_err(|e| e.to_string())
            }
            None => open_default(p),
        }
    }

    /// Installed apps able to open this kind of file (parses freedesktop .desktop entries on Linux).
    pub fn apps_for(kind: Kind, ext: &str) -> Vec<AppInfo> {
        let mut out = vec![AppInfo { id: "system".into(), name: "System default".into(), exec: None, icon: Icon::OpenExt, default: true }];
        #[cfg(target_os = "linux")]
        {
            let want: &[&str] = match kind {
                Kind::Image => &["image/"],
                Kind::Audio => &["audio/"],
                Kind::Video => &["video/"],
                Kind::Text | Kind::Code => &["text/"],
                Kind::Document => &["application/pdf", "application/vnd", "text/"],
                Kind::Archive => &["application/zip", "application/x-tar", "application/x-compressed"],
                Kind::Folder => &["inode/directory"],
                _ => &["application/octet-stream"],
            };
            let _ = ext;
            let mut dirs = vec!["/usr/share/applications".to_string(), "/usr/local/share/applications".to_string()];
            if let Some(h) = dirs::data_dir() {
                dirs.push(h.join("applications").to_string_lossy().to_string());
            }
            for d in dirs {
                let Ok(rd) = std::fs::read_dir(&d) else { continue };
                for f in rd.flatten() {
                    let Ok(txt) = std::fs::read_to_string(f.path()) else { continue };
                    let get = |k: &str| txt.lines().find_map(|l| l.strip_prefix(&format!("{k}=")).map(|s| s.to_string()));
                    if get("NoDisplay").as_deref() == Some("true") || get("Type").as_deref() != Some("Application") {
                        continue;
                    }
                    let (Some(name), Some(exec), Some(mimes)) = (get("Name"), get("Exec"), get("MimeType")) else { continue };
                    if want.iter().any(|w| mimes.split(';').any(|m| m.starts_with(w))) && !out.iter().any(|a| a.name == name) {
                        out.push(AppInfo { id: f.file_name().to_string_lossy().to_string(), name, exec: Some(exec), icon: Icon::App, default: false });
                    }
                }
            }
        }
        let _ = (kind, ext);
        out.truncate(14);
        out
    }

    // ───────────── phones via adb ─────────────
    pub fn adb_devices() -> Vec<(String, String)> {
        let Ok(o) = Command::new("adb").args(["devices", "-l"]).output() else { return vec![] };
        String::from_utf8_lossy(&o.stdout)
            .lines()
            .skip(1)
            .filter(|l| l.contains("device") && !l.contains("offline"))
            .filter_map(|l| {
                let serial = l.split_whitespace().next()?.to_string();
                let model = l.split("model:").nth(1).and_then(|m| m.split_whitespace().next()).unwrap_or("Android phone").replace('_', " ");
                Some((serial, model))
            })
            .collect()
    }

    /// Read-only Android browsing over adb (`adb shell ls`, `adb exec-out`).
    pub struct AdbFs {
        pub serial: String,
        pub root: String,
    }
    impl AdbFs {
        fn sh(&self, cmd: &str) -> R<Vec<u8>> {
            let o = Command::new("adb").args(["-s", &self.serial, "exec-out", cmd]).output().map_err(|e| e.to_string())?;
            Ok(o.stdout)
        }
        fn remote(&self, p: &str) -> String {
            let rel = p.strip_prefix(&self.root).unwrap_or("");
            format!("/sdcard{}", rel)
        }
    }
    impl Vfs for AdbFs {
        fn read_dir(&self, p: &str) -> R<Vec<Entry>> {
            let out = self.sh(&format!("ls -la --time-style=+%s '{}'", self.remote(p)))?;
            Ok(String::from_utf8_lossy(&out)
                .lines()
                .filter_map(|l| {
                    let f: Vec<&str> = l.split_whitespace().collect();
                    if f.len() < 7 || !(l.starts_with('d') || l.starts_with('-') || l.starts_with('l')) {
                        return None;
                    }
                    let name = f[6..].join(" ");
                    if name == "." || name == ".." {
                        return None;
                    }
                    Some(Entry::new(&join(p, name.split(" -> ").next().unwrap_or(&name)), l.starts_with('d'), f[4].parse().unwrap_or(0), f[5].parse().unwrap_or(0)))
                })
                .collect())
        }
        fn stat(&self, p: &str) -> R<Entry> {
            if p == self.root {
                return Ok(Entry::new(p, true, 0, 0));
            }
            self.read_dir(&parent(p))?.into_iter().find(|e| e.path == p).ok_or_else(|| "Not found".into())
        }
        fn mkdir(&mut self, _: &str) -> R<()> {
            Err("Phone storage is read-only here".into())
        }
        fn remove(&mut self, _: &str) -> R<()> {
            Err("Phone storage is read-only here".into())
        }
        fn rename(&mut self, _: &str, _: &str) -> R<()> {
            Err("Phone storage is read-only here".into())
        }
        fn read_at(&self, p: &str, off: u64, len: u64) -> R<Chunk> {
            Ok(Chunk::Data(self.sh(&format!("tail -c +{} '{}' | head -c {}", off + 1, self.remote(p), len))?))
        }
        fn write_chunk(&mut self, _: &str, _: u64, _: Chunk) -> R<()> {
            Err("Phone storage is read-only here".into())
        }
    }
}
