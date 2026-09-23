# Codegen workarounds

`4bc9a9a-r2u2-false-verdict.patch` captures the exact file diff from commit
`4bc9a9ae60daefad311bcf23546ee8d4e7468c1b` with project-relative paths.
It preserves visibility of a false verdict in the current monitor step even when
a later true verdict replaces the cached value. It modifies generated logging;
it does not provide the production one-time timeout reporter or fix alert routing.

After every codegen invocation run:

```sh
python3 bin/apply-codegen-workarounds.py
```

The helper checks for an already-applied patch, checks forward applicability, then
applies it without partial rejects. A conflict exits nonzero for review. It works
from any current directory and locates this project's containing Git repository.
Direct CLI codegen does not automatically invoke this helper; AGENTS.md requires
the post-generation step in the agent workflow.

Validate with the isolated generated-monitor probe. Reassess this exact patch if
HAMR changes its reporting code or the number of monitor specifications changes;
the captured fix has a one-element per-specification array.
