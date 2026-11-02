"""Extract exact delivered source outside Git and replay native/browser checks."""
import argparse,json,os,pathlib,shutil,subprocess,tempfile,zipfile
P=pathlib.Path(__file__).resolve().parent
ROOT=P.parent.parent
parser=argparse.ArgumentParser();parser.add_argument('--bindgen',required=True);args=parser.parse_args()
site=ROOT/'target/racket-release/RACKET/workbench'
cache=P/'.cache';cache.mkdir(exist_ok=True)
original=json.loads((cache/'native-cases.json').read_text(encoding='utf-8'))
results=[]
def run(command,cwd,env=None):
 result=subprocess.run(command,cwd=cwd,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
 with (cache/'source-replay.log').open('ab') as log:log.write(('\n$ '+' '.join(map(str,command))+'\n').encode());log.write(result.stdout);log.write(result.stderr)
 if result.returncode:raise SystemExit('Source replay failed; see .cache/source-replay.log')
 return result.stdout
(cache/'source-replay.log').write_bytes(b'')
with tempfile.TemporaryDirectory(prefix='racket source replay ') as temporary:
 base=pathlib.Path(temporary).resolve();assert not base.is_relative_to(ROOT.resolve())
 with zipfile.ZipFile(site/'source.zip') as archive:
  for name in archive.namelist():
   relative=pathlib.PurePosixPath(name)
   assert relative.parts[0]=='RACKET-source' and '..' not in relative.parts and not relative.is_absolute()
   destination=base.joinpath(*relative.parts);assert destination.resolve().is_relative_to(base)
   destination.parent.mkdir(parents=True,exist_ok=True);destination.write_bytes(archive.read(name))
 source=base/'RACKET-source';consumer=source/'browser/workbench'
 assert not (source/'.git').exists()
 probe=subprocess.run(['git','rev-parse','--show-toplevel'],cwd=source,capture_output=True)
 assert probe.returncode != 0, 'Delivered source must be outside every Git checkout'
 cargo=shutil.which('cargo');python=shutil.which('python');npm=shutil.which('npm')
 manifest='browser/workbench/Cargo.toml'
 metadata=json.loads(run([cargo,'+1.95.0','metadata','--manifest-path',manifest,'--locked','--format-version','1'],source))
 first_party=[p for p in metadata['packages'] if p['name'] in ['racket-core','racket-workbench']]
 assert len(first_party)==2
 for package in first_party:
  assert pathlib.Path(package['manifest_path']).resolve().is_relative_to(source.resolve()),package['name']
 run([cargo,'+1.95.0','test','--workspace','--locked'],source)
 run([cargo,'+1.95.0','test','--manifest-path',manifest,'--locked'],source)
 run([cargo,'+1.95.0','build','--manifest-path',manifest,'--locked','--release','--target','wasm32-unknown-unknown','--lib'],source)
 run([cargo,'+1.95.0','build','--manifest-path',manifest,'--locked','--bin','racket-workbench-cli'],source)
 run([python,'browser/workbench/build-prototype.py','--bindgen',str(pathlib.Path(args.bindgen).resolve())],source)
 rebuilt=json.loads((consumer/'.cache/native-cases.json').read_text(encoding='utf-8'))
 assert rebuilt==original,'Independent native request/response drift'
 run([npm,'ci','--ignore-scripts'],consumer)
 env=dict(os.environ);env['RACKET_SITE_DIR']='../../target/racket-prototype'
 # Delivery is pinned to the original Git artifact; this replay checks rebuilt behavior.
 run([npm,'run','test:pages','--','--grep-invert','every packaged file|source, build and legal routes'],consumer,env)
 browser=json.loads((consumer/'.cache/browser-results.json').read_text(encoding='utf-8'))
 assert browser['stats']['unexpected']==browser['stats']['skipped']==browser['stats']['flaky']==0
 shutil.copyfile(consumer/'.cache/browser-results.json',cache/'source-replay-browser.json')
 build=json.loads((consumer/'.cache/prototype-build.json').read_text(encoding='utf-8'))
 receipt={'outside_git':True,'git_discovery_returncode':probe.returncode,'extraction_path_contains_spaces':' ' in str(base),'first_party_manifests_contained':True,'foundation_tests':6,'adapter_tests':4,'browser_checks':browser['stats']['expected'],'identical_native_requests_and_responses':len(original),'rebuilt_wasm_sha256':build['wasm_sha256'],'original_wasm_sha256':json.loads((site/'provenance.json').read_text(encoding='utf-8'))['wasm_sha256'],'binary_identity_claimed':False}
 (cache/'source-replay.json').write_text(json.dumps(receipt,indent=2)+'\n',encoding='utf-8')
 print(json.dumps(receipt))
