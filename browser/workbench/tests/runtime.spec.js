import {test,expect} from '@playwright/test';
import fs from 'node:fs';
const source=fs.readFileSync(new URL('../fixtures/snapshot.json',import.meta.url),'utf8');
test('browser agrees with the pre-existing accepted and rejected runtime proof',async({page})=>{
 await page.goto('./');const controls=JSON.parse(fs.readFileSync('.cache/native-proof-controls.json','utf8'));
 const actual=await page.evaluate(async controls=>{const m=await import('./pkg/racket_workbench.js');await m.default();return controls.map(c=>{const r=JSON.parse(m.inspect_frames(c.input,c.frames)).runtime;return {completed:r.completed,frameCount:r.frames.length,readyFrameCount:r.ready_frame_count,diagnostics:r.diagnostics};});},controls);
 expect(actual).toEqual(controls.map(c=>c.expected));
});
async function ready(page){await page.goto('./');await expect(page.locator('#status')).toContainText('Ready.');await page.locator('#input').fill(source);}
async function run(page,frames){await page.locator('#frames').fill(frames);await page.locator('#inspect').click();await expect(page.locator('#status')).toContainText(/Snapshot inspected|Input was rejected/);return JSON.parse(await page.locator('#report').textContent());}

test('native readiness survives diagnostics; zero frames and not-ready frames stay distinct',async({page})=>{
 await ready(page);
 let r=await run(page,'3');expect(r.plan.is_scene_ready).toBe(true);expect(r.plan.diagnostics.length).toBeGreaterThan(0);expect(r.runtime.completed).toBe(true);expect(r.runtime.ready_frame_count).toBe(3);
 expect(r.runtime.frames.map(f=>f.frame_index)).toEqual([0,1,2]);for(const f of r.runtime.frames)expect(f.plan).toEqual(r.plan);
 r=await run(page,'0');expect(r.plan.is_scene_ready).toBe(true);expect(r.runtime.completed).toBe(false);expect(r.runtime.frames).toEqual([]);
 const v=JSON.parse(source);v.scene=[];await page.locator('#input').fill(JSON.stringify(v));r=await run(page,'2');expect(r.plan.is_scene_ready).toBe(false);expect(r.runtime.completed).toBe(false);expect(r.runtime.frames.length).toBe(2);expect(r.runtime.ready_frame_count).toBe(0);
 for(const frames of ['-1','1.5','33','4294967296','']){r=await run(page,frames);expect(r.ok).toBe(false);}
});

test('large diagnostic expansion rejects before cloning and leaves engine usable',async({page})=>{
 await ready(page);const v=JSON.parse(source);v.scene[0].unsupported_features=[{feature:'x'.repeat(40000),fallback:'reduce'}];await page.locator('#input').fill(JSON.stringify(v));
 let r=await run(page,'32');expect(r.ok).toBe(false);expect(r.error).toContain('Expanded frame plans');
 await page.locator('#input').fill(source);r=await run(page,'32');expect(r.ok).toBe(true);expect(r.runtime.frames.length).toBe(32);
});

test('changing frame count fences old worker callbacks and file reads',async({page})=>{
 await page.addInitScript(()=>{window.workers=[];window.Worker=class{constructor(){window.workers.push(this);setTimeout(()=>this.onmessage?.({data:{type:'ready'}}),0);}postMessage(data){this.request=data;}terminate(){}};const original=Blob.prototype.arrayBuffer;Blob.prototype.arrayBuffer=function(){return new Promise(resolve=>setTimeout(()=>resolve(original.call(this)),700));};});
 await ready(page);await page.locator('#inspect').click();await page.locator('#frames').fill('0');await expect(page.locator('#status')).toContainText('Ready.');
 await page.evaluate(()=>{const w=window.workers[0];w.onmessage({data:{type:'result',id:w.request.id,text:JSON.stringify({ok:false,error:'OLD FRAMES'})}});});await expect(page.locator('#download')).toBeDisabled();await expect(page.locator('#report')).toHaveText('No report yet.');
 await page.locator('#file').setInputFiles({name:'late.json',mimeType:'application/json',buffer:Buffer.from('{}')});await page.locator('#frames').fill('2');await page.waitForTimeout(900);await expect(page.locator('#input')).toHaveValue(source);await expect(page.locator('#original')).toBeDisabled();
});

test('compiled WASM enforces the 64 MiB memory ceiling',async({page})=>{
 await page.goto('./');const result=await page.evaluate(async()=>{const m=await import('./pkg/racket_workbench.js');const wasm=await m.default();const before=wasm.memory.buffer.byteLength;wasm.memory.grow(1024-before/65536);let rejected=false;try{wasm.memory.grow(1);}catch{rejected=true;}return {rejected,bytes:wasm.memory.buffer.byteLength};});expect(result).toEqual({rejected:true,bytes:67108864});
});
