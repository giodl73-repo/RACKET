"""Require both committed lockfiles to share the published COURT decoder revision."""
import json,pathlib,subprocess,tomllib
ROOT=pathlib.Path(__file__).resolve().parents[2]
COURT='git+https://github.com/giodl73-repo/COURT.git?branch=main#1b9ecb5193e844cb39d0dc76c8ef099ce57edda7'
for name in ['Cargo.lock','browser/workbench/Cargo.lock']:
 packages=tomllib.loads((ROOT/name).read_text(encoding='utf-8'))['package']
 court=[p for p in packages if p['name'] in ['court-core','court-inspector']]
 assert len(court)==(1 if name=='Cargo.lock' else 2)
 assert all(p['source']==COURT for p in court),name
metadata=json.loads(subprocess.check_output(['cargo','+1.95.0','metadata','--manifest-path','browser/workbench/Cargo.toml','--locked','--format-version','1'],cwd=ROOT))
for p in metadata['packages']:
 if p['name'] in ['court-core','court-inspector']:assert p['source']==COURT,p['name']
 if p['name'] in ['racket-core','racket-workbench']:
  assert p['source'] is None and pathlib.Path(p['manifest_path']).resolve().is_relative_to(ROOT.resolve()),p['name']
print(json.dumps({'court_revision':COURT,'locks_agree':True,'first_party_manifests_contained':True,'local_court_override':False}))
