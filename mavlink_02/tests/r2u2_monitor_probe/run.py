#!/usr/bin/env python3
"""Compile the current HAMR specification and test the unchanged generated monitor."""
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

probe = Path(__file__).resolve().parent
project = probe.parent.parent
generated = project / 'hamr/microkit/crates/seL4_MAVLinkFirewall_MAVLinkFirewall/src/component'
compiler = os.environ.get('R2U2_CLI') or shutil.which('r2u2_cli')
if not compiler:
    raise SystemExit('Install r2u2_cli 4.2.4 and put it on PATH, or set R2U2_CLI.')
version = subprocess.check_output([compiler, '--version'], text=True).strip()
if version != 'r2u2_cli 4.2.4':
    raise SystemExit(f'Expected r2u2_cli 4.2.4, got {version!r}')
bounds = probe / '.cargo/config.toml'
bounds.parent.mkdir(exist_ok=True)
with tempfile.TemporaryDirectory(prefix='cr02-monitor-') as tmp:
    mapping = Path(tmp) / 'spec.map'
    mapping.write_text('\n'.join(line for line in (generated / 'spec.map').read_text().splitlines()
                                 if not line.startswith('--')) + '\n')
    subprocess.run([compiler, 'compile', '-o', str(generated), '-b', str(bounds),
                    str(generated / 'spec.c2po'), str(mapping)], check=True)
raise SystemExit(subprocess.run(['cargo', '+stable', 'test', '--locked', '--offline', '--', '--nocapture'], cwd=probe).returncode)
