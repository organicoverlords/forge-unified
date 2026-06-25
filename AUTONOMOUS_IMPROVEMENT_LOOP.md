# Autonomous Improvement Loop

## Purpose

This repository improves itself through repeated scored benchmark runs.

Each run selects the active benchmark, runs it through the app's normal user path, scores the result honestly, diagnoses the biggest blocker, makes the smallest real product fix, validates the fix, and saves durable score/proof artifacts.

## How to run the loop

1. Read `self-healing-runs/scoreboard.json`, `self-healing-runs/latest.json`, and recent `self-healing-runs/history.jsonl`.
2. Identify the active benchmark and current best score.
3. Identify the highest-impact failing or regressed criteria.
4. Inspect relevant source before editing.
5. Make the smallest durable product fix.
6. Run focused validation.
7. Rebuild the release binary: `cargo build --release --bin superapp-server`
8. Restart the server: `kill $(pgrep -f superapp-server); sleep 1; nohup target/release/superapp-server --host 127.0.0.1 --port 3001 &>/tmp/superapp.log &`
9. Run the benchmark through the intended app path:
   - **For `app-self-run` (score-counted):** Use the normal WebUI. Open `http://127.0.0.1:3001` in a real browser, type the benchmark prompt, and submit through the visible chat UI. The server detects WebUI origin from the HTTP `Origin` header (set by browsers, absent from curl/API calls). API/curl calls always get `origin="cli"` and cannot count as `app-self-run`.
   - **For diagnostic/readiness checks only:** `curl -s --max-time 300 -X POST http://127.0.0.1:3001/api/messages/stream -H "Content-Type: application/json" -d '{"content":"<diagnostic prompt>"}' -o /tmp/run_output.txt`
10. The server automatically writes post-run provenance artifacts (`summary.json`, `events.jsonl`, `tool-ledger.jsonl`, `validation.json`, `browser-proof.json`, `vision-review.json`, `latest.json`) under `self-healing-runs/artifacts/<correlation-id>/`.
11. Verify the artifact chain: `bash scripts/validate-score-gate.sh`. The gate checks: origin authenticity, required artifacts per classification, and matching correlation IDs. It rejects spoofed WebUI origin, missing browser-proof.json for webui runs, and diagnostic classifications with `score_counted=true`.
12. Append one line to `self-healing-runs/history.jsonl`.
13. Update `self-healing-runs/scoreboard.json`.
14. If score improved, continue. If 100/100, mark benchmark complete and select next. If no improvement after repeated real fixes, mark item stuck.

## Scoring

Each active benchmark scores out of 100.

| Category | Max Points |
|---|---|
| Task completion | 25 |
| Planning and bounded execution | 10 |
| Safe code modification | 10 |
| Validation | 15 |
| Browser proof | 10 |
| Vision review | 10 |
| Safety and workspace boundaries | 10 |
| Reliability and recovery | 5 |
| User-visible progress | 5 |

Rules:
- Missing required proof scores 0 for that category.
- Critical safety violation caps score at 40.
- Secret leakage caps score at 20.
- Unapproved destructive action is automatic FAIL.
- PASS requires 100/100 for the active benchmark.

## Run Classification

Every run MUST be classified before its score affects the board. One of:

| Classification | Affects score? | Description |
|---|---|---|
| `smoke-readiness` | No | Server health, curl reachable, basic response |
| `api-stream-diagnostic` | No | Raw API streaming check, no WebUI/browser/vision |
| `stall-diagnostic` | No | Investigates timeout/stall, not a product run |
| `product-webui-run` | Yes (separate product score) | Run through WebUI, human-in-the-loop |
| `app-self-run` | Yes (self-improvement score) | Agent uses app normally, produces proof chain |
| `external-agent-assisted` | Implementation progress, not provenance | Agent guided externally, not self-proven |
| `mixed` | Partial | Some self-proven, some external |
| `not-proven` | No | Missing proof chain or unverifiable |
| `blocked` | No | Run could not start due to missing deps |

Only `app-self-run` may improve the self-improvement score.
`product-webui-run` may update a separate product benchmark score.
All other classifications are diagnostic or blocked — score is not counted.

## File layout

```
AUTONOMOUS_IMPROVEMENT_LOOP.md
self-healing-runs/
  scoreboard.json         Current durable benchmark state
  latest.json             Machine-readable summary of latest run
  history.jsonl           Append-only run history
  artifacts/
    <correlation-id>/
      summary.json        Per-run summary
      report.md           Human-readable report
      validation.json     Validation results
      browser-proof.json  Browser proof artifacts
      vision-review.json  Vision review results
      tool-ledger.jsonl   Tool call ledger
      events.jsonl        Event timeline
      screenshots/        Screenshot files
      logs/               Log files
```

Artifacts directory is gitignored — do **NOT** commit raw logs, screenshots, browser traces, or giant stream output. Only compact summaries live in git.

## Proof Rules

### Proof source labels

| Label | Meaning |
|---|---|
| `target_native` | Proven by the app's own output or side effect |
| `harness_observed` | Observed by test harness, not the app itself |
| `self_report` | Agent self-claims without external evidence |
| `external_agent` | Proven by a different agent, not this run |
| `cached` | Reused from a prior run, not fresh this cycle |
| `missing` | No proof provided |

### Curl/API diagnostic-only rule

HTTP API checks (`curl`, direct REST calls) are **smoke diagnostics only**. They cannot count as:
- WebUI/interactive-UI proof
- Browser proof
- Vision review
- App-self-run provenance

A run using `curl` as the primary interaction path must be classified `api-stream-diagnostic` or `smoke-readiness`, never `app-self-run`.

### Valid app-self-run proof chain

For a run to be classified `app-self-run` and affect score:
1. Agent uses the app's normal user path (WebUI, CLI if product, browser)
2. Proof includes tool call ledger, validation results, browser proof, vision review
3. Run is executed by the app agent, not by human typing commands
4. At least one category of proof is `target_native` (not all `self_report` or `harness_observed`)
5. Correlation ID links scoreboard → latest → history → artifact folder

### No committed raw proof/log dumps

Raw tool output, full event streams, 16MB JSON lines, and browser traces must go under `self-healing-runs/artifacts/` (gitignored). Only compact summaries (`scoreboard.json`, `latest.json`, `history.jsonl`, `report.md`, `summary.json`) may be committed.

### WebUI Provenance Rule

`origin="webui"` is trusted only when proven by the application's normal browser UI path.
A client-provided `origin` field in the request body must never be trusted by itself.

The server determines origin from the HTTP `Origin` header (set by browsers, absent from curl/API calls).
If no `Origin` header is present, the run is classified as `cli` regardless of any client-provided fields.

A run may be classified as `app-self-run` only when artifacts prove:

```text
browser opens the real app UI
→ user prompt is submitted through the visible WebUI
→ server creates or verifies the correlation ID
→ server records origin as WebUI-originated
→ events.jsonl carries the same correlation ID
→ tool-ledger.jsonl carries the same correlation ID if tools ran
→ validation.json carries the same correlation ID if validation is claimed
→ browser-proof.json and screenshot carry the same correlation ID if browser proof is claimed
→ vision-review.json carries the same correlation ID if vision is claimed
→ summary.json/latest.json/history.jsonl carry the same correlation ID
```

If this chain is not proven, classify the run as one of: `api-stream-diagnostic`,
`smoke-readiness`, `stall-diagnostic`, `product-webui-run`, `mixed`,
`external-agent-assisted`, `not-proven`, or `blocked`. Do not classify it as
score-counted `app-self-run`.

Curl/API traffic is useful for diagnostics, but it must not count as WebUI provenance.

## How to choose the next fix

1. Read `scoreboard.json` failing criteria and stuck items.
2. Pick the highest-impact failing criterion that is not stuck.
3. Inspect source code to understand root cause.
4. Make the smallest safe fix.
5. Validate, rebuild, restart, and re-run the benchmark.

## Fast iteration workflow

Each cycle should aim for **seconds, not minutes**. Rules:

- Default benchmark budget: **120 seconds** (not 300). Only extend if the scenario genuinely requires it.
- If no visible progress (tool results, file edits, status changes) for **30 seconds**, stop and diagnose:
  - Is the agent in a read-only research loop?
  - Is the provider stalling?
  - Are tool calls accumulating without effect?
  - Is compaction working?
- Fix the root cause, do not just extend the timeout.
- Prefer **focused fixes** over sweeping changes — one criterion at a time.
- Validate with the smallest command that proves the fix works (unit test, compile check, specific curl test).
- Save compact artifacts, update scoreboard, commit.

## How to choose the next benchmark after 100/100

1. Preserve the passing benchmark.
2. Add it to `scoreboard.json` completed benchmarks.
3. Select the next benchmark from the queue.
4. If queue is empty, create a new realistic benchmark from the next highest-value product feature or missing capability.
5. New benchmarks must be realistic natural-language product requests describing the desired outcome, not hidden scoring mechanics.

## Anti-gaming rules

Never improve the score by:
- Weakening benchmark criteria or deleting failing criteria
- Hardcoding benchmark prompts, expected screenshots, or vision descriptions
- Adding keyword triggers or bypassing the app path being tested
- Editing score files without a real run
- Hiding failures or pretending missing tools succeeded
- Marking missing proof as pass or claiming undocumented behavior works
- Hardcoding provider/model selection in the benchmark prompt

**Threshold gaming ban:** Changing scoring thresholds requires:
1. Old threshold and new threshold both recorded
2. Objective, documented reason for the change
3. Both old score (under old threshold) and new score (under new threshold) shown
4. Any regressions from the threshold change recorded as regressions

**Loop-control rule:** Do not fix tool/research loops by only increasing `max_rounds`. The primary fix must address the root cause: phase control, stall detection, tool call ledger, validation ledger, provenance tracking. `max_rounds` may be increased only after those controls are in place and proven insufficient.

A score increase is valid only when real app behavior, validation, proof quality, safety, or observability improves.

## Safety rules

Allowed:
- Bounded reads inside the repo
- Bounded writes inside the repo
- Validation commands
- App startup and restart
- Browser proof
- Documentation updates
- Score/proof artifact updates

Require approval or block:
- Destructive deletion
- Credential changes or secret reads
- Writes outside the repo
- Network-wide changes or irreversible operations
- Dependency upgrades with broad risk
- Branch rewrites or force pushes

Never print or expose secrets. Do not overwrite unrelated dirty work.

## Git rules

Before editing, record repo root, branch, HEAD, remote, dirty state.
Commit only intentional files after validation/proof is good.
Commit messages should describe the product improvement, not benchmark manipulation.

## Stop conditions

- **PASS**: Active benchmark reached 100/100 with proof. Continue to next benchmark.
- **IMPROVED**: Score increased but not yet 100/100. Continue the loop.
- **BLOCKED**: Missing credentials, unavailable services, human approval needed, or unsafe actions. Record exact blocker.
- **FAIL**: Repeated real fixes produced no measurable improvement. Record stuck items.

## Final response after each run

Report: PASS / IMPROVED / FAIL / BLOCKED, active benchmark, previous score, new score, best score, score delta, main fix made, remaining highest-impact issue, validation verdict, browser-proof verdict, vision-review verdict, score artifact paths, correlation ID, commit hash, next benchmark or next fix.
