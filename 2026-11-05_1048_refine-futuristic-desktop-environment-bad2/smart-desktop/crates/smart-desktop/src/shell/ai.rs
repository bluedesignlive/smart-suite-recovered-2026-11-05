//! The OS-level assistant engine. One brain for every app: it sees the live shell state, remembers across sessions,
//! and acts through *tools* (open apps, arrange windows, change the theme, set the wallpaper, run shell commands,
//! generate small apps from design-system blocks…). Providers are plain OpenAI-compatible endpoints, so OpenRouter,
//! NVIDIA NIM, OpenAI, Ollama or any gateway works with a key; with no key an on-device intent engine still drives the OS.
use super::*;
use crate::sound::Snd;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, PartialEq)]
pub enum Role {
    User,
    Ai,
    Tool,
}

#[derive(Clone)]
pub struct Msg {
    pub role: Role,
    pub text: String,
    pub born: f64,
}

pub enum Ev {
    Reply(String, Vec<(String, Value)>),
    Error(String),
    Heard(String, bool),
    Listening(bool),
}

#[derive(Clone)]
pub struct Cfg {
    pub provider: usize,
    pub keys: Vec<String>,
    pub models: Vec<String>,
    pub base: String,
    pub voice_reply: bool,
}

#[derive(Clone)]
pub struct GenBlock {
    pub kind: String,
    pub text: String,
    pub value: f32,
    pub items: Vec<String>,
}

#[derive(Clone)]
pub struct GenApp {
    pub name: String,
    pub icon: String,
    pub blocks: Vec<GenBlock>,
}

pub const PROVIDERS: [(&str, &str, &str); 5] = [
    ("On-device", "", ""),
    ("OpenRouter", "https://openrouter.ai/api/v1", "openai/gpt-4o-mini"),
    ("NVIDIA NIM", "https://integrate.api.nvidia.com/v1", "meta/llama-3.1-70b-instruct"),
    ("OpenAI-compatible", "https://api.openai.com/v1", "gpt-4o-mini"),
    ("Ollama (local)", "http://localhost:11434/v1", "llama3.2"),
];

pub struct Ai {
    pub open: bool,
    pub mode: bool,
    pub input: String,
    pub msgs: Vec<Msg>,
    pub busy: bool,
    pub inbox: Arc<Mutex<Vec<Ev>>>,
    pub cfg: Cfg,
    pub memory: Vec<String>,
    pub name: String,
    pub speaking: bool,
    pub listening: bool,
    pub level: f32,
    pub ctx_app: Option<AppId>,
    pub focus_input: bool,
}

const KEY: &str = "smart.ai.v1";

impl Ai {
    pub fn load() -> Self {
        let mut a = Ai {
            open: false,
            mode: false,
            input: String::new(),
            msgs: vec![],
            busy: false,
            inbox: Arc::new(Mutex::new(vec![])),
            cfg: Cfg { provider: 0, keys: vec![String::new(); 5], models: PROVIDERS.iter().map(|p| p.2.to_string()).collect(), base: PROVIDERS[3].1.into(), voice_reply: false },
            memory: vec!["The user is building Smart Desktop, a portable Rust shell.".into()],
            name: "Zuri".into(),
            speaking: false,
            listening: false,
            level: 0.0,
            ctx_app: None,
            focus_input: false,
        };
        #[cfg(target_arch = "wasm32")]
        if let Some(s) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()).and_then(|st| st.get_item(KEY).ok().flatten()) {
            if let Ok(v) = serde_json::from_str::<Value>(&s) {
                a.name = v["name"].as_str().unwrap_or("Zuri").to_string();
                a.cfg.provider = v["provider"].as_u64().unwrap_or(0) as usize;
                a.cfg.voice_reply = v["voice"].as_bool().unwrap_or(false);
                a.cfg.base = v["base"].as_str().unwrap_or(PROVIDERS[3].1).to_string();
                for i in 0..5 {
                    if let Some(k) = v["keys"][i].as_str() {
                        a.cfg.keys[i] = k.to_string();
                    }
                    if let Some(m) = v["models"][i].as_str() {
                        a.cfg.models[i] = m.to_string();
                    }
                }
                if let Some(m) = v["memory"].as_array() {
                    a.memory = m.iter().filter_map(|x| x.as_str().map(String::from)).collect();
                }
            }
        }
        a
    }
    pub fn save(&self) {
        let v = json!({"name": self.name, "provider": self.cfg.provider, "voice": self.cfg.voice_reply, "base": self.cfg.base, "keys": self.cfg.keys, "models": self.cfg.models, "memory": self.memory});
        #[cfg(target_arch = "wasm32")]
        if let Some(st) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = st.set_item(KEY, &v.to_string());
        }
        let _ = v;
    }
}

fn has(t: &str, words: &[&str]) -> bool {
    words.iter().any(|w| t.contains(w))
}

pub fn tools_json() -> Value {
    let f = |name: &str, desc: &str, props: Value, req: &[&str]| json!({"type":"function","function":{"name":name,"description":desc,"parameters":{"type":"object","properties":props,"required":req}}});
    json!([
        f("open_app", "Open an app window", json!({"app":{"type":"string","enum":["files","photos","terminal","capture","settings","studio"]}}), &["app"]),
        f("close_app", "Close an app window", json!({"app":{"type":"string"}}), &["app"]),
        f("arrange_windows", "Re-lay-out all open windows", json!({"layout":{"type":"string","enum":["grid","cascade","split","focus"]}}), &["layout"]),
        f("snap_window", "Snap an app to a screen zone", json!({"app":{"type":"string"},"zone":{"type":"string","enum":["left","right","max","tl","tr","bl","br"]}}), &["app","zone"]),
        f("set_theme", "Switch light or dark", json!({"mode":{"type":"string","enum":["light","dark"]}}), &["mode"]),
        f("set_wallpaper", "Pick a built-in wallpaper by name", json!({"name":{"type":"string"}}), &["name"]),
        f("set_dock", "Change the dock", json!({"size":{"type":"number"},"magnification":{"type":"boolean"},"autohide":{"type":"boolean"}}), &[]),
        f("set_volume", "Set system volume 0-100", json!({"level":{"type":"number"}}), &["level"]),
        f("notify", "Show a desktop notification", json!({"title":{"type":"string"},"body":{"type":"string"}}), &["title"]),
        f("run_terminal", "Run a command in the Smart terminal", json!({"command":{"type":"string"}}), &["command"]),
        f("relayout_desktop", "Tidy the desktop icons", json!({}), &[]),
        f("remember", "Store a durable fact about the user", json!({"fact":{"type":"string"}}), &["fact"]),
        f("create_app", "Generate a small app from design-system blocks (heading,text,stat,progress,list,buttons)", json!({"name":{"type":"string"},"icon":{"type":"string"},"blocks":{"type":"array","items":{"type":"object","properties":{"kind":{"type":"string"},"text":{"type":"string"},"value":{"type":"number"},"items":{"type":"array","items":{"type":"string"}}}}}}), &["name","blocks"]),
        f("photos_adjust", "Adjust the open photo (values -1..1)", json!({"exposure":{"type":"number"},"contrast":{"type":"number"},"saturation":{"type":"number"},"warmth":{"type":"number"},"vignette":{"type":"number"}}), &[]),
        f("take_screenshot", "Capture the screen", json!({}), &[]),
        f("ai_mode", "Enter or leave full-screen AI mode", json!({"on":{"type":"boolean"}}), &["on"]),
        f("empty_trash", "Ask to empty the Void", json!({}), &[])
    ])
}

pub fn gen_template(kind: &str, name: &str) -> GenApp {
    let b = |k: &str, t: &str, v: f32, items: &[&str]| GenBlock { kind: k.into(), text: t.into(), value: v, items: items.iter().map(|s| s.to_string()).collect() };
    let blocks = match kind {
        "timer" => vec![b("heading", "Focus timer", 0.0, &[]), b("stat", "25:00", 0.0, &["minutes left"]), b("progress", "Session", 0.35, &[]), b("buttons", "", 0.0, &["Start", "Pause", "Reset"])],
        "notes" => vec![b("heading", "Quick notes", 0.0, &[]), b("list", "", 0.0, &["Ship the dock preview", "Record the demo", "Ask Zuri to tile windows"]), b("buttons", "", 0.0, &["New note", "Clear"])],
        "habit" => vec![b("heading", "Habits", 0.0, &[]), b("progress", "Water", 0.7, &[]), b("progress", "Reading", 0.4, &[]), b("progress", "Training", 0.9, &[]), b("buttons", "", 0.0, &["Check in"])],
        _ => vec![b("heading", name, 0.0, &[]), b("text", "Generated from design-system blocks. Ask your assistant to change anything here.", 0.0, &[]), b("stat", "42", 0.0, &["things done"]), b("buttons", "", 0.0, &["Primary", "Secondary"])],
    };
    GenApp { name: name.to_string(), icon: "widget-4".into(), blocks }
}

fn parse_chat(txt: &str) -> Ev {
    let Ok(v) = serde_json::from_str::<Value>(txt) else { return Ev::Error("The provider returned something that isn\u{2019}t JSON".into()) };
    if let Some(e) = v["error"]["message"].as_str() {
        return Ev::Error(e.to_string());
    }
    let m = &v["choices"][0]["message"];
    let text = m["content"].as_str().unwrap_or("").to_string();
    let mut calls = vec![];
    if let Some(tc) = m["tool_calls"].as_array() {
        for c in tc {
            let name = c["function"]["name"].as_str().unwrap_or("").to_string();
            let args: Value = serde_json::from_str(c["function"]["arguments"].as_str().unwrap_or("{}")).unwrap_or(json!({}));
            calls.push((name, args));
        }
    }
    Ev::Reply(text, calls)
}

#[cfg(target_arch = "wasm32")]
fn fetch_chat(url: String, key: String, body: String, inbox: Arc<Mutex<Vec<Ev>>>) {
    wasm_bindgen_futures::spawn_local(async move {
        let res: Result<String, String> = async {
            use wasm_bindgen::JsCast;
            use wasm_bindgen_futures::JsFuture;
            let win = web_sys::window().ok_or("no window".to_string())?;
            let headers = web_sys::Headers::new().map_err(|_| "headers".to_string())?;
            headers.set("Content-Type", "application/json").map_err(|_| "headers".to_string())?;
            if !key.is_empty() {
                headers.set("Authorization", &format!("Bearer {key}")).map_err(|_| "headers".to_string())?;
            }
            let _ = headers.set("X-Title", "Smart Desktop");
            #[allow(unused_mut)]
            let mut init = web_sys::RequestInit::new();
            init.set_method("POST");
            init.set_headers(&headers);
            init.set_body(&wasm_bindgen::JsValue::from_str(&body));
            let req = web_sys::Request::new_with_str_and_init(&url, &init).map_err(|_| "bad request".to_string())?;
            let resp = JsFuture::from(win.fetch_with_request(&req)).await.map_err(|_| "Couldn\u{2019}t reach the provider (network or CORS)".to_string())?;
            let resp: web_sys::Response = resp.dyn_into().map_err(|_| "bad response".to_string())?;
            let txt = JsFuture::from(resp.text().map_err(|_| "no body".to_string())?).await.map_err(|_| "no body".to_string())?.as_string().unwrap_or_default();
            if !resp.ok() {
                return Err(format!("{} \u{2014} {}", resp.status(), txt.chars().take(180).collect::<String>()));
            }
            Ok(txt)
        }
        .await;
        let ev = match res {
            Ok(t) => parse_chat(&t),
            Err(e) => Ev::Error(e),
        };
        inbox.lock().unwrap().push(ev);
    });
}
#[cfg(not(target_arch = "wasm32"))]
fn fetch_chat(_url: String, _key: String, _body: String, inbox: Arc<Mutex<Vec<Ev>>>) {
    inbox.lock().unwrap().push(Ev::Error("Network providers run in the web build".into()));
}

#[cfg(target_arch = "wasm32")]
pub fn start_listening(inbox: Arc<Mutex<Vec<Ev>>>) -> bool {
    use js_sys::Reflect;
    use wasm_bindgen::{closure::Closure, JsCast, JsValue};
    let Some(win) = web_sys::window() else { return false };
    let ctor = ["SpeechRecognition", "webkitSpeechRecognition"].iter().find_map(|n| Reflect::get(&win, &JsValue::from_str(n)).ok().filter(|v| v.is_function()));
    let Some(ctor) = ctor else { return false };
    let Ok(ctor) = ctor.dyn_into::<js_sys::Function>() else { return false };
    let Ok(rec) = Reflect::construct(&ctor, &js_sys::Array::new()) else { return false };
    let _ = Reflect::set(&rec, &"lang".into(), &"en-US".into());
    let _ = Reflect::set(&rec, &"interimResults".into(), &JsValue::TRUE);
    let ib = inbox.clone();
    let onresult = Closure::<dyn FnMut(JsValue)>::new(move |ev: JsValue| {
        let results = Reflect::get(&ev, &"results".into()).unwrap_or(JsValue::NULL);
        let len = Reflect::get(&results, &"length".into()).ok().and_then(|v| v.as_f64()).unwrap_or(0.0) as u32;
        let (mut text, mut fin) = (String::new(), false);
        for i in 0..len {
            let r = Reflect::get_u32(&results, i).unwrap_or(JsValue::NULL);
            let alt = Reflect::get_u32(&r, 0).unwrap_or(JsValue::NULL);
            if let Some(t) = Reflect::get(&alt, &"transcript".into()).ok().and_then(|v| v.as_string()) {
                text.push_str(&t);
            }
            fin = Reflect::get(&r, &"isFinal".into()).ok().and_then(|v| v.as_bool()).unwrap_or(false);
        }
        ib.lock().unwrap().push(Ev::Heard(text, fin));
    });
    let _ = Reflect::set(&rec, &"onresult".into(), onresult.as_ref());
    onresult.forget();
    let ib2 = inbox.clone();
    let onend = Closure::<dyn FnMut()>::new(move || ib2.lock().unwrap().push(Ev::Listening(false)));
    let _ = Reflect::set(&rec, &"onend".into(), onend.as_ref());
    onend.forget();
    if let Ok(start) = Reflect::get(&rec, &"start".into()) {
        if let Ok(f) = start.dyn_into::<js_sys::Function>() {
            return f.call0(&rec).is_ok();
        }
    }
    false
}
#[cfg(not(target_arch = "wasm32"))]
pub fn start_listening(_inbox: Arc<Mutex<Vec<Ev>>>) -> bool {
    false
}

impl SmartApp {
    pub fn ai_open_with(&mut self, text: String, app: Option<AppId>) {
        self.shell.ai.open = !self.shell.ai.mode;
        self.shell.ai.ctx_app = app;
        self.os_ai_send(text);
    }

    pub(crate) fn ai_cmd(&mut self, cmd: &str) -> bool {
        match cmd {
            "ai.toggle" => {
                if self.shell.ai.mode {
                    self.shell.ai.mode = false;
                } else {
                    self.shell.ai.open = !self.shell.ai.open;
                    self.shell.ai.ctx_app = Some(self.focused_app());
                    self.shell.ai.focus_input = true;
                    self.shell.snd.play(if self.shell.ai.open { Snd::Listen } else { Snd::Close });
                }
            }
            "ai.mode" => {
                self.shell.ai.mode = !self.shell.ai.mode;
                self.shell.ai.open = false;
                self.shell.ai.focus_input = true;
                self.shell.snd.play(Snd::Open);
            }
            "ai.mic" => {
                if self.shell.ai.listening {
                    self.shell.ai.listening = false;
                } else if start_listening(self.shell.ai.inbox.clone()) {
                    self.shell.ai.listening = true;
                    self.shell.snd.play(Snd::Listen);
                } else {
                    self.shell.notify("Voice input needs a browser with speech recognition", "Type instead \u{2014} everything works the same", NoteKind::Warn);
                }
            }
            "ai.clear" => self.shell.ai.msgs.clear(),
            _ => {
                if let Some(a) = cmd.strip_prefix("ai.open:") {
                    self.shell.ai.open = true;
                    self.shell.ai.ctx_app = AppId::from_name(a);
                    self.shell.ai.focus_input = true;
                    self.shell.snd.play(Snd::Listen);
                } else if let Some(t) = cmd.strip_prefix("ai.ask:") {
                    self.ai_open_with(t.to_string(), Some(self.focused_app()));
                }
            }
        }
        true
    }

    fn ai_state(&self) -> String {
        let wins: Vec<String> = self.shell.z.iter().rev().filter(|a| self.is_open(**a)).map(|a| format!("{}{}", a.name(), if self.is_visible(*a) { "" } else { " (minimized)" })).collect();
        let items: Vec<String> = self.items.iter().take(12).map(|e| e.name.clone()).collect();
        format!(
            "time {} {}; focused app: {}; open windows (front to back): {}; theme: {}; volume {:.0}%; dock size {:.0}; trash items {}; clipboard: {}; files in current folder: {}; wallpapers: {}",
            clock_date(),
            clock_hm(),
            self.focused_app().name(),
            if wins.is_empty() { "none".into() } else { wins.join(", ") },
            if self.st().dark { "dark" } else { "light" },
            self.shell.volume * 100.0,
            self.shell.dock.size,
            self.shell.trash_n,
            self.shell.clip.first().map(|c| c.text.chars().take(100).collect::<String>()).unwrap_or_else(|| "empty".into()),
            items.join(", "),
            WALLPAPERS.iter().map(|w| w.0).collect::<Vec<_>>().join(", ")
        )
    }

    pub fn os_ai_send(&mut self, text: String) {
        let text = text.trim().to_string();
        if text.is_empty() || self.shell.ai.busy {
            return;
        }
        let now = self.shell.now;
        self.shell.ai.msgs.push(Msg { role: Role::User, text: text.clone(), born: now });
        self.shell.ai.input.clear();
        self.shell.ai.busy = true;
        self.shell.snd.play(Snd::Think);
        let cfg = self.shell.ai.cfg.clone();
        let needs_key = cfg.provider != 0 && cfg.provider != 4;
        if cfg.provider == 0 || (needs_key && cfg.keys[cfg.provider].is_empty()) {
            let (reply, calls) = self.ai_local(&text);
            let mut r = reply;
            if cfg.provider != 0 && needs_key && cfg.keys[cfg.provider].is_empty() {
                r.push_str(&format!("\n\n(Add your {} key in Settings \u{2192} Assistant for open-ended answers.)", PROVIDERS[cfg.provider].0));
            }
            self.shell.ai.inbox.lock().unwrap().push(Ev::Reply(r, calls));
            return;
        }
        let name = self.shell.ai.name.clone();
        let system = format!(
            "You are {name}, the assistant built into Smart Desktop, an operating-system shell. You can see and control the machine through tools. Prefer calling tools for actions, then reply in one or two short friendly sentences. Durable memory about the user: {}. Live state: {}.",
            self.shell.ai.memory.join(" | "),
            self.ai_state()
        );
        let mut messages = vec![json!({"role":"system","content":system})];
        for m in self.shell.ai.msgs.iter().rev().take(12).collect::<Vec<_>>().into_iter().rev() {
            match m.role {
                Role::User => messages.push(json!({"role":"user","content":m.text})),
                Role::Ai => messages.push(json!({"role":"assistant","content":m.text})),
                Role::Tool => {}
            }
        }
        let base = if cfg.provider == 3 { cfg.base.clone() } else { PROVIDERS[cfg.provider].1.to_string() };
        let body = json!({"model": cfg.models[cfg.provider], "messages": messages, "tools": tools_json(), "tool_choice": "auto"});
        fetch_chat(format!("{}/chat/completions", base.trim_end_matches('/')), cfg.keys[cfg.provider].clone(), body.to_string(), self.shell.ai.inbox.clone());
    }

    pub(crate) fn ai_pump(&mut self, ctx: &egui::Context) {
        let evs: Vec<Ev> = std::mem::take(&mut *self.shell.ai.inbox.lock().unwrap());
        if self.shell.ai.busy || self.shell.ai.speaking || self.shell.ai.listening {
            ctx.request_repaint();
        }
        let now = self.shell.now;
        for ev in evs {
            match ev {
                Ev::Reply(text, calls) => {
                    for (n, a) in calls {
                        let s = self.ai_tool(&n, &a);
                        self.shell.ai.msgs.push(Msg { role: Role::Tool, text: s, born: now });
                    }
                    let text = if text.trim().is_empty() { "Done.".to_string() } else { text };
                    self.shell.ai.msgs.push(Msg { role: Role::Ai, text: text.clone(), born: now });
                    self.shell.ai.busy = false;
                    self.shell.snd.play(Snd::Reply);
                    if self.shell.ai.cfg.voice_reply || self.shell.ai.mode {
                        crate::sound::speak(&text, self.shell.tts_rate, self.shell.volume.max(0.3));
                    }
                    self.shell.ai.save();
                }
                Ev::Error(e) => {
                    self.shell.ai.msgs.push(Msg { role: Role::Ai, text: format!("That didn\u{2019}t work: {e}"), born: now });
                    self.shell.ai.busy = false;
                    self.shell.snd.play(Snd::Error);
                }
                Ev::Heard(t, fin) => {
                    self.shell.ai.input = t.clone();
                    self.shell.ai.level = 1.0;
                    if fin {
                        self.shell.ai.listening = false;
                        self.os_ai_send(t);
                    }
                }
                Ev::Listening(on) => self.shell.ai.listening = on,
            }
        }
        self.shell.ai.level *= 0.94;
    }

    pub(crate) fn ai_tool(&mut self, name: &str, a: &Value) -> String {
        let s = |k: &str| a[k].as_str().unwrap_or("").to_string();
        match name {
            "open_app" => match AppId::from_name(&s("app")) {
                Some(id) => {
                    self.open_app(id);
                    format!("Opened {}", id.name())
                }
                None => format!("No app called {}", s("app")),
            },
            "close_app" => match AppId::from_name(&s("app")) {
                Some(AppId::Files) => {
                    self.win_closed = true;
                    "Closed Files".into()
                }
                Some(id) => {
                    self.app_win_mut(id).open = false;
                    format!("Closed {}", id.name())
                }
                None => "Nothing to close".into(),
            },
            "arrange_windows" => self.apply_layout(&s("layout")),
            "snap_window" => match AppId::from_name(&s("app")) {
                Some(id) => self.snap_app(id, &s("zone")),
                None => "Unknown app".into(),
            },
            "set_theme" => {
                let want_dark = s("mode") == "dark";
                if self.st().dark != want_dark {
                    self.acts.push(Act::ToggleTheme);
                }
                format!("Theme \u{2192} {}", s("mode"))
            }
            "set_wallpaper" => {
                let n = s("name").to_lowercase();
                match WALLPAPERS.iter().position(|w| w.0.to_lowercase().contains(&n) || n.contains(&w.0.to_lowercase())) {
                    Some(i) => {
                        self.acts.push(Act::SetWallpaper(i));
                        super::desktop::clear_video();
                        self.shell.video_bg = None;
                        format!("Wallpaper \u{2192} {}", WALLPAPERS[i].0)
                    }
                    None => {
                        self.shell.desk.wall_open = true;
                        "Opened the wallpaper picker".into()
                    }
                }
            }
            "set_dock" => {
                if let Some(v) = a["size"].as_f64() {
                    self.shell.dock.size = (v as f32).clamp(32.0, 64.0);
                }
                if let Some(v) = a["magnification"].as_bool() {
                    self.shell.dock.mag = if v { 1.0 } else { 0.0 };
                }
                if let Some(v) = a["autohide"].as_bool() {
                    self.shell.dock.autohide = v;
                }
                "Dock updated".into()
            }
            "set_volume" => {
                self.shell.volume = (a["level"].as_f64().unwrap_or(50.0) as f32 / 100.0).clamp(0.0, 1.0);
                self.shell.snd.vol = self.shell.volume;
                self.shell.snd.play(Snd::Volume);
                format!("Volume \u{2192} {:.0}%", self.shell.volume * 100.0)
            }
            "notify" => {
                self.shell.notify(&s("title"), &s("body"), NoteKind::Info);
                "Notification sent".into()
            }
            "run_terminal" => {
                self.open_app(AppId::Terminal);
                if self.shell.term.sessions.is_empty() {
                    self.shell.term.command("new");
                }
                self.term_submit(0, s("command"));
                format!("Ran `{}`", s("command"))
            }
            "relayout_desktop" => {
                self.shell.desk.pos.clear();
                self.shell.snd.play(Snd::Snap);
                "Desktop icons tidied".into()
            }
            "remember" => {
                self.shell.ai.memory.push(s("fact"));
                if self.shell.ai.memory.len() > 40 {
                    self.shell.ai.memory.remove(0);
                }
                self.shell.ai.save();
                format!("Remembered: {}", s("fact"))
            }
            "create_app" => {
                let blocks: Vec<GenBlock> = a["blocks"]
                    .as_array()
                    .map(|v| {
                        v.iter()
                            .map(|b| GenBlock { kind: b["kind"].as_str().unwrap_or("text").into(), text: b["text"].as_str().unwrap_or("").into(), value: b["value"].as_f64().unwrap_or(0.0) as f32, items: b["items"].as_array().map(|i| i.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default() })
                            .collect()
                    })
                    .unwrap_or_default();
                self.shell.gen.push(GenApp { name: s("name"), icon: "widget-4".into(), blocks });
                self.shell.gen_sel = self.shell.gen.len() - 1;
                self.open_app(AppId::Studio);
                format!("Built \u{201c}{}\u{201d} in Studio", s("name"))
            }
            "photos_adjust" => {
                self.open_app(AppId::Photos);
                if let Some(l) = self.photos_first() {
                    self.photos_show(l);
                }
                let g = |k: &str, d: f32| a[k].as_f64().map(|v| v as f32).unwrap_or(d);
                self.shell.photos.edit = true;
                self.shell.photos.adj = crate::photos::Adjust { exposure: g("exposure", 0.1), contrast: g("contrast", 0.2), saturation: g("saturation", 0.2), warmth: g("warmth", 0.0), vignette: g("vignette", 0.2) };
                self.shell.photos.dirty = true;
                "Adjusted the photo".into()
            }
            "take_screenshot" => {
                self.cap_cmd("cap.shot");
                "Captured the screen".into()
            }
            "ai_mode" => {
                self.shell.ai.mode = a["on"].as_bool().unwrap_or(true);
                self.shell.ai.open = false;
                "AI mode".into()
            }
            "empty_trash" => {
                self.app_cmd("dock.empty");
                "Asked for confirmation".into()
            }
            _ => format!("Unknown tool {name}"),
        }
    }

    /// On-device intent engine: no key, no network, still drives the whole OS.
    fn ai_local(&mut self, text: &str) -> (String, Vec<(String, Value)>) {
        let t = text.to_lowercase();
        let nm = self.shell.ai.name.clone();
        let call = |n: &str, v: Value| vec![(n.to_string(), v)];
        let app_in = |t: &str| -> Option<AppId> { ["files", "photos", "terminal", "settings", "capture", "screenshot", "recorder", "studio"].iter().find(|w| t.contains(*w)).and_then(|w| AppId::from_name(w)) };
        if has(&t, &["remember "]) {
            let fact = text.split_once("remember").map(|x| x.1).unwrap_or("").trim().trim_start_matches("that").trim().to_string();
            return (format!("Got it \u{2014} I\u{2019}ll remember that across every app."), call("remember", json!({ "fact": fact })));
        }
        if has(&t, &["make ", "create ", "build ", "generate "]) && has(&t, &[" app", "tool", "widget"]) {
            let kind = ["timer", "notes", "habit"].iter().find(|k| t.contains(*k)).copied().unwrap_or("custom");
            let nm2 = format!("{} app", kind[..1].to_uppercase() + &kind[1..]);
            let g = gen_template(kind, &nm2);
            let blocks: Vec<Value> = g.blocks.iter().map(|b| json!({"kind":b.kind,"text":b.text,"value":b.value,"items":b.items})).collect();
            return (format!("Built a {kind} app from the design system and put it in Studio. Tell me what to change."), call("create_app", json!({"name": nm2, "blocks": blocks})));
        }
        if has(&t, &["ai mode", "full screen assistant", "voice mode"]) {
            return ("Entering AI mode \u{2014} the dock steps aside. Press Esc to return.".into(), call("ai_mode", json!({"on": true})));
        }
        if has(&t, &["tile", "grid", "cascade", "split", "arrange", "relayout", "re-layout", "layout", "clean up"]) && has(&t, &["window", "layout", "arrange", "tile", "cascade", "split", "everything"]) {
            let l = if has(&t, &["cascade"]) { "cascade" } else if has(&t, &["split"]) { "split" } else if has(&t, &["focus"]) { "focus" } else { "grid" };
            return (format!("Rearranging your windows ({l}) \u{2014} watch them glide."), call("arrange_windows", json!({"layout": l})));
        }
        if has(&t, &["clean up the desktop", "tidy", "desktop icons"]) {
            return ("Tidied the desktop icons.".into(), call("relayout_desktop", json!({})));
        }
        if has(&t, &["snap", "put "]) && has(&t, &["left", "right", "corner", "full"]) {
            let id = app_in(&t).unwrap_or(self.focused_app());
            let z = if has(&t, &["top left"]) { "tl" } else if has(&t, &["top right"]) { "tr" } else if has(&t, &["bottom left"]) { "bl" } else if has(&t, &["bottom right"]) { "br" } else if has(&t, &["left"]) { "left" } else if has(&t, &["right"]) { "right" } else { "max" };
            return (format!("Snapped {} to {z}.", id.name()), call("snap_window", json!({"app": id.name(), "zone": z})));
        }
        if has(&t, &["dark mode", "dark theme", "go dark", "switch to dark"]) {
            return ("Dark it is.".into(), call("set_theme", json!({"mode": "dark"})));
        }
        if has(&t, &["light mode", "light theme", "go light", "switch to light"]) {
            return ("Light mode on.".into(), call("set_theme", json!({"mode": "light"})));
        }
        if has(&t, &["wallpaper", "background"]) {
            let found = WALLPAPERS.iter().find(|w| t.contains(&w.0.to_lowercase())).map(|w| w.0);
            return match found {
                Some(n) => (format!("Wallpaper changed to {n}."), call("set_wallpaper", json!({"name": n}))),
                None => ("Here are your wallpapers \u{2014} pick one, upload a picture or a video.".into(), call("set_wallpaper", json!({"name": "?"}))),
            };
        }
        if has(&t, &["volume", "louder", "quieter", "mute"]) {
            let n: Option<f64> = t.split_whitespace().find_map(|w| w.trim_end_matches('%').parse().ok());
            let cur = self.shell.volume as f64 * 100.0;
            let lvl = n.unwrap_or(if has(&t, &["mute"]) { 0.0 } else if has(&t, &["louder", "up"]) { cur + 15.0 } else { cur - 15.0 });
            return (format!("Volume set to {:.0}%.", lvl.clamp(0.0, 100.0)), call("set_volume", json!({"level": lvl})));
        }
        if has(&t, &["dock"]) {
            let v = if has(&t, &["bigger", "larger"]) { json!({"size": self.shell.dock.size + 8.0}) } else if has(&t, &["smaller"]) { json!({"size": self.shell.dock.size - 8.0}) } else if has(&t, &["hide"]) { json!({"autohide": true}) } else if has(&t, &["show"]) { json!({"autohide": false}) } else if has(&t, &["magnif"]) { json!({"magnification": self.shell.dock.mag < 0.1}) } else { json!({}) };
            return ("Dock updated.".into(), call("set_dock", v));
        }
        if has(&t, &["empty the trash", "empty trash", "empty the void"]) {
            return ("I\u{2019}ll ask you to confirm before emptying the Void.".into(), call("empty_trash", json!({})));
        }
        if has(&t, &["screenshot", "screen shot", "capture the screen"]) {
            return ("Captured \u{2014} it\u{2019}s in Pictures.".into(), call("take_screenshot", json!({})));
        }
        if has(&t, &["warmer", "enhance", "brighten", "make it pop", "improve this photo", "vibrant"]) {
            return ("Warmed it up and added some punch.".into(), call("photos_adjust", json!({"exposure": 0.12, "contrast": 0.25, "saturation": 0.3, "warmth": 0.3, "vignette": 0.25})));
        }
        if t.starts_with("run ") || t.starts_with("terminal ") {
            let cmd = t.splitn(2, ' ').nth(1).unwrap_or("ls").to_string();
            return (format!("Running `{cmd}` in the terminal."), call("run_terminal", json!({"command": cmd})));
        }
        if has(&t, &["open ", "launch ", "start ", "show me"]) {
            if let Some(id) = app_in(&t) {
                return (format!("Opening {}.", id.name()), call("open_app", json!({"app": id.name()})));
            }
        }
        if has(&t, &["close "]) {
            if let Some(id) = app_in(&t) {
                return (format!("Closed {}.", id.name()), call("close_app", json!({"app": id.name()})));
            }
        }
        if has(&t, &["folder", "this directory", "what's here", "whats here", "summar"]) {
            let n = self.items.len();
            let size: u64 = self.items.iter().map(|e| e.size).sum();
            let mut big: Vec<&Entry> = self.items.iter().filter(|e| !e.is_dir).collect();
            big.sort_by_key(|e| std::cmp::Reverse(e.size));
            let top: Vec<String> = big.iter().take(3).map(|e| format!("{} ({})", e.name, format_size(e.size))).collect();
            return (format!("This folder holds {n} items, {} in total.{}", format_size(size), if top.is_empty() { String::new() } else { format!(" Largest: {}.", top.join(", ")) }), vec![]);
        }
        if has(&t, &["clipboard", "copied"]) {
            self.shell.clip_open = true;
            return (match self.shell.clip.first() {
                Some(c) => format!("Latest on your clipboard: \u{201c}{}\u{201d}", c.text.chars().take(120).collect::<String>()),
                None => "Your clipboard is empty.".into(),
            }, vec![]);
        }
        if has(&t, &["what's open", "whats open", "status", "what is open", "state"]) {
            return (self.ai_state(), vec![]);
        }
        if has(&t, &["explain", "what does", "last command"]) {
            let last = self.shell.term.history.last().cloned().unwrap_or_default();
            let ex = match last.split_whitespace().next().unwrap_or("") {
                "ls" => "`ls` lists what is in a folder.",
                "cd" => "`cd` changes the working folder.",
                "top" | "btop" => "`top` is a live view of processes, CPU and memory.",
                "cat" => "`cat` prints a file.",
                "" => "Run something in the terminal first and I\u{2019}ll explain it.",
                _ => "That runs in the Smart shell \u{2014} Tab completes, \u{2191} recalls history.",
            };
            return (ex.into(), vec![]);
        }
        if has(&t, &["who are you", "your name", "what are you"]) {
            return (format!("I\u{2019}m {nm}, the assistant built into this desktop. I see every window, your clipboard and your files, and I can act on them. You can rename me in Settings."), vec![]);
        }
        if has(&t, &["hello", "hi ", "hey", "yo "]) || t == "hi" {
            return (format!("Hi \u{2014} {nm} here. Tell me what to do: \u{201c}tile my windows\u{201d}, \u{201c}dark mode\u{201d}, \u{201c}snap photos left\u{201d}, \u{201c}make a timer app\u{201d}\u{2026}"), vec![]);
        }
        (format!("I can open apps, arrange windows, change theme, wallpaper or dock, run terminal commands, take screenshots and build small apps. Add a model key in Settings \u{2192} Assistant and I can talk about anything."), vec![])
    }
}
