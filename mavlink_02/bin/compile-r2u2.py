#!/usr/bin/env python3
"""Compile the generated specification with the project's pinned R2U2 CLI."""
import argparse
from pathlib import Path
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cli', default='r2u2_cli')
    args = parser.parse_args()
    version = subprocess.check_output([args.cli, '--version'], text=True).strip()
    if version != 'r2u2_cli 4.2.4':
        parser.error(f'Expected r2u2_cli 4.2.4, got {version!r}')
    crate = Path(__file__).resolve().parents[1] / 'hamr/microkit/crates/seL4_MAVLinkFirewall_MAVLinkFirewall'
    component = crate / 'src/component'
    (crate / '.cargo').mkdir(exist_ok=True)
    # HAMR's map includes comment lines that the CLI map reader does not accept.
    with tempfile.TemporaryDirectory(prefix='cr02-r2u2-') as temporary:
        mapping = Path(temporary) / 'spec.map'
        mapping.write_text(''.join(line for line in (component / 'spec.map').read_text().splitlines(True)
                                   if not line.startswith('--')))
        subprocess.run([args.cli, 'compile', '-o', '.', '-b', '../../.cargo/config.toml',
                        'spec.c2po', str(mapping)], cwd=component, check=True)
    print(f'Compiled MAVLinkFirewall specification with {version}', flush=True)


if __name__ == '__main__':
    main()
