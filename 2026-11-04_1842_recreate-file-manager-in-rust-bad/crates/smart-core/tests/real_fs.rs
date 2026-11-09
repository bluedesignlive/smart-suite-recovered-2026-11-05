use smart_core::*;
use smart_core::archive::Format;

fn run(e: &mut Engine) {
    for _ in 0..4000 {
        e.tick(0.016);
        if e.jobs.iter().all(|j| !j.active()) { break; }
    }
}

#[test]
fn real_filesystem_roundtrip() {
    let base = std::env::temp_dir().join(format!("smart-core-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(base.join("src/sub")).unwrap();
    std::fs::write(base.join("src/a.txt"), b"hello world").unwrap();
    std::fs::write(base.join("src/sub/b.md"), b"# title\nneedle inside").unwrap();
    std::fs::create_dir_all(base.join("dst")).unwrap();
    let b = vfs::from_real(&base);
    let mut e = Engine::native();
    e.settings.pace = Pace::Instant;
    // list real directory
    let l = e.list(&format!("{b}/src")).unwrap();
    assert_eq!(l.len(), 2);
    // copy a folder tree
    e.start_transfer(&[format!("{b}/src")], &format!("{b}/dst"), false).unwrap();
    run(&mut e);
    assert_eq!(std::fs::read(base.join("dst/src/sub/b.md")).unwrap(), b"# title\nneedle inside");
    // compress then extract
    e.start_compress(&[format!("{b}/src")], &format!("{b}/dst"), Format::Zip, "bundle").unwrap();
    run(&mut e);
    assert!(base.join("dst/bundle.zip").exists());
    std::fs::create_dir_all(base.join("out")).unwrap();
    e.start_extract(&format!("{b}/dst/bundle.zip"), &format!("{b}/out")).unwrap();
    run(&mut e);
    assert_eq!(std::fs::read(base.join("out/bundle/src/a.txt")).unwrap(), b"hello world");
    // tar.gz
    e.start_compress(&[format!("{b}/src")], &format!("{b}/dst"), Format::TarGz, "t").unwrap();
    run(&mut e);
    e.start_extract(&format!("{b}/dst/t.tar.gz"), &format!("{b}/out")).unwrap();
    run(&mut e);
    assert!(base.join("out/t/src/sub/b.md").exists());
    // move (atomic rename)
    e.start_transfer(&[format!("{b}/dst/src")], &format!("{b}/out"), true).unwrap();
    run(&mut e);
    assert!(base.join("out/src/a.txt").exists() && !base.join("dst/src").exists());
    // conflict: copy again -> needs decision -> keep both
    e.start_transfer(&[format!("{b}/src/a.txt")], &format!("{b}/out/src"), false).unwrap();
    for _ in 0..5 { e.tick(0.016); }
    let id = e.jobs.iter().find(|j| j.state == JobState::NeedsDecision).expect("conflict").id;
    e.resolve(id, Resolution::KeepBoth, false);
    run(&mut e);
    assert!(base.join("out/src/a 2.txt").exists());
    // index + search
    e.index.reset(vec![b.clone()]);
    for _ in 0..100 { e.tick(0.016); }
    let q = search::parse("a.txt");
    let hits = search::run(&e.index, &q, &|_| None, &|_| 0, 10);
    assert!(!hits.is_empty());
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn demo_big_transfer_is_paced() {
    let mut e = Engine::demo();
    let dest = "/media/stick".to_string();
    let id = e.start_transfer(&["/home/naledzi/Desktop/file transfer.txt".into()], &dest, false).unwrap();
    for _ in 0..60 { e.tick(0.016); }
    let j = e.jobs.iter().find(|j| j.id == id).unwrap();
    assert!(j.active(), "5 GB copy must still be running after 1s");
    assert!(j.progress() > 0.0 && j.progress() < 0.5);
    for _ in 0..4000 { e.tick(0.05); if !e.jobs[0].active() { break; } }
    assert_eq!(e.stat("/media/stick/file transfer.txt").unwrap().size, 5_000_000_000);
}
