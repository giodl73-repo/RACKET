const $ = id => document.getElementById(id);
const input = $('input'), MAX_BYTES = 131072;
let worker, generation = 0, revision = 0, jobTimer, ready = false;
let report = null, pending = null, currentText = '', original = null;
let readToken = 0, readController = null, reading = false, engineMessage = 'Starting Rust engine...';
const bytes = text => new TextEncoder().encode(text).length;
const status = message => { $('status').textContent = message; };
function updateControls() {
  $('bytes').textContent = `${bytes(currentText).toLocaleString()} UTF-8 bytes`;
  $('inspect').disabled = !ready || pending !== null;
  $('lf-copy').disabled = !currentText;
}
function clearReport() {
  report = null; $('download').disabled = true; $('report').textContent = 'No report yet.';
  $('result-state').textContent = 'No report';
  $('summary').replaceChildren(element('p', 'Inspect the current input to see its declarations.', 'empty'));
}
function element(tag, text, className) {
  const node = document.createElement(tag); node.textContent = text;
  if (className) node.className = className;
  return node;
}
function render(result) {
  const container = $('summary'); container.replaceChildren();
  $('result-state').textContent = result.ok ? 'Inspected' : 'Rejected';
  if (!result.ok) { container.append(element('p', result.error, 'error')); return; }
  container.append(element('h3', 'Native windowless plan'));
  container.append(element('p', `Native count readiness: ${result.plan.is_scene_ready}; completion: ${result.runtime.completed}; ready frames: ${result.runtime.ready_frame_count}/${result.frame_count_requested}.`));
  container.append(element('p', `Commands: ${result.plan.command_count}; player-available: ${result.plan.player_command_count}; Surface / Actor / Prop nodes: ${result.plan.surface_nodes} / ${result.plan.actor_nodes} / ${result.plan.prop_nodes}.`));
  container.append(element('p', result.scope, 'boundary'));
  container.append(element('h3', `RACKET diagnostics (${result.plan.diagnostics.length})`));
  for (const diagnostic of result.plan.diagnostics) container.append(element('p', `${diagnostic.code}: ${diagnostic.message}`, 'command'));
  if (!result.plan.diagnostics.length) container.append(element('p', 'No native diagnostics; this does not certify a product or renderer.'));
  container.append(element('h3', 'COURT snapshot declarations'));
  const snap = result.snapshot;
  container.append(element('h3', snap.experience.title));
  container.append(element('p', `${snap.state_label} / ${snap.metadata.scene_contract_version}`, 'muted'));
  container.append(element('h3', `Available commands (${result.available_commands.length})`));
  if (!result.available_commands.length) container.append(element('p', 'No player-available commands.', 'muted'));
  for (const command of result.available_commands) container.append(element('div', command, 'command'));
  container.append(element('h3', 'All declared actions'));
  snap.actions.forEach((action, index) => {
    const row = element('div', '', 'command');
    row.append(element('strong', `${action.label} / ${action.availability.kind}`));
    row.append(element('span', `${action.command} - ${result.actions[index].player_available ? 'player-available' : 'not player-available'}`, 'muted'));
    for (const key of ['reason', 'guidance', 'warning', 'note']) if (key in action.availability) row.append(element('p', action.availability[key]));
    container.append(row);
  });
  container.append(element('h3', 'Present scene roles'));
  const roles = element('div', '', 'role-list');
  result.present_roles.forEach(role => roles.append(element('span', role)));
  container.append(roles);
  if (!result.present_roles.length) container.append(element('p', 'No scene roles.', 'muted'));
  container.append(element('h3', 'Unsupported-feature declarations'));
  for (const feature of result.unsupported_features) container.append(element('p', `${feature.feature}: ${feature.fallback}`));
  if (!result.unsupported_features.length) container.append(element('p', 'No declared hints. This does not establish renderer compatibility.', 'muted'));
  container.append(element('h3', 'Caller-supplied provenance'));
  container.append(element('p', `${snap.experience.provenance.class}: ${snap.experience.provenance.source_id ?? '(none)'}`, 'command'));
  container.append(element('h3', 'COURT adapter observations (not RACKET diagnostics)'));
  for (const observation of result.adapter_observations) container.append(element('p', observation, 'boundary'));
}
function invalidateRead() { readToken++; readController?.abort(); readController = null; if (reading) $('file-status').textContent = 'Pending read canceled; current input and original retained.'; reading = false; }
function changed(text, message) {
  revision++; currentText = text; clearReport();
  // Superseded work cannot publish a report or timeout into the new revision.
  if (pending) start();
  updateControls(); if (message) status(ready ? message : `${message} ${engineMessage}`);
}
function fail(message) {
  clearTimeout(jobTimer); generation++; worker?.terminate(); worker = null;
  ready = false; pending = null; clearReport(); updateControls();
  engineMessage = `${message} Restart engine to retry. Inputs and opened original are retained.`; status(engineMessage);
}
function start() {
  worker?.terminate(); clearTimeout(jobTimer); generation++;
  ready = false; pending = null; clearReport(); updateControls(); engineMessage = 'Starting Rust engine...'; status(engineMessage);
  const own = generation, startTime = performance.now();
  try { worker = new Worker('./worker.js', {type: 'module'}); }
  catch { fail('Rust worker could not start.'); return; }
  const active = worker;
  jobTimer = setTimeout(() => { if (own === generation) fail('Engine startup exceeded 12 seconds.'); }, 12000);
  active.onerror = active.onmessageerror = () => { if (own === generation && active === worker) fail('Rust engine failed.'); };
  active.onmessage = ({data}) => {
    if (own !== generation || active !== worker) return;
    if (data.type === 'ready' && !ready) {
      clearTimeout(jobTimer); ready = true; updateControls();
      $('status').dataset.startupMs = String(performance.now() - startTime);
      if (Number.isFinite(data.initialization_ms)) $('status').dataset.workerInitializationMs = String(data.initialization_ms);
      status('Ready. Inspect your snapshot or load the example.'); return;
    }
    if (!pending || data.id !== pending.id) return;
    clearTimeout(jobTimer); const request = pending; pending = null; updateControls();
    if (request.revision !== revision) return;
    if (data.type === 'error') { fail(data.message); return; }
    try {
      const result = JSON.parse(data.text); report = data.text;
      $('status').dataset.queryRoundtripMs = String(performance.now() - request.started);
      $('status').dataset.inputBytes = String(request.inputBytes);
      if (Number.isFinite(data.computation_ms)) $('status').dataset.workerComputationMs = String(data.computation_ms);
      $('report').textContent = report; $('download').disabled = false; render(result);
      status(result.ok ? 'Snapshot inspected. No actions were executed.' : 'Input was rejected; see the report.');
    } catch { fail('Rust engine returned an unreadable report.'); }
  };
}
input.addEventListener('input', () => { invalidateRead(); changed(input.value, 'Input changed; previous report cleared.'); });
function insert(text, event) {
  const next = input.value.slice(0, input.selectionStart) + text + input.value.slice(input.selectionEnd);
  if (bytes(next) > MAX_BYTES) {
    event.preventDefault(); $('file-status').textContent = 'Paste/drop exceeds 128 KiB UTF-8. Entire operation rejected; previous input retained.'; return;
  }
  // Drop is inserted explicitly at the current selection; paste uses browser editing.
  if (event.type === 'drop') {
    event.preventDefault(); input.setRangeText(text, input.selectionStart, input.selectionEnd, 'end');
    input.dispatchEvent(new Event('input', {bubbles: true}));
  }
}
input.addEventListener('paste', event => insert(event.clipboardData.getData('text/plain'), event));
input.addEventListener('dragover', event => event.preventDefault());
input.addEventListener('drop', event => {
  if (event.dataTransfer.files.length) { event.preventDefault(); openFile(event.dataTransfer.files[0]); }
  else insert(event.dataTransfer.getData('text/plain'), event);
});
$('inspect').onclick = () => {
  if (!ready || pending) return; clearReport();
  if ($('frames').value.length > 10) { status('Input was rejected: frame count exceeds 10 characters; nothing was truncated.'); return; }
  if (bytes(currentText) > MAX_BYTES) { status('Input exceeds 128 KiB UTF-8; nothing was truncated.'); return; }
  const id = crypto.randomUUID(); pending = {id, revision, started: performance.now(), inputBytes: bytes(currentText)}; updateControls(); status('Inspecting with Rust...');
  const own = generation;
  jobTimer = setTimeout(() => { if (own === generation) fail('Inspection exceeded 10 seconds.'); }, 10000);
  worker.postMessage({id, text: currentText, frames: $('frames').value});
};
$('frames').addEventListener('input', () => { invalidateRead(); changed(currentText, 'Frame count changed; previous report cleared.'); });
$('restart').onclick = start;
$('lf-copy').onclick = () => {
  invalidateRead(); const text = currentText.replace(/^\uFEFF/, '').replace(/\r\n?/g, '\n');
  input.value = text; changed(text, 'Explicit LF copy created; opened original bytes remain downloadable.');
};
async function openFile(file) {
  invalidateRead(); const token = readToken, ownRevision = revision;
  if (file.size > MAX_BYTES) { $('file-status').textContent = 'File exceeds 128 KiB UTF-8; previous input and original retained.'; return; }
  reading = true; $('file-status').textContent = `Reading ${file.name}...`;
  let timer;
  try {
    const buffer = await Promise.race([file.arrayBuffer(), new Promise((_, reject) => { timer = setTimeout(() => reject(new Error('File read exceeded 5 seconds.')), 5000); })]);
    if (token !== readToken || ownRevision !== revision) return;
    const raw = new Uint8Array(buffer);
    if (raw.length > MAX_BYTES) throw new Error('File exceeds 128 KiB UTF-8.');
    const text = new TextDecoder('utf-8', {fatal: true, ignoreBOM: true}).decode(raw);
    original = {raw, name: file.name}; $('original').disabled = false;
    input.value = text; changed(text, 'File loaded. Inspect it or edit your input.');
    $('file-status').textContent = `Opened ${file.name}: ${raw.length.toLocaleString()} original bytes retained.`;
  } catch (error) {
    if (token === readToken && ownRevision === revision) $('file-status').textContent = `${error.message} Previous input and original retained.`;
  } finally { clearTimeout(timer); if (token === readToken) reading = false; }
}
$('open-file').onclick = () => $('file').click();
$('file').onchange = event => { const file = event.target.files[0]; event.target.value = ''; if (file) openFile(file); };
$('example').onclick = async () => {
  invalidateRead(); const token = readToken, ownRevision = revision;
  const controller = new AbortController(); readController = controller; reading = true; $('file-status').textContent = 'Reading authored example...';
  const timer = setTimeout(() => controller.abort(), 5000);
  try {
    const response = await fetch('./snapshot.json', {signal: controller.signal});
    if (!response.ok) throw new Error('Example unavailable.');
    const text = await response.text();
    if (token !== readToken || ownRevision !== revision) return;
    if (bytes(text) > MAX_BYTES) throw new Error('Example exceeds adapter limit.');
    input.value = text; changed(text, 'Authored example loaded. Inspect it or edit your own input.');
    $('file-status').textContent = 'Authored example loaded; any opened original remains separately downloadable.';
  } catch (error) {
    if (token === readToken && ownRevision === revision) $('file-status').textContent = `Example could not load (${error.message}). Your input is retained.`;
  } finally { clearTimeout(timer); if (readController === controller) readController = null; if (token === readToken) reading = false; }
};
function download(data, name, type) {
  const url = URL.createObjectURL(new Blob([data], {type}));
  const a = document.createElement('a'); a.href = url; a.download = name; a.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}
$('original').onclick = () => { if (original) download(original.raw, original.name, 'application/octet-stream'); };
$('current').onclick = () => download(currentText, 'racket-current-snapshot.json', 'text/plain;charset=utf-8');
$('download').onclick = () => { if (report) download(report, 'racket-frame-report.json', 'application/json'); };
updateControls(); start();
