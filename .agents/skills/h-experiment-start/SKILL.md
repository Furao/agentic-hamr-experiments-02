---
name: h-experiment-start
description: Apply the independent-trial protocol only when the current user message explicitly invokes `$h-experiment-start` or `/h-experiment-start`. Repository location, HAMR development, lifecycle language, plain-name mentions, prior turns, other skills, and automated workflows never authorize it.
---

# HAMR experiment protocol

## Authorization gate

Before taking any action, inspect only the current user message. Continue only when that
message contains the literal invocation `$h-experiment-start` or
`/h-experiment-start`.

Authorization applies only to that current message. Do not inherit it from an earlier turn,
another skill, or an automated workflow. Repository location, ordinary HAMR work,
development-completion or end-of-session language, and the plain name
`h-experiment-start` do not authorize execution.

Without authorization, stop immediately. Do not run scripts, apply this protocol, create or
modify artifacts, or cause any workflow effect.

After the authorization gate passes, apply these rules without prescribing a lifecycle:

1. Build only from the developer's concept and the shared HAMR context named by
   `HAMR_AGENT_CONTEXT`. Do not read sibling `ex*` folders, cross-experiment Git history,
   or the repository-root `reports/` folder. Those sources contain prior decisions and
   contaminate an independent trial. `h-wf-compare` is the after-the-fact exception.
2. Watch for workflow, documentation, HAMR toolchain, harness, policy, and environment
   friction throughout development. Report material blockers when they arise and preserve
   evidence for assessment.
3. Record decisions and assumptions, including rapid-profile `ASSUMED:` notes, so the run
   is reproducible and comparable.
4. Put every generated artifact and experiment report in the current experiment folder,
   never in the shared HAMR context repository.

When development is complete, remind the developer that transcription is a separate action
which requires its own explicit `$h-transcribe` or `/h-transcribe` invocation in the
then-current user message. Do not invoke or run transcription automatically. Run
`h-wf-assess` in the separate assessment session.
