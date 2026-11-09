// Boot loader for Smart Files (the app is 100% Rust; this file unpacks the embedded wasm module
// and forwards keyboard chords that browsers would otherwise swallow).
import init, { start, chord } from './glue.js';
import b64 from './wasm.js';

async function unpack(base64) {
  const bin = atob(base64);
  const bytes = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i += 1) bytes[i] = bin.charCodeAt(i);
  if (typeof DecompressionStream !== 'function') throw new Error('this browser lacks DecompressionStream');
  const stream = new Blob([bytes]).stream().pipeThrough(new DecompressionStream('gzip'));
  return new Response(stream).arrayBuffer();
}

// ⌘/Ctrl + letter rarely reaches a canvas app, so the chords are recognised here (capture phase)
// and handed to the Rust side, which decides what to do based on the current UI state.
const CHORDS = [
  ['palette', (m, s, k) => (m && !s && k === 'k') || k === 'f1' || (m && s && k === 'p')],
  ['copy', (m, s, k) => m && !s && k === 'c'],
  ['cut', (m, s, k) => m && !s && k === 'x'],
  ['paste', (m, s, k) => m && !s && k === 'v'],
  ['selectall', (m, s, k) => m && !s && k === 'a'],
  ['undo', (m, s, k) => m && !s && k === 'z'],
  ['duplicate', (m, s, k) => m && !s && k === 'd'],
  ['info', (m, s, k) => m && !s && k === 'i'],
  ['newfolder', (m, s, k) => m && s && k === 'n'],
  ['search', (m, s, k) => m && !s && k === 'f'],
  ['ai', (m, s, k) => m && !s && k === 'j'],
  ['sidebar', (m, s, k) => m && !s && k === '\\'],
];
const PREVENT = new Set(['palette', 'search', 'ai', 'sidebar', 'newfolder']);
document.addEventListener(
  'keydown',
  (e) => {
    const m = e.ctrlKey || e.metaKey;
    const s = e.shiftKey;
    const k = (e.key || '').toLowerCase();
    const hit = CHORDS.find(([, test]) => test(m, s, k));
    if (!hit || !window.__smart_ready) return;
    if (PREVENT.has(hit[0])) e.preventDefault();
    chord(hit[0]);
  },
  true,
);

(async () => {
  const boot = document.getElementById('boot');
  try {
    const wasm = await unpack(b64);
    await init({ module_or_path: wasm });
    window.__smart_ready = true;
    const reduced = window.matchMedia && window.matchMedia('(prefers-reduced-motion: reduce)').matches;
    const coarse = window.matchMedia && window.matchMedia('(pointer: coarse)').matches;
    await start('smart_canvas', !!reduced, !!coarse);
    if (boot) boot.remove();
  } catch (e) {
    if (boot) boot.textContent = 'Failed to start: ' + e;
    console.error(e);
  }
})();
