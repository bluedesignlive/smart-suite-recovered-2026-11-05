//! Pure-Rust archive support (works natively and in wasm): zip, tar, tar.gz, tar.xz, tar.bz2, gz, xz, bz2.
use crate::model::*;
use std::io::{Cursor, Read};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Format {
    Zip,
    Tar,
    TarGz,
}
impl Format {
    pub const ALL: [Format; 3] = [Format::Zip, Format::TarGz, Format::Tar];
    pub fn ext(self) -> &'static str {
        match self {
            Format::Zip => "zip",
            Format::Tar => "tar",
            Format::TarGz => "tar.gz",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Format::Zip => "ZIP",
            Format::Tar => "TAR",
            Format::TarGz => "TAR.GZ",
        }
    }
    pub fn hint(self) -> &'static str {
        match self {
            Format::Zip => "Opens everywhere",
            Format::Tar => "Uncompressed bundle",
            Format::TarGz => "Smaller, Unix-friendly",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Read_ {
    Zip,
    Tar,
    Tgz,
    Txz,
    Tbz,
    Gz,
    Xz,
    Bz,
}

pub fn detect(name: &str) -> Option<Read_> {
    let n = name.to_lowercase();
    Some(if n.ends_with(".zip") {
        Read_::Zip
    } else if n.ends_with(".tar.gz") || n.ends_with(".tgz") {
        Read_::Tgz
    } else if n.ends_with(".tar.xz") || n.ends_with(".txz") {
        Read_::Txz
    } else if n.ends_with(".tar.bz2") || n.ends_with(".tbz2") {
        Read_::Tbz
    } else if n.ends_with(".tar") {
        Read_::Tar
    } else if n.ends_with(".gz") {
        Read_::Gz
    } else if n.ends_with(".xz") {
        Read_::Xz
    } else if n.ends_with(".bz2") {
        Read_::Bz
    } else {
        return None;
    })
}
pub fn is_archive(name: &str) -> bool {
    detect(name).is_some()
}
/// Folder name to extract into.
pub fn extract_stem(name: &str) -> String {
    let l = name.to_lowercase();
    for suf in [".tar.gz", ".tar.xz", ".tar.bz2", ".tgz", ".txz", ".tbz2", ".tar", ".zip", ".gz", ".xz", ".bz2"] {
        if l.ends_with(suf) {
            return name[..name.len() - suf.len()].to_string();
        }
    }
    name.to_string()
}

pub struct ArcItem {
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
}

fn clean(p: &str) -> Option<String> {
    let parts: Vec<&str> = p.split(['/', '\\']).filter(|s| !s.is_empty() && *s != "." && !s.ends_with(':')).collect();
    if parts.iter().any(|s| *s == "..") || parts.is_empty() {
        return None; // zip-slip protection
    }
    Some(parts.join("/"))
}

fn tar_entries<Rd: Read>(r: Rd) -> R<Vec<(String, bool, Vec<u8>)>> {
    let mut a = tar::Archive::new(r);
    let mut out = Vec::new();
    for e in a.entries().map_err(|e| e.to_string())? {
        let mut e = e.map_err(|e| e.to_string())?;
        let is_dir = e.header().entry_type().is_dir();
        if !is_dir && !e.header().entry_type().is_file() {
            continue;
        }
        let p = e.path().map_err(|e| e.to_string())?.to_string_lossy().to_string();
        let mut v = Vec::new();
        if !is_dir {
            e.read_to_end(&mut v).map_err(|e| e.to_string())?;
        }
        if let Some(p) = clean(&p) {
            out.push((p, is_dir, v));
        }
    }
    Ok(out)
}

/// Decode a whole archive into (relative path, is_dir, bytes).
pub fn read_all(bytes: Vec<u8>, name: &str) -> R<Vec<(String, bool, Vec<u8>)>> {
    let fmt = detect(name).ok_or("Unsupported archive type")?;
    let single = |data: Vec<u8>| vec![(extract_stem(name), false, data)];
    match fmt {
        Read_::Zip => {
            let mut z = zip::ZipArchive::new(Cursor::new(bytes)).map_err(|e| e.to_string())?;
            let mut out = Vec::new();
            for i in 0..z.len() {
                let mut f = z.by_index(i).map_err(|e| e.to_string())?;
                let Some(p) = f.enclosed_name().map(|p| p.to_string_lossy().to_string()).and_then(|p| clean(&p)) else { continue };
                let mut v = Vec::new();
                if !f.is_dir() {
                    f.read_to_end(&mut v).map_err(|e| e.to_string())?;
                }
                out.push((p, f.is_dir(), v));
            }
            Ok(out)
        }
        Read_::Tar => tar_entries(Cursor::new(bytes)),
        Read_::Tgz => tar_entries(flate2::read::GzDecoder::new(Cursor::new(bytes))),
        Read_::Txz => {
            let mut out = Vec::new();
            lzma_rs::xz_decompress(&mut Cursor::new(bytes), &mut out).map_err(|e| format!("{e:?}"))?;
            tar_entries(Cursor::new(out))
        }
        Read_::Tbz => tar_entries(bzip2_rs::DecoderReader::new(Cursor::new(bytes))),
        Read_::Gz => {
            let mut v = Vec::new();
            flate2::read::GzDecoder::new(Cursor::new(bytes)).read_to_end(&mut v).map_err(|e| e.to_string())?;
            Ok(single(v))
        }
        Read_::Xz => {
            let mut v = Vec::new();
            lzma_rs::xz_decompress(&mut Cursor::new(bytes), &mut v).map_err(|e| format!("{e:?}"))?;
            Ok(single(v))
        }
        Read_::Bz => {
            let mut v = Vec::new();
            bzip2_rs::DecoderReader::new(Cursor::new(bytes)).read_to_end(&mut v).map_err(|e| e.to_string())?;
            Ok(single(v))
        }
    }
}

pub fn list(bytes: Vec<u8>, name: &str) -> R<Vec<ArcItem>> {
    Ok(read_all(bytes, name)?.into_iter().map(|(path, is_dir, d)| ArcItem { path, is_dir, size: d.len() as u64 }).collect())
}

// ───────────────────────── writing ─────────────────────────
enum W {
    Zip(zip::ZipWriter<Cursor<Vec<u8>>>),
    Tar(tar::Builder<Vec<u8>>),
    Tgz(tar::Builder<flate2::write::GzEncoder<Vec<u8>>>),
}
pub struct Compressor {
    w: W,
}

fn tar_header(len: u64, dir: bool, mtime: i64) -> tar::Header {
    let mut h = tar::Header::new_gnu();
    h.set_size(len);
    h.set_mode(if dir { 0o755 } else { 0o644 });
    h.set_mtime(mtime.max(0) as u64);
    h.set_entry_type(if dir { tar::EntryType::Directory } else { tar::EntryType::Regular });
    h.set_cksum();
    h
}

impl Compressor {
    pub fn new(f: Format) -> Self {
        Compressor {
            w: match f {
                Format::Zip => W::Zip(zip::ZipWriter::new(Cursor::new(Vec::new()))),
                Format::Tar => W::Tar(tar::Builder::new(Vec::new())),
                Format::TarGz => W::Tgz(tar::Builder::new(flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default()))),
            },
        }
    }
    pub fn add_dir(&mut self, name: &str, mtime: i64) -> R<()> {
        let e = |e: &dyn std::fmt::Display| e.to_string();
        match &mut self.w {
            W::Zip(z) => z.add_directory(format!("{name}/"), zip::write::SimpleFileOptions::default()).map_err(|x| e(&x)),
            W::Tar(t) => t.append_data(&mut tar_header(0, true, mtime), format!("{name}/"), std::io::empty()).map_err(|x| e(&x)),
            W::Tgz(t) => t.append_data(&mut tar_header(0, true, mtime), format!("{name}/"), std::io::empty()).map_err(|x| e(&x)),
        }
    }
    pub fn add_file(&mut self, name: &str, data: &[u8], mtime: i64) -> R<()> {
        use std::io::Write;
        let e = |e: &dyn std::fmt::Display| e.to_string();
        match &mut self.w {
            W::Zip(z) => {
                let o = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
                z.start_file(name, o).map_err(|x| e(&x))?;
                z.write_all(data).map_err(|x| e(&x))
            }
            W::Tar(t) => t.append_data(&mut tar_header(data.len() as u64, false, mtime), name, data).map_err(|x| e(&x)),
            W::Tgz(t) => t.append_data(&mut tar_header(data.len() as u64, false, mtime), name, data).map_err(|x| e(&x)),
        }
    }
    pub fn finish(self) -> R<Vec<u8>> {
        let e = |e: &dyn std::fmt::Display| e.to_string();
        match self.w {
            W::Zip(mut z) => z.finish().map(|c| c.into_inner()).map_err(|x| e(&x)),
            W::Tar(t) => t.into_inner().map_err(|x| e(&x)),
            W::Tgz(t) => t.into_inner().map_err(|x| e(&x))?.finish().map_err(|x| e(&x)),
        }
    }
}
