//! Syntax highlighting with tree-sitter (Rust, Python, JavaScript, JSON, TOML, Bash, CSS, HTML, C, Go, Markdown) and a
//! small lexer for everything else, so *every* text file gets colour and nothing ever fails to preview.
use egui::{text::LayoutJob, Color32, FontId, TextFormat};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cls {
    Plain,
    Keyword,
    Function,
    Type,
    Str,
    Number,
    Comment,
    Punct,
    Property,
    Tag,
    Title,
    Link,
    Escape,
}

const NAMES: [&str; 37] = [
    "attribute", "comment", "constant", "constant.builtin", "constructor", "embedded", "escape", "function", "function.builtin", "function.method", "keyword", "number", "operator", "property", "punctuation", "punctuation.bracket", "punctuation.delimiter", "punctuation.special", "string", "string.special", "string.escape", "tag", "type", "type.builtin", "variable", "variable.builtin", "variable.parameter", "text.title", "text.literal", "text.uri", "text.reference", "text.emphasis", "text.strong", "label", "module", "boolean", "character",
];

fn cls_of(i: usize) -> Cls {
    match NAMES[i] {
        "keyword" | "boolean" | "variable.builtin" | "label" => Cls::Keyword,
        "function" | "function.builtin" | "function.method" | "constructor" => Cls::Function,
        "type" | "type.builtin" | "module" | "attribute" => Cls::Type,
        "string" | "string.special" | "text.literal" | "character" | "embedded" => Cls::Str,
        "number" | "constant" | "constant.builtin" => Cls::Number,
        "comment" => Cls::Comment,
        "operator" | "punctuation" | "punctuation.bracket" | "punctuation.delimiter" | "punctuation.special" => Cls::Punct,
        "property" | "variable.parameter" => Cls::Property,
        "tag" => Cls::Tag,
        "text.title" | "text.strong" => Cls::Title,
        "text.uri" | "text.reference" | "text.emphasis" => Cls::Link,
        "escape" | "string.escape" => Cls::Escape,
        _ => Cls::Plain,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn config(ext: &str) -> Option<tree_sitter_highlight::HighlightConfiguration> {
    let (lang, name, q): (tree_sitter::Language, &str, &str) = match ext {
        "rs" => (tree_sitter_rust::LANGUAGE.into(), "rust", tree_sitter_rust::HIGHLIGHTS_QUERY),
        "py" | "pyw" => (tree_sitter_python::LANGUAGE.into(), "python", tree_sitter_python::HIGHLIGHTS_QUERY),
        "js" | "mjs" | "cjs" | "jsx" => (tree_sitter_javascript::LANGUAGE.into(), "javascript", tree_sitter_javascript::HIGHLIGHT_QUERY),
        "json" | "jsonc" => (tree_sitter_json::LANGUAGE.into(), "json", tree_sitter_json::HIGHLIGHTS_QUERY),
        "toml" => (tree_sitter_toml_ng::LANGUAGE.into(), "toml", tree_sitter_toml_ng::HIGHLIGHTS_QUERY),
        "sh" | "bash" | "zsh" => (tree_sitter_bash::LANGUAGE.into(), "bash", tree_sitter_bash::HIGHLIGHT_QUERY),
        "css" => (tree_sitter_css::LANGUAGE.into(), "css", tree_sitter_css::HIGHLIGHTS_QUERY),
        "html" | "htm" => (tree_sitter_html::LANGUAGE.into(), "html", tree_sitter_html::HIGHLIGHTS_QUERY),
        "c" | "h" => (tree_sitter_c::LANGUAGE.into(), "c", tree_sitter_c::HIGHLIGHT_QUERY),
        "go" => (tree_sitter_go::LANGUAGE.into(), "go", tree_sitter_go::HIGHLIGHTS_QUERY),
        "md" | "markdown" => (tree_sitter_md::LANGUAGE.into(), "markdown", tree_sitter_md::HIGHLIGHT_QUERY_BLOCK),
        _ => return None,
    };
    let mut c = tree_sitter_highlight::HighlightConfiguration::new(lang, name, q, "", "").ok()?;
    c.configure(&NAMES);
    Some(c)
}

pub fn has_grammar(ext: &str) -> bool {
    !cfg!(target_arch = "wasm32") && matches!(ext, "rs" | "py" | "pyw" | "js" | "mjs" | "cjs" | "jsx" | "json" | "jsonc" | "toml" | "sh" | "bash" | "zsh" | "css" | "html" | "htm" | "c" | "h" | "go" | "md" | "markdown")
}

/// Classified byte ranges covering the whole of `src`, in order.
pub fn runs(src: &str, ext: &str) -> Vec<(usize, usize, Cls)> {
    let ext = ext.to_ascii_lowercase();
    ts_runs(src, &ext).unwrap_or_else(|| lex(src, &ext))
}

/// tree-sitter path. The C grammars do not build for `wasm32-unknown-unknown` with the current crates, so the web
/// preview uses the lexer below; the desktop app (and AppImage) gets full parse-tree highlighting.
#[cfg(target_arch = "wasm32")]
fn ts_runs(_src: &str, _ext: &str) -> Option<Vec<(usize, usize, Cls)>> {
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn ts_runs(src: &str, ext: &str) -> Option<Vec<(usize, usize, Cls)>> {
    use tree_sitter_highlight::{Highlight, HighlightEvent, Highlighter};
    let cfg = config(ext)?;
    let mut h = Highlighter::new();
    let it = h.highlight(&cfg, src.as_bytes(), None, None, |_| None).ok()?;
    let mut out: Vec<(usize, usize, Cls)> = vec![];
    let mut stack: Vec<Cls> = vec![];
    for ev in it {
        match ev {
            Ok(HighlightEvent::HighlightStart(Highlight(i))) => stack.push(cls_of(i)),
            Ok(HighlightEvent::HighlightEnd) => {
                stack.pop();
            }
            Ok(HighlightEvent::Source { start, end }) if end > start && end <= src.len() => {
                let c = stack.iter().rev().find(|c| **c != Cls::Plain).copied().unwrap_or(Cls::Plain);
                match out.last_mut() {
                    Some(l) if l.2 == c && l.1 == start => l.1 = end,
                    _ => out.push((start, end, c)),
                }
            }
            Ok(_) => {}
            Err(_) => break,
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

const KEYWORDS: &[&str] = &[
    "fn", "let", "const", "var", "if", "else", "elif", "for", "while", "do", "return", "class", "struct", "enum", "impl", "trait", "interface", "import", "export", "from", "package", "use", "mod", "pub", "static", "public", "private", "protected", "void", "int", "new", "try", "catch", "finally", "throw", "switch", "case", "break", "continue", "true", "false", "null", "nil", "None", "True", "False", "this", "self", "super", "async", "await", "match", "in", "is", "not", "and", "or", "def", "lambda", "type", "func", "extends", "implements", "namespace", "using", "end", "then", "fi", "done", "function", "echo",
];

/// A tiny lexer: comments, strings, numbers, a generic keyword list. Good enough to make unknown formats readable.
fn lex(src: &str, ext: &str) -> Vec<(usize, usize, Cls)> {
    let hash_comment = matches!(ext, "yaml" | "yml" | "conf" | "ini" | "cfg" | "env" | "rb" | "pl" | "mk" | "dockerfile" | "gitignore" | "txt" | "properties");
    let slash_comment = !hash_comment || matches!(ext, "ts" | "tsx" | "java" | "kt" | "swift" | "cpp" | "hpp" | "cc" | "cs" | "php" | "scss");
    let b = src.as_bytes();
    let mut out: Vec<(usize, usize, Cls)> = vec![];
    let push = |out: &mut Vec<(usize, usize, Cls)>, s: usize, e: usize, c: Cls| {
        if e > s {
            match out.last_mut() {
                Some(l) if l.2 == c && l.1 == s => l.1 = e,
                _ => out.push((s, e, c)),
            }
        }
    };
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        let start = i;
        if (hash_comment && c == b'#') || (slash_comment && c == b'/' && b.get(i + 1) == Some(&b'/')) || (ext == "sql" && c == b'-' && b.get(i + 1) == Some(&b'-')) {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
            push(&mut out, start, i, Cls::Comment);
        } else if slash_comment && c == b'/' && b.get(i + 1) == Some(&b'*') {
            i += 2;
            while i + 1 < b.len() && !(b[i] == b'*' && b[i + 1] == b'/') {
                i += 1;
            }
            i = (i + 2).min(b.len());
            push(&mut out, start, i, Cls::Comment);
        } else if c == b'"' || c == b'\'' || c == b'`' {
            i += 1;
            while i < b.len() && b[i] != c && b[i] != b'\n' {
                if b[i] == b'\\' {
                    i += 1;
                }
                i += 1;
            }
            i = (i + 1).min(b.len());
            push(&mut out, start, i, Cls::Str);
        } else if c.is_ascii_digit() {
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'.' || b[i] == b'_') {
                i += 1;
            }
            push(&mut out, start, i, Cls::Number);
        } else if c.is_ascii_alphabetic() || c == b'_' {
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            let w = &src[start..i];
            let cls = if KEYWORDS.contains(&w) {
                Cls::Keyword
            } else if w.chars().next().is_some_and(|ch| ch.is_uppercase()) {
                Cls::Type
            } else if b.get(i) == Some(&b'(') {
                Cls::Function
            } else {
                Cls::Plain
            };
            push(&mut out, start, i, cls);
        } else {
            // advance one full UTF-8 char
            i += 1;
            while i < b.len() && (b[i] & 0xC0) == 0x80 {
                i += 1;
            }
            push(&mut out, start, i, if matches!(c, b'{' | b'}' | b'(' | b')' | b'[' | b']' | b';' | b',' | b'=' | b'<' | b'>' | b'+' | b'-' | b'*' | b'/' | b'&' | b'|' | b'!' | b':') { Cls::Punct } else { Cls::Plain });
        }
    }
    out
}

pub fn color(c: Cls, dark: bool) -> Color32 {
    let (d, l) = match c {
        Cls::Plain => ((0xd8, 0xe2, 0xf2), (0x22, 0x2b, 0x3d)),
        Cls::Keyword => ((0xc7, 0x92, 0xea), (0x8a, 0x3f, 0xc4)),
        Cls::Function => ((0x82, 0xaa, 0xff), (0x1f, 0x5f, 0xd6)),
        Cls::Type => ((0xff, 0xcb, 0x6b), (0xa6, 0x6a, 0x00)),
        Cls::Str => ((0xc3, 0xe8, 0x8d), (0x2a, 0x7d, 0x32)),
        Cls::Number => ((0xf7, 0x8c, 0x6c), (0xc2, 0x4a, 0x1d)),
        Cls::Comment => ((0x6c, 0x7f, 0x9c), (0x7a, 0x86, 0x9c)),
        Cls::Punct => ((0x89, 0xdd, 0xff), (0x3b, 0x7d, 0x9e)),
        Cls::Property => ((0xf0, 0x71, 0x78), (0xc1, 0x32, 0x3a)),
        Cls::Tag => ((0xf0, 0x71, 0x78), (0xc1, 0x32, 0x3a)),
        Cls::Title => ((0x82, 0xaa, 0xff), (0x1f, 0x5f, 0xd6)),
        Cls::Link => ((0x80, 0xcb, 0xc4), (0x0e, 0x7c, 0x75)),
        Cls::Escape => ((0x89, 0xdd, 0xff), (0x3b, 0x7d, 0x9e)),
    };
    let (r, g, b) = if dark { d } else { l };
    Color32::from_rgb(r, g, b)
}

/// A ready-to-draw layout job. `wrap` is the wrap width (`f32::INFINITY` for horizontal scrolling code).
pub fn layout(src: &str, ext: &str, dark: bool, font: FontId, wrap: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    job.wrap.max_width = wrap;
    let mut at = 0;
    for (s, e, c) in runs(src, ext) {
        if s > at {
            job.append(&src[at..s], 0.0, TextFormat { font_id: font.clone(), color: color(Cls::Plain, dark), ..Default::default() });
        }
        job.append(&src[s..e], 0.0, TextFormat { font_id: font.clone(), color: color(c, dark), italics: c == Cls::Comment, ..Default::default() });
        at = e;
    }
    if at < src.len() {
        job.append(&src[at..], 0.0, TextFormat { font_id: font, color: color(Cls::Plain, dark), ..Default::default() });
    }
    job
}

#[cfg(test)]
mod tests {
    use super::*;

    fn class_at(src: &str, ext: &str, needle: &str) -> Cls {
        let at = src.find(needle).unwrap();
        runs(src, ext).into_iter().find(|(s, e, _)| *s <= at && at < *e).map(|r| r.2).unwrap_or(Cls::Plain)
    }

    fn covers(src: &str, ext: &str) {
        let r = runs(src, ext);
        let mut at = 0;
        for (s, e, _) in &r {
            assert!(*s >= at && e > s && *e <= src.len(), "overlap or overrun in {ext}");
            assert!(src.is_char_boundary(*s) && src.is_char_boundary(*e), "split inside a char in {ext}");
            at = *e;
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn tree_sitter_rust() {
        let s = "// hi\nfn main() { let x = \"a\"; let n = 42; }\n";
        assert_eq!(class_at(s, "rs", "fn"), Cls::Keyword);
        assert_eq!(class_at(s, "rs", "\"a\""), Cls::Str);
        assert_eq!(class_at(s, "rs", "42"), Cls::Number);
        assert_eq!(class_at(s, "rs", "// hi"), Cls::Comment);
        covers(s, "rs");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_grammar_loads_and_covers_its_input() {
        for (ext, src) in [("py", "def f(x):\n    return 'é' # c\n"), ("js", "const a = () => `x${1}`;\n"), ("json", "{\"a\": [1, true, null]}"), ("toml", "[a]\nb = \"c\"\n"), ("sh", "echo \"$HOME\" | grep x\n"), ("css", "a { color: #fff; }\n"), ("html", "<p class=\"x\">é</p>\n"), ("c", "int main(){return 0;}\n"), ("go", "package main\nfunc main() {}\n"), ("md", "# Title\n\nSome *text* and `code`\n\n- item\n")] {
            assert!(has_grammar(ext), "{ext}");
            assert!(config(ext).is_some(), "{ext} grammar must load");
            covers(src, ext);
            assert!(!runs(src, ext).is_empty());
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn markdown_headings_stand_out() {
        assert_eq!(class_at("# Brand brief\n\ntext\n", "md", "Brand"), Cls::Title);
    }

    #[test]
    fn fallback_lexer_for_unknown_formats() {
        let s = "# comment\nname: \"x\" # tail\nport: 8080\n";
        assert_eq!(class_at(s, "yaml", "# comment"), Cls::Comment);
        assert_eq!(class_at(s, "yaml", "\"x\""), Cls::Str);
        assert_eq!(class_at(s, "yaml", "8080"), Cls::Number);
        covers(s, "yaml");
        covers("日本語 テキスト /* c */ 'unterminated", "ts");
    }

    #[test]
    fn hostile_input_never_panics() {
        for s in ["", "\"", "'\\", "/*", "\u{0}\u{1}", "fn", "((((((((((", "\n\n\n"] {
            for ext in ["rs", "py", "md", "json", "unknown", "yaml"] {
                covers(s, ext);
            }
        }
    }

    #[test]
    fn layout_job_reproduces_the_text() {
        let s = "fn a() {}\n// x\n";
        let j = layout(s, "rs", true, FontId::monospace(12.0), f32::INFINITY);
        assert_eq!(j.text, s);
    }
}
