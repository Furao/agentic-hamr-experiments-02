#!/usr/bin/env python3
"""CR-02 delta adapter for h-wf-effort v1. Does not run codegen/build/tests."""
import argparse, collections, copy, difflib, hashlib, html, json, re, subprocess, sys
from pathlib import Path
sys.dont_write_bytecode = True
OUT=Path(__file__).resolve().parent
PROJECT=OUT.parent.parent
SKILL=PROJECT.parent/'.agents/skills/h-wf-effort'
sys.path.insert(0,str(SKILL/'scripts'))
import measure_project as mp
import estimate_effort as ee

def read(p): return json.loads(p.read_text())
def write(p,x): p.write_text(json.dumps(x,indent=2,ensure_ascii=False)+'\n')
def sha(b): return hashlib.sha256(b).hexdigest()
def git(*args,check=True): return subprocess.run(['git',*args],cwd=PROJECT,check=check,capture_output=True).stdout

def at(ref,path):
 p=subprocess.run(['git','show',f'{ref}:mavlink_02/{path}'],cwd=PROJECT,capture_output=True)
 return p.stdout.decode() if p.returncode==0 else None

def delta(old,new):
 a=[x.strip() for x in old.splitlines()];b=[x.strip() for x in new.splitlines()]
 added=set();removed=set()
 for op,i,j,k,l in difflib.SequenceMatcher(None,a,b,autojunk=False).get_opcodes():
  if op!='equal':removed.update(range(i,j));added.update(range(k,l))
 # Reordered identical lines are not new authorship.
 available=collections.defaultdict(list)
 for i in sorted(removed):available[a[i]].append(i)
 for i in sorted(added):
  if available[b[i]]:
   removed.remove(available[b[i]].pop());added.remove(i)
 return added,removed

def language(path):
 ext=Path(path).suffix
 return {'.rs':'rust','.sysml':'sysml','.c':'c','.h':'c','.xml':'xml','.system':'xml','.toml':'toml','.md':'markdown','.py':'shell','.cmd':'shell','.mk':'makefile','.dot':'dot'}.get(ext,'makefile' if Path(path).name=='Makefile' else 'text')

def classes(text,lang):
 c=mp.classify_lines(text,lang)
 if lang=='sysml':
  lines=text.splitlines()
  for sp in mp.extract_gumbo_spans(text):
   a,b=sp['start_line']-1,sp['end_line']
   c[a:b]=['code']+mp.classify_lines('\n'.join(lines[a+1:b-1])+'\n','gumbo')+['code']
 assert len(c)==len(text.splitlines()), 'classification line-count mismatch'
 return c

def rust_regions(text):
 # Blank comments and string contents while preserving byte positions/newlines.
 clean=re.sub(r'"(?:\\.|[^"\\])*"|//[^\n]*|/\*.*?\*/',
              lambda m: ''.join('\n' if c=='\n' else ' ' for c in m.group()),
              text, flags=re.S)
 tests=set();proofs=set()
 patterns=[(r'#\[cfg\(test\)\]\s*mod\s+\w+\s*\{',tests),
           (r'\b(?:spec|proof)\s+fn\s+\w+',proofs)]
 for pattern,target in patterns:
  for match in re.finditer(pattern,clean):
   brace=clean.find('{',match.start());depth=0
   if brace<0:continue
   end=brace
   for end in range(brace,len(clean)):
    depth += (clean[end]=='{')-(clean[end]=='}')
    if depth==0:break
   first=clean[:match.start()].count('\n');last=clean[:end].count('\n')
   target.update(range(first,last+1))
 return tests,proofs

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--now',required=True);args=ap.parse_args()
 scope=read(OUT/'config/delta-scope-v1.json');base=scope['baseline'];seed=scope['scaffold_ref']
 assert git('rev-parse',base).decode().strip()==base
 assert git('rev-parse',seed).decode().strip()==seed
 paths=git('diff','--name-only',base,'--','sysmlv2/open_platform','hamr/microkit','bin','tests').decode().splitlines()
 paths=[p.removeprefix('mavlink_02/') for p in paths]
 paths+=git('ls-files','--others','--exclude-standard','--','bin','tests','hamr/microkit/bin').decode().splitlines()
 paths += ['hamr/microkit/bin/build.cmd',scope['requirements_final']]
 report=mp.parse_codegen_report(PROJECT/'hamr/microkit/reporting/codegen_report_sysml.json')
 resources=report['resources']
 inventory={}
 for line in (PROJECT/'reports/CR-02/final-validation/CR-02-final-artifact-inventory.md').read_text().splitlines():
  cells=[x.strip() for x in line.split('|')]
  if len(cells)>=5 and cells[2].startswith('`'):inventory[cells[2].strip('`')]=cells[3]
 rows=[]
 for n,line in enumerate((PROJECT/'reports/workflow-status.md').read_text().splitlines(),1):
  cols=[x.strip() for x in line.split('|')]
  if len(cols)<6:continue
  key,status,date,notes=cols[1:5]
  if 'CR-02' in key or key.startswith('CodeGen-HAMRUpgrade'):
   rows.append({'key':key,'status':status,'updated':date,'notes':notes,'source_line':n})
 assert any(r['key']=='ChangeExec(CR-02)' and r['status']=='done' for r in rows)
 files=[];details={};audit=[];excluded=[];template=[]
 for path in sorted(set(paths)):
  p=PROJECT/path
  reason=None
  if any(x in path.split('/') for x in ['target','build','.git','.venv','__pycache__']):reason='Build/cache output'
  if any(path.startswith(x) for x in ['hamr/microkit/vmm/','hamr/microkit/attestation/','hamr/microkit/reporting/']):reason='Vendor or generated reporting evidence'
  if p.name in ['Cargo.lock','.gitignore'] or path.endswith(('.bin','.patch')):reason='Mechanical lock/binary/housekeeping/archive'
  if path.endswith('.md') and path!=scope['requirements_final']:reason='Documentation; not source-priced'
  if path.startswith('bin/') and path!='bin/compile-r2u2.py':reason='Archived or out-of-scope helper'
  if not p.is_file():reason='Deleted path; deletion effort not modeled'
  if reason:excluded.append({'path':path,'reason':reason});continue
  try:new=p.read_text()
  except UnicodeError:excluded.append({'path':path,'reason':'Binary file'});continue
  old=at(base,path);reference=base
  if path==scope['requirements_final']:
   old=(PROJECT/scope['requirements_input']).read_text();reference=scope['requirements_input']
  old=old or ''
  added,removed=delta(old,new)
  if not added and not removed:continue
  lang=language(path);cls=classes(new,lang);lines=new.splitlines();oldcls=classes(old,lang)
  marker=mp.rust_marker_line_flags(lines) if lang=='rust' else [False]*len(lines)
  test_regions,proof_regions=rust_regions(new) if lang=='rust' else (set(),set())
  seed_text=at(seed,path) if not old and '/seL4_ModeManager_ModeManager/' in path else None
  seed_added=delta(seed_text,new)[0] if seed_text is not None else None
  component=next((x for x in ['ModeManager','MAVLinkFirewall','RxFirewall','TxFirewall','LowLevelEthernetDriver','firewall_core','mavlink_core'] if x in path),None)
  role=inventory.get(path,'')
  gumbo={}
  if lang=='sysml':
   for span in mp.extract_gumbo_spans(new):
    for i in range(span['start_line'],span['end_line']-1):gumbo[i]=span['owner']
  groups=collections.defaultdict(list)
  evidence=['reports/CR-02/final-validation/CR-02-final-artifact-inventory.md'] if role else []
  for i,line in enumerate(lines):
   prov='preserved_origin_unknown';typ='docs';step='CR-02.Unresolved';basis='fallback';mode='unknown'
   is_test=('/src/test/tests.rs' in path or path.startswith('tests/') or i in test_regions)
   if i not in added:
    prov='library';typ='inherited_baseline';step='Excluded baseline/input';basis='template_comparison'
   elif path==scope['requirements_final']:
    prov='developer_authored';typ='requirements_text';step='ChangeExec(CR-02).W1.Requirements';basis='template_comparison'
   elif lang=='sysml':
    prov='developer_authored';basis='path_rule';mode='not_codegen'
    typ='gumbo_contract' if i in gumbo or p.name=='GumboLib.sysml' else 'sysml_model'
    owner=gumbo.get(i)
    step=f'CR-02.CompGUMBOSpec({owner})' if typ=='gumbo_contract' and owner in ['ModeManager','MAVLinkFirewall','RxFirewall','TxFirewall'] else ('CR-02.SharedContracts' if typ=='gumbo_contract' else 'ChangeExec(CR-02).W1.SysModeling')
   elif marker[i]:
    prov='hamr_woven';typ='rust_contract_woven';step='CR-02.CodeGen';basis='marker';mode='woven'
   elif seed_added is not None and i not in seed_added:
    prov='hamr_template_retained';typ='rust_test_infra' if '/test/' in path else 'rust_infra';step='CR-02.CodeGen';basis='template_comparison';mode='generate_once'
    if p.name in ['Cargo.toml','rust-toolchain.toml']:typ='build_config'
    if p.name=='Makefile':typ='makefile'
   elif path=='hamr/microkit/bin/build.cmd':
    prov='developer_authored';typ='build_script';step='ChangeExec(CR-02).W1.SetupBuildScript';basis='workflow_status_note'
   elif path.startswith('tests/r2u2_monitor_probe/'):
    prov='developer_authored';typ='rust_test_code' if lang=='rust' else 'python_tool' if path.endswith('.py') else 'build_config';step='ChangeExec(CR-02).W1.R2U2Reporting';basis='path_rule'
    if path.endswith('.cargo/config.toml'):prov='hamr_generated';typ='build_config';step='CR-02.CodeGen'
   elif path=='bin/compile-r2u2.py':
    prov='developer_authored';typ='python_tool';step='ChangeExec(CR-02).W3.Build';basis='workflow_status_note'
   elif path.endswith('microkit.schedule.xml'):
    prov='developer_authored';typ='microkit_config';step='ChangeExec(CR-02).W3.SysSchedDef.1';basis='workflow_status_note'
   elif path.endswith('custom.mk') or p.name=='Makefile' and '/crates/' in path:
    prov='developer_authored';typ='makefile';step='ChangeExec(CR-02).W3.Build';basis='workflow_status_note'
   elif p.name in ['Cargo.toml','rust-toolchain.toml'] and '/crates/' in path:
    prov='developer_authored';typ='build_config';step='CR-02.Toolchain';basis='workflow_status_note'
   elif path.endswith('_app.rs') or path.endswith('/src/test/tests.rs') or 'Developer implementation' in role or 'Developer shared core' in role:
    prov='developer_authored';typ='rust_test_code' if is_test else 'proof_code' if i in proof_regions else 'rust_app_code';step=f'CR-02.CompDev({component}).'+('3' if is_test else '2');basis='workflow_status_note'
    if component=='firewall_core':step='ChangeExec(CR-02).W3.BoundsCompDev'
    if component=='LowLevelEthernetDriver':step='ChangeExec(CR-02).W3.DriverBounds'
   elif resources.get(path.removeprefix('hamr/microkit/')) is True or role.startswith(('Generated scaffold','Generated monitor')):
    prov='hamr_generated';typ='rust_infra' if lang=='rust' else 'c_infra' if lang=='c' else 'microkit_config' if lang=='xml' else 'makefile' if lang=='makefile' else 'build_config';step='CR-02.CodeGen';basis='codegen_report' if resources.get(path.removeprefix('hamr/microkit/')) is True else 'workflow_status_note';mode='overwrite'
    if 'GUMBOX' in path or '/test/util/' in path:typ='rust_test_infra'
   # Newly added syntax wrappers/comments are still inventoried; only code is priced.
   groups[(prov,typ,step,basis,mode)].append(i)
  pairs=[]
  for (prov,typ,step,basis,mode),indices in groups.items():
   bucket=mp._bucket(typ,mode,prov,basis,'derived' if prov!='library' else 'exact',{'key':step,'granularity':'workflow_only' if step.startswith('CR-02.') else 'inferred_step'},mp.tally([cls[i] for i in indices]),component=component)
   pairs.append((bucket,indices))
  mp._finalize_file(files,details,path,lang,cls,pairs,False)
  audit.append({'path':path,'baseline_reference':reference,'baseline_sha256':sha(old.encode()),'final_sha256':sha(new.encode()),'baseline_lines':len(old.splitlines()),'final_lines':len(lines),'added_or_replacement_ranges':mp._compress_ranges(sorted(added)),'removed_ranges':mp._compress_ranges(sorted(removed)),'added_sloc':sum(cls[i]=='code' for i in added),'removed_sloc':sum(oldcls[i]=='code' for i in removed),'ownership_evidence':evidence,'inventory_role':role,'historical_scaffold_ref':seed if seed_text is not None else None})
  if seed_text is not None:
   seedcls=classes(seed_text,lang);_,seed_removed=delta(seed_text,new)
   retained=sum(cls[i]=='code' for i in added if i not in seed_added)
   template.append({'file':path,'supplied':seedcls.count('code'),'retained':retained,'replaced':sum(seedcls[i]=='code' for i in seed_removed)})
 # Derive aggregate step status ONLY from selected CR-02 workflow rows.
 for key in sorted({b['workflow']['key'] for f in files for b in f['buckets']}):
  if not key.startswith('CR-02.'):continue
  matches=[]
  token=key.removeprefix('CR-02.')
  if token.startswith('CompDev('):
   comp,sub=re.match(r'CompDev\((.*?)\)\.(\d)',token).groups()
   matches=[r for r in rows if f'CompDev({comp})' in r['key'] and r['key'].endswith('.'+sub)]
  elif token.startswith('CompGUMBOSpec('):matches=[r for r in rows if token in r['key']]
  elif token=='CodeGen':matches=[r for r in rows if ('CodeGen' in r['key']) and r['status']=='done']
  elif token=='SharedContracts':matches=[r for r in rows if 'CompGUMBOSpec' in r['key']]
  elif token=='Toolchain':matches=[r for r in rows if 'CoreRegression' in r['key']]
  rows.append({'key':key,'status':'done' if matches and all(r['status'] in ['done','n/a'] for r in matches) else 'unknown','notes':'Aggregated CR-02 artifact step; spanning original and correction waves. No additional scope inferred.','source_rows':[r['key'] for r in matches]})
 config=read(OUT/'config/effort-model-v1.json');rules=config['activity_overheads']['selection_rules'];config['activity_overheads']['steps']=[]
 for r in rows:
  for rule in rules:
   if r['status']=='done' and re.search(rule['regex'],r['key']):
    config['activity_overheads']['steps'].append({'step_pattern':r['key'],'hours':rule['hours'],'activity':rule['activity']});break
 write(OUT/'config/effort-model-resolved.json',config)
 manifest=read(PROJECT/'experiment-reports/effort-manifest.json');manifest.update(experiment_id='mavlink_02-CR-02',workflow='ChangeExec(CR-02)',metrics_cover_full_run=False)
 manifest['known_exclusions']=[
  'CR-02 only: baseline 043d574 to current working tree; unchanged lines, all CR-01/vendor work and post-session reporting work excluded from priced delta.',
  'Final requirements compared with initial supplied CR-02 draft, not empty text. Revisions are collaborative/developer-owned work, not claimed as agent-authored. Intermediate versions are not summed.',
  'Net retained additions/replacements are a proxy, not actual authoring time; removed-only work, debugging, reverted experiments, repeated edits and review effort are not fully modeled.',
  'Historical ModeManager scaffold at b68c70a provides limited provenance, not a pristine final-model regeneration. Other generated classifications use markers, generator report and contemporaneous artifact inventory.',
  'Scope is partial: the session includes interaction, troubleshooting and closeout omitted from source-based effort; subagents are excluded, costs unavailable, ratios suppressed.',
  'concept_quote retains the skill-required historical conops section 0; current CR-02 concept is quoted separately in config/delta-scope-v1.json.',
  'Unchanged audited lines use library provenance only as the v1 excluded category; they are not claimed to be third-party libraries. They never contribute hours.',
  'Fixed tool overheads reuse uncalibrated v1 amounts for explicit completed CR-02 steps; not observed command durations. Build-script setup is counted once.']
 write(OUT/'effort-manifest.json',manifest)
 original=read(PROJECT/'experiment-reports/project-measures.json')
 measures={k:copy.deepcopy(original[k]) for k in ['schema_version','inputs']}
 measures.update(generated=args.now,project={'name':'mavlink_02-CR-02','path':str(PROJECT)},manifest_ref=str(OUT/'effort-manifest.json'),files=files,template_accounting=template,excluded=excluded,caveats=manifest['known_exclusions'])
 measures['inputs']['generation_baseline']={'present':False,'usable':False,'provenance':{'limited_historical_scaffold':seed,'scope':'new ModeManager resources only'}}
 measures['workflow_status']={'profile':'audited','rows':rows,'components':['ModeManager','MAVLinkFirewall','RxFirewall','TxFirewall','LowLevelEthernetDriver'],'workflows':{'ChangeExec(CR-02)':'done'},'completed':['ChangeExec(CR-02)']}
 measures['assurance']={'tests':{'defined_static':{},'runtime':{}},'coverage':{},'component_verus':{},'system_proof':{},'integration_check':{},'audit_findings':{},'generated_resources':{}}
 for name in ['tests','verification']:
  src='reports/CR-02/final-validation/'+('CR-02-final-test-results.json' if name=='tests' else 'CR-02-full-verification-results.json')
  measures['assurance']['tests']['runtime' if name=='tests' else 'verification_runs']={'source':src,'records':read(PROJECT/src)}
 roll={'by_provenance':{},'by_artifact_type':{},'by_component':{},'by_step_bucket':{},'total_sloc':0}
 for f in files:
  for b in f['buckets']:
   roll['total_sloc']+=b['sloc']
   for kind,key in [('by_provenance',b['provenance']),('by_artifact_type',b['artifact_type']),('by_component',b['component'])]:
    if key is None:continue
    v=roll[kind].setdefault(key,dict(sloc=0,comment_lines=0,blank_lines=0,buckets=0))
    for c in ['sloc','comment_lines','blank_lines']:v[c]+=b[c]
    v['buckets']+=1
   v=roll['by_step_bucket'].setdefault(b['workflow']['key'],{'sloc':0,'granularity':b['workflow']['granularity'],'by_provenance':{}});v['sloc']+=b['sloc'];v['by_provenance'][b['provenance']]=v['by_provenance'].get(b['provenance'],0)+b['sloc']
 measures['rollups']=roll
 allocation=mp.build_line_allocation(files,details,args.now,measures['project']);allocation['measures_ref']='project-measures.json'
 write(OUT/'project-measures.json',measures);write(OUT/'line-allocation.json',allocation)
 write(OUT/'delta-audit.json',{'schema_version':1,'scope':scope,'head':git('rev-parse','HEAD').decode().strip(),'files':audit,'excluded':excluded,'config_sha256':{p.name:sha(p.read_bytes()) for p in sorted((OUT/'config').glob('*.json'))},'script_sha256':sha(Path(__file__).read_bytes())})
 estimate=ee.estimate(measures,manifest,OUT/'config/effort-model-resolved.json',OUT/'config/counterfactual-v1.json',PROJECT/'experiment-reports/session-metrics.json',args.now,OUT/'project-measures.json')
 write(OUT/'effort-estimate.json',estimate)
 render(estimate,measures,audit,scope,args.now)
 validate()
 print('CR-02 effort report, delta evidence and schema/partition checks complete.')


def render(e,m,audit,scope,now):
 esc=html.escape
 total=e['comparisons'][0]['values']['modeled_human'];sess=e['session_totals'];profiles=list(total)
 counts={k:v['sloc'] for k,v in m['rollups']['by_provenance'].items()};auth=counts.get('developer_authored',0);gen=sum(counts.get(k,0) for k in ['hamr_generated','hamr_woven','hamr_template_retained']);unk=counts.get('preserved_origin_unknown',0)
 sections=[]
 def para(s):sections.append(('p',s))
 def heading(s):sections.append(('h2',s))
 def table(headers,rows):sections.append(('table',(headers,rows)))
 para('UNCALIBRATED MODEL — CR-02 only. Human-equivalent coding, specification, test and configuration effort for the retained change, using named placeholder profiles. No measured savings, agent productivity ratio or calibrated prediction is claimed.')
 para(f"Observed scope: approved baseline {scope['baseline']} to the current working tree. {len(audit)} changed text artifacts audited; {auth:,} authored/revised SLOC, {gen:,} generated/woven/template SLOC and {unk:,} unresolved SLOC in the retained delta. Unchanged baseline/input lines are excluded from all modeled effort.")
 heading('Human-equivalent CR-02 effort — Modeled')
 table(['Profile','Hours','8-hour person-days','Cost','Hourly rate'],[[p,f"{v['hours']:,.2f}",f"{v['hours']/8:,.2f}",f"${v['cost_usd']:,.2f}",'$150' if p=='experienced_sel4' else '$110'] for p,v in total.items()])
 para('Counterfactual: a human performs the same retained CR-02 changes using HAMR, not rebuilding the pre-existing system. Rates use the skill’s linear SLOC model; requirements revisions include developer collaboration. Generated changes are excluded from the human HAMR-path total. Fixed technical tool overheads are modeled separately per completed workflow step; no observed tool duration is converted into human hours.')
 para(f"Observed primary session: {sess['active_seconds']/3600:.2f} active hours, {sess['wall_clock_seconds']/3600:.2f} wall-clock hours; {sess['tokens']['output']:,} output tokens. Active time uses the adapter’s 300-second gap rule, not human labor time. Actual billed and list-price-equivalent agent costs are unavailable. Subagents are excluded. Partial scope: no ratio rendered.")
 heading('Per-workflow-step delta — Observed quantities / Modeled hours')
 data=[]
 for st in e['steps']:
  if st['step_key']=='Excluded baseline/input':continue
  p=st['measures']['by_provenance'];hours=[st['human_estimates'][x]['hours'] for x in profiles]
  if not st['measures']['sloc_total'] and not any(hours):continue
  data.append([st['step_key'],st['status'] or 'unknown',str(p.get('developer_authored',0)),str(sum(p.get(k,0) for k in ['hamr_generated','hamr_woven','hamr_template_retained'])),str(p.get('preserved_origin_unknown',0)),*[f'{h:.2f}' for h in hours]])
 table(['Step','Status','Authored Δ SLOC','Generated Δ SLOC','Unknown Δ SLOC',*profiles],data)
 para('CR-02.CompDev and CompGUMBOSpec aggregate retained changes across original W1/W2 and W3 correction/logging work, because final hunks do not identify one unique producing pass. Exact source workflow rows are retained in project-measures.json. Activity-only rows retain their full original step names. A step’s zero source count does not mean it required no work.')
 heading('Authored delta by artifact class — Observed')
 by=collections.Counter()
 for f in m['files']:
  for b in f['buckets']:
   if b['provenance']=='developer_authored':by[b['artifact_type']]+=b['sloc']
 table(['Artifact class','Retained added/replacement SLOC'],[[k,str(v)] for k,v in sorted(by.items())])
 heading('Generated change replacement sensitivity — Modeled, separate from effort above')
 table(['Profile','Scenario','Modeled manual-equivalent hours','Modeled cost'],[[p,scenario,f"{v['hours']:,.2f}",f"${v['cost_usd']:,.2f}"] for p,sc in e['hamr_generated_value']['estimates'].items() for scenario,v in sc.items()])
 para('This sensitivity prices only changed generated lines, including generator-version formatting/API churn. Nominal assumes a manual equivalent of each line; conservative uses half. It is not a benefit, avoided labor measurement, net saving or addition to the HAMR-path human total.')
 heading('Counting and provenance')
 para(scope['method'])
 para(f"Requirements use final {scope['requirements_final']} versus initial supplied {scope['requirements_input']}; only retained revisions are priced. They are not all agent authorship. The CR-02 concept is: {scope['concept_quote']}")
 para('For existing applications/tests, only final changes relative to the approved baseline are eligible. Marker regions are HAMR-woven; report-owned or contemporaneously inventoried generated scaffolds are generated. New ModeManager retained template lines are identified from b68c70a; later non-marker implementation/test changes are attributed using the final artifact inventory. Unknown ownership stays unpriced. The historical scaffold is not a pristine final-model baseline.')
 para(f"Observed deleted/replaced old SLOC: {sum(x['removed_sloc'] for x in audit):,}. Old-side deletions are recorded but not priced a second time. Reordered identical lines and leading/trailing whitespace-only changes are excluded. Intermediate overwritten work and the reverted +200 ms experiment are not reconstructed. Line-allocation.json partitions every audited final file into excluded input and delta buckets; delta-audit.json retains both-side ranges and hashes.")
 heading('Evidence and limitations')
 para('Current recorded acceptance: 67 application/core tests plus 2 driver helper tests; ModeManager/Rx/Tx/MAVLink/firewall_core/mavlink_core proofs 9/28/16/69/39/38 with zero errors. Full target loader build passes. Driver full host suite is unavailable and driver application proof is disabled. CR-02-HW-01 remains High/Open, accepted for later work; its future fix is outside this estimate. Source: reports/CR-02/final-validation/CR-02-final-validation.md. No build, test, codegen or hardware run was performed for this report.')
 for caveat in e['caveats']:para(caveat)
 para('This is a retained-change coding/unit-debug model, not a complete labor estimate: deleted-only/reverted work, elapsed troubleshooting, audits, meetings, manual testing and board bring-up are not fully priced. The 1267 metrics parser warnings limit friction counts. No duration per workflow step is observed. The two profiles are alternative assumptions, not a confidence interval.')
 heading('Reproduction and audit files')
 para(f'python3 experiment-reports/CR-02-effort/refresh.py --now {now}')
 para('Inputs and outputs are isolated under experiment-reports/CR-02-effort/. Original snapshot and August reports remain intact. v1 skill schemas validate manifest, measures, allocation and estimate. Model/counterfactual hashes are in effort-estimate.json; scope/config/script hashes and per-file input hashes are in delta-audit.json. Rate changes belong in config/effort-model-v1.json, not the script.')
 title='CR-02-only effort report — '+now[:10]
 md=['# '+title,''];ht=['<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>'+esc(title)+'</title><style>body{max-width:1100px;margin:32px auto;padding:0 20px;font:16px/1.55 system-ui;color:#17212b;background:#fafaf8}h1{font-size:28px}h2{font-size:21px;margin-top:32px}table{border-collapse:collapse;width:100%;font-size:13px}th,td{padding:8px;border-bottom:1px solid #ccc;text-align:left;overflow-wrap:anywhere}th{background:#e9eef2}p{overflow-wrap:anywhere}.table{overflow-x:auto}body>p:first-of-type{padding:15px;background:#fff2cf;border-left:5px solid #ba7e00}</style><body><h1>'+esc(title)+'</h1>']
 for typ,value in sections:
  if typ=='table':
   headers,rs=value;md+=['| '+' | '.join(headers)+' |','|'+'|'.join(['---']*len(headers))+'|']+['| '+' | '.join(str(c).replace('|','\\|').replace('\n',' ') for c in r)+' |' for r in rs]+['']
   ht.append('<div class="table"><table><thead><tr>'+''.join('<th>'+esc(h)+'</th>' for h in headers)+'</tr></thead><tbody>'+''.join('<tr>'+''.join('<td>'+esc(str(c))+'</td>' for c in r)+'</tr>' for r in rs)+'</tbody></table></div>')
  else:md+= [('## ' if typ=='h2' else '')+value,''];ht.append(f'<{typ}>'+esc(value)+f'</{typ}>')
 stem='EFFORT-CR-02-'+now[:10];(OUT/(stem+'.md')).write_text('\n'.join(md));(OUT/(stem+'.html')).write_text('\n'.join(ht)+ '</body></html>\n')

def validate():
 import jsonschema
 for filename,schema in [('effort-manifest','experiment-manifest'),('project-measures','project-measures'),('line-allocation','line-allocation'),('effort-estimate','effort-estimate')]:
  jsonschema.validate(read(OUT/(filename+'.json')),read(SKILL/f'schema/{schema}-v1.schema.json'))
 a=read(OUT/'line-allocation.json');audit={x['path']:x for x in read(OUT/'delta-audit.json')['files']}
 for f in a['files']:
  changed={i for a,b in audit[f['path']]['added_or_replacement_ranges'] for i in range(a,b+1)}
  seen=[];selected=set()
  for b in f['buckets']:
   owned=[i for a,z in b['ranges'] for i in range(a,z+1)];seen.extend(owned)
   assert len(owned)==b['sloc']+b['comment_lines']+b['blank_lines']
   if b['provenance']!='library':selected.update(owned)
  assert sorted(seen)==list(range(1,f['line_count']+1))
  assert selected==changed, f['path']
 e=read(OUT/'effort-estimate.json');assert e['comparisons'][0]['ratio'] is None
 assert all('CR-01' not in s['step_key'] for s in e['steps'])
 print('Four v1 schemas and exact delta/full-file line partitions PASS:',len(a['files']),'files')

if __name__=='__main__':main()
