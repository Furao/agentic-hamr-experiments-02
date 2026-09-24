#!/usr/bin/env python3
"""Run the bundled effort pipeline and retain this change experiment's scope caveats."""
import argparse
import html
import json
import os
from pathlib import Path
import subprocess
import sys

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--now', required=True, help='ISO timestamp for reproducible output')
args = parser.parse_args()
reports = Path(__file__).resolve().parent
project = reports.parent
skill = project.parent / '.agents/skills/h-wf-effort'
env = dict(os.environ, PYTHONDONTWRITEBYTECODE='1')
subprocess.run([sys.executable, str(skill / 'scripts/run_effort_report.py'),
                '--project', str(project), '--now', args.now], check=True, env=env)
manifest = json.loads((reports / 'effort-manifest.json').read_text())
estimate_path = reports / 'effort-estimate.json'
estimate = json.loads(estimate_path.read_text())
notes = manifest['known_exclusions'] + [
    'Observed SLOC is a snapshot inventory, not newly produced session output. '
    'Vendor/VMM content contributes to the large unknown and unattributed buckets; '
    'the partial codegen report is not itself evidence of a failed generation.',
    'Workflow parsing misses some CR-02-qualified steps and retains older CR-01 '
    'assurance values (for example MAVLink 14/0 and Rx 27/0). A dash or zero modeled '
    'overhead does not mean CR-02 work was absent or incomplete. Final recorded '
    'proof counts are ModeManager 9/0, Rx 28/0, Tx 16/0, MAVLink 69/0, '
    'firewall_core 39/0, mavlink_core 38/0; 67 application/core tests and 2 driver '
    'helper tests pass. See reports/CR-02/final-validation/CR-02-final-validation.md.'
]
estimate['caveats'] = list(dict.fromkeys(estimate['caveats'] + notes))
assert all(c['scope_alignment'] == 'partial' and c['ratio'] is None
           for c in estimate['comparisons'])
estimate_path.write_text(json.dumps(estimate, indent=2, ensure_ascii=False) + '\n')
subprocess.run([sys.executable, str(skill / 'scripts/render_report.py'),
                '--estimate', str(estimate_path), '--measures',
                str(reports / 'project-measures.json')], check=True, env=env)
notice = (
    'CHANGE-REQUEST SCOPE — This is a current-project snapshot estimate under the '
    'greenfield-oriented v1 model, not an incremental CR-02 effort estimate. '
    'Inherited CR-01 and vendor content are present; current CR-02 requirements '
    'under action-requests are excluded by the measurer, while retired requirements '
    'are counted. Some workflow status and assurance values below are historical '
    'or unrecognized by the parser. Unknown-provenance code is excluded from modeled '
    'headline effort, so those totals are not a complete implementation estimate. '
    'The observed session covers CR-02 only. Ratios are suppressed; no savings are '
    'measured. All hours and costs beyond observed session metrics are an '
    'UNCALIBRATED MODEL. See the data-quality caveats for details.'
)
stem = f"EFFORT-{estimate['project']['name']}-{estimate['generated'][:10]}"
md = reports / (stem + '.md')
text = md.read_text()
heading, rest = text.split('\n', 1)
text = heading + '\n\n> **Scope limitation.** ' + notice + '\n' + rest
text = text.replace('Modeled human effort for the same authored artifacts plus tool-running overheads:',
                    'Modeled human effort for the classified project-snapshot artifacts plus parsed tool-running overheads (different scope from the session):')
text = text.replace('Per-step agent actuals do not exist in Wave 1',
                    'Per-step agent actuals are unavailable in this pipeline')
md.write_text(text)
page = reports / (stem + '.html')
text = page.read_text().replace('<body>', '<body>\n<aside role="note" style="border:2px solid #eda100;padding:1rem;background:#fdf3d8"><strong>Scope limitation.</strong> ' + html.escape(notice) + '</aside>', 1)
page.write_text(text)
print('Applied change-request scope caveats; modeled numbers and rate configs unchanged.')
