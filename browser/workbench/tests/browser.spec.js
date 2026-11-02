import {test,expect} from '@playwright/test';
import fs from 'node:fs';
const fixture=JSON.parse(fs.readFileSync(new URL('../fixtures/snapshot.json',import.meta.url),'utf8'));
test('authored example uses real worker and native behavior',async({page})=>{
 await page.goto('./'); await expect(page.locator('#status')).toContainText('Ready.');
 await page.getByRole('button',{name:'Load authored example'}).click();
 await expect(page.locator('#input')).toHaveValue(/racket:authored-example/);
 await page.getByRole('button',{name:'Inspect snapshot',exact:true}).click();
 await expect(page.locator('#status')).toContainText('Snapshot inspected');
 const r=JSON.parse(await page.locator('#report').textContent());
 expect(r.available_commands).toEqual(['inspect','reset']);expect(r.present_roles).toEqual(['Surface','Control']);expect(r.unsupported_features[0].fallback).toBe('Text label');
 const timings=await page.locator('#status').evaluate(node=>({measurements:{...node.dataset},user_agent:navigator.userAgent,hardware_concurrency:navigator.hardwareConcurrency,scope:'Descriptive sample; includes browser/worker/message overhead. No comparative performance claim.'}));
 expect(Number(timings.measurements.inputBytes)).toBe(r.input_bytes);
 for(const field of ['startupMs','workerInitializationMs','queryRoundtripMs','workerComputationMs']){expect(Number.isFinite(Number(timings.measurements[field]))).toBe(true);expect(Number(timings.measurements[field])).toBeGreaterThanOrEqual(0);}
 fs.writeFileSync('.cache/worker-timings.json',JSON.stringify(timings,null,2));
});
test('own input, native rejection, clearing stale exports and keyboard',async({page})=>{
 await page.goto('./');await expect(page.locator('#status')).toContainText('Ready.');
 const v=structuredClone(fixture);v.actions[0].command='own command';v.experience.title='<img src=x onerror=alert(1)>';
 await page.locator('#input').fill(JSON.stringify(v));await page.getByRole('button',{name:'Inspect snapshot',exact:true}).focus();await page.keyboard.press('Enter');await expect(page.locator('#status')).toContainText('Snapshot inspected');
 expect(JSON.parse(await page.locator('#report').textContent()).available_commands[0]).toBe('own command');expect(await page.locator('img').count()).toBe(0);
 await page.locator('#input').fill('{"unknown":true}');await expect(page.locator('#download')).toBeDisabled();await page.locator('#inspect').click();await expect(page.locator('#status')).toContainText('Input was rejected');
 const empty=structuredClone(fixture);empty.actions=[];empty.scene=[];await page.locator('#input').fill(JSON.stringify(empty));await page.locator('#inspect').click();await expect(page.locator('#status')).toContainText('Snapshot inspected');await expect(page.locator('#summary')).toContainText('No player-available commands.');await expect(page.locator('#summary')).toContainText('No scene roles.');
});
test('native and browser exact JSON responses',async({page})=>{
 await page.goto('./');const cases=JSON.parse(fs.readFileSync('.cache/native-cases.json','utf8'));
 const measured=await page.evaluate(async cases=>{const m=await import('./pkg/racket_workbench.js');await m.default();return cases.map(c=>{const started=performance.now();const output=m.inspect_frames(c.input, c.frames ?? "3");return {output,ms:performance.now()-started,input_bytes:new TextEncoder().encode(c.input).length};});},cases);
 expect(measured.map(c=>c.output)).toEqual(cases.map(c=>c.output));
 fs.writeFileSync('.cache/parity-timings.json',JSON.stringify({cases:measured.map(({output,...timing})=>timing),scope:'Sequential direct WASM samples including JS call and serialization; not a benchmark advantage claim.'},null,2));
});
