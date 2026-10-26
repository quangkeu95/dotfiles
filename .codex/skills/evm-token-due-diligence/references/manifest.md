# Target-integrity manifest and helper contract

## Portable installation and dependencies

The package may live anywhere; resolve references and scripts from its own directory,
and evidence paths from the case manifest's parent. For Codex, copy the entire folder
to `${CODEX_HOME:-$HOME/.codex}/skills/evm-token-due-diligence` if that target does not
already contain a skill. Inspect an existing installation before updating it. Never
hardcode an author's home directory, RPC credentials, or a chain-specific RPC.

Bundled helpers need **Rust/Cargo 1.85+** to build; the resulting native binary needs
no interpreter. Runtime dependencies are serde, serde_json, sha2, num-bigint and
chrono; tests additionally use tempfile. Cargo.lock pins transitive dependencies.
The binary/library does not use RPC, download dependencies, sign, broadcast, or
spawn processes. The shell wrapper invokes Cargo, which can execute dependency
build scripts: build in a trusted environment. Actual investigations need a verified
read-only RPC and appropriate chain data. `curl`/a JSON-RPC client, ABI decoders,
Ethereum Keccak tools, Solidity builds/Foundry, trace/archive services and a local
fork are optional tools for triggered work; verify their availability/version and
document missing coverage. They are not bundled or presumed. No paid provider is
required. Ethereum Keccak is not NIST SHA3-256.

If dependencies are not cached, explicitly fetch them once (this step needs network
access). Subsequent wrapper commands use `--locked --offline` and fail if unavailable:

```sh
cargo fetch --locked --manifest-path scripts/rust/Cargo.toml
cargo build --release --locked --offline --manifest-path scripts/rust/Cargo.toml
```

The release binary is `scripts/rust/target/release/evm-diligence`. Do not distribute
the target directory as source. On systems without a POSIX shell, invoke the binary
or Cargo directly with the same subcommands.

`sh scripts/diligence selftest --emit-example PATH` creates a **synthetic** case in a new/empty
directory, including a working manifest, source report, raw RPC fixtures and rendered
report. The shipped [synthetic manifest](../examples/synthetic-case/manifest.json)
is a schema example, not live evidence or an invitation to reuse its identities.
Never relabel that fixture as live or replace only its token name. Build real cases
from actual queries when separately requested.

## Files and lifecycle

`manifest.json` binds identity, pins, raw artifacts, scope, metadata, discovery and
safety to `report-source.json` with a SHA-256 hash. The source is the reconciled
report data; `render_report` validates it before generating Markdown. The renderer
creates a new output exclusively, so choose a new versioned filename after edits.
Update the source checksum in the manifest after legitimate source edits. Do not
update checksums merely to mask an unexplained evidence discrepancy.

```sh
sh scripts/diligence selftest
sh scripts/diligence validate_report examples/synthetic-case/manifest.json --report examples/synthetic-case/report.md --allow-synthetic
sh scripts/diligence render_report /path/to/live-case/manifest.json --output /path/to/live-case/report-v1.md
sh scripts/diligence validate_report /path/to/live-case/manifest.json --report /path/to/live-case/report-v1.md
```

For validation, exit 0 means internal consistency, 1 rejection. Reconciliation exits
0 within declared bounds, 2 with a larger residual, and 1 on malformed input.
Never use `--allow-synthetic` to approve live evidence. Tests use only temporary
directories and synthetic data. The helpers are additive and perform no network calls.

## Manifest schema (version 1)

The shipped fixture is executable documentation for exact JSON types. The validator
fails closed on missing required fields, invalid JSON/duplicate keys, inconsistent
references or malformed evidence. Its procedural schema is in
`scripts/rust/src/validate.rs`. The source schema remains version 1; new execution
claims use the explicit fields below. Previously rendered Markdown must be rendered
again because the Rust renderer escapes untrusted metadata and text.

| Field | Required content |
| --- | --- |
| `schema_version`, `evidence_kind`, `mode` | `1`; `live` or `synthetic`; `focused`, `broad` or `formal` |
| `target.requested`, `.queried`, `.observed` | Each `{chain_id: positive integer, address: 0x + 40 hex}`; queried also has `code_artifact` referring to the target's raw getCode query |
| `question`, `materiality_rules`, `limitations` | Decision question, thresholds/units/exclusions and no authority-discovery cutoff, list of coverage limitations |
| `current_pin` | Pin ID for the target's current chain state |
| `pins` | ID, chain ID, integer number, non-placeholder 32-byte hash, UTC `YYYY-MM-DDTHH:MM:SSZ`, `current`/`historical` time basis, chain-ID artifact and header artifact IDs; current pins also require an explicit-number header recheck artifact |
| `metadata` | Exact `name`, `symbol`, `decimals`, `total_supply` entries; see below |
| `artifacts` | Unique ID, relative path, lowercase SHA-256, kind, subject chain/address, pin ID, root `target_binding`, non-secret provenance |
| `scope` | Every evidence subject's chain/address, role, material Boolean, pin ID, provenance artifact IDs, runtime status, proxy status and related scope |
| `discovery` | Declared search universe and coverage rows; see below |
| `safety` | True no-real-private-keys, no-real-signing, no-broadcast, external-content-untrusted declarations; Boolean simulation use and fork inventory |
| `report_source` | Relative path and SHA-256 of the JSON report source |

An artifact's `target_binding` is always the user's requested target (even on another
chain). Its `chain_id`/`address` identify the actual queried subject in scope. Every
chain needs its own pin/chain-ID response. No token substitution is permitted when
tracing dependencies. Artifact references cannot escape the case directory, including
through symlinks. Add nonmaterial discovery addresses to scope with honest roles;
material addresses need the same runtime/provenance inventory, never just a label.

Metadata entries:

- Resolved: `{status: "resolved", value: ..., evidence: [raw_call_id]}`. Name/symbol
  must decode as standard ABI strings, decimals as uint8 integer, total_supply as a
  decimal string representing uint256 raw units. The validator re-decodes the target
  call and compares the values. It does not rely on an explorer metadata object.
- Unresolved: `{status: "unresolved", value: null, reason: "...", evidence: [...]}`.
  Include failed/nonstandard calls if obtained; an unattempted field must explain why.
  Preserve nonstandard bytes32 returns as raw data; this schema intentionally does
  not upgrade them into standard string identity. No default decimals.

Runtime: `{status: "code"|"no_code", artifact: raw_getCode_id, sha256: ...}` hashes
the decoded runtime bytes, not the JSON or hex string. `no_code` requires `0x` but
does not establish an EOA. `unresolved` requires `reason` and `evidence` list.
The target's query anchor still requires a captured getCode response artifact; if
even this cannot be collected, do not release a validated broad report. Give a
focused coverage statement instead. Proxy objects require `status` (`resolved`,
`not_detected`, `unresolved`), `basis`, `evidence`, and `related_scope` identities for
implementation, beacon, facet or authority contracts. A non-unresolved proxy status
needs evidence; source equivalence and exhaustiveness still require analyst review.

## Raw RPC wrapper

Each RPC artifact is a JSON object with `request` (JSON-RPC 2.0, ID, method, exact
params) and `response` (matching ID and either result or error). Preserve the actual
raw result; add an outer `endpoint: "[REDACTED]"`, non-secret `endpoint_label` and
`captured_at_utc`. Array/batched RPC can be normalized into one wrapper per original
request/response with preserved IDs; retain original batch as a document if needed.

Supported methods: eth_chainId; getBlockByNumber/Hash; getCode; getStorageAt; call;
getBalance; getLogs; getTransactionByHash; getTransactionReceipt; getProof (all with
the `eth_` prefix). They are read methods, not signing/broadcast methods. State calls
require the explicit pin number or EIP-1898 hash plus requireCanonical. Arbitrary
state overrides do not enter this ordinary-state evidence format; preserve fork
experiment inputs separately and label them counterfactual.

Receipts/transactions must identify the actual transaction hash and its own matching
pin. Historical pin headers can use exact block number/hash. Log queries require
subject addresses and bounded numeric ranges or the pin hash. Full log replay still
requires the track's completeness/order/reorg checks; the validator does not replay.
Use `source`/`document` kinds for ABI/source/compiler/call-trace artifacts not covered
by this minimal RPC wrapper, preserving method/arguments/provenance separately.
Never disguise a signing/broadcast request as source evidence to bypass policy.

## Source report and ledger

Source fields: `target`, `metadata`, `mode`, `question`, `evidence_kind`, `current_pin`
(matching manifest); `verdict`; `decision` (`conditional_go`, `no_go`, `inconclusive`,
`focused_answer`); nonempty lists `conditions`, `reasons`, `contrary_evidence`,
`unknowns`, `evidence_to_change`, `recommendations`; `findings`; `ratings`. Where there
is no supported contrary evidence or recommendation, explain that rather than
inventing one to fill a field. Do not use empty boilerplate as completed diligence.

Findings have `id`, `proposition`, chain/address, `pin_id`, optional `transaction`,
`status`, `severity`, `evidence_type`, `confidence`, `alternatives`, `coverage`,
`time_basis`, `stale_when` and evidence rows `{artifact_id, decoding_basis}`. Unknown,
unassessed and inapplicable findings also require `reason`. Status is `finding`,
`bounded_negative`, `unknown`, `not_assessed`, or `not_applicable`; there is no generic
“pass” state. A bounded negative requires complete coverage **of its declared scope**,
usable evidence and known confidence. A failed RPC alone cannot support a token-risk
finding. Do not hide a coverage gap by narrowing the headline after research.

Ratings use the eleven `SURFACES` identifiers in `scripts/rust/src/validate.rs`, each with status,
severity (`critical`, `high`, `medium`, `low`, `none`, `unknown`, `not_applicable`),
likelihood (`observed`, `likely`, `plausible`, `unlikely`, `unknown`, `not_applicable`),
confidence (`high`, `medium`, `low`, `unknown`), coverage (`complete`, `partial`,
`unavailable`, `not_assessed`, `not_applicable`), time basis and finding IDs. Broad and
formal modes require all eleven. Unknown/unassessed is severity unknown. No-risk
ratings cannot reference unknown or risky findings, and a surface cannot lower a
linked finding's severity by averaging. Separate an observed power from the likelihood
that it will be misused; never fabricate likelihood percentages.

Every finding must be linked to a rating, including critical findings and unknowns.
If a finding supplies `surface`, it must match a linked rating. Null RPC results are
not usable evidence. A receipt supporting a bounded negative must be successful.
RPC wrappers explicitly marked synthetic cannot be included in a live manifest.
These checks prevent known internal contradictions, not deliberate evidence forgery.

### Structured execution claims

Use `claim_kind: "executed_exit"` for decisive executed-sale/redemption claims;
`observation` is the default for other findings. Execution claims require an
`execution` object with `basis` (`historical` or `fork`), `receipt_artifact`,
`underlying_asset` (address or `native`), `recipient`, `balance_before_artifact`,
`balance_after_artifact`, positive decimal-string `minimum_received_raw`, and
nonempty `route`, `costs`, `attribution_basis`. Include all three artifacts in the
finding's evidence list. The validator requires a successful receipt, matching
chains, matching asset/recipient balance queries, a before pin strictly preceding
the receipt block, an after pin at or after it, and sufficient positive balance
delta. Fork claims additionally require `counterfactual: true` and the fork gate.

Block-level deltas can include unrelated transfers and do not prove transaction-level
causation. Reconcile intervening activity or capture isolated fork evidence; disclose
fees and attribution limitations. The validator cannot infer claim_kind reliably
from arbitrary prose, so human review must reject execution claims disguised as
observations and synthetic claims presented as underlying assets.

Discovery rows carry chain/address/pin, `claim`, `search_universe`, integer
`from_block`/`to_block`, `pagination`, `inclusion_rules`, `exclusions`, `materiality`,
coverage and evidence IDs. Even unattempted broad discovery needs an explicit row
explaining the omission. Missing discovery evidence does not justify completeness.

## Fork evidence

If simulation_used is true, forks must be nonempty. Each entry declares disposable,
synthetic_accounts_only, loopback_only, no_upstream_writes, counterfactual, upstream_pin,
positive local_chain_id, process_identity, overrides, disposal, verification_artifacts.
At least one verification artifact has kind `fork` and JSON mirroring these fields,
plus literal-loopback HTTP local_endpoint, upstream_chain_id and upstream_block_hash.
The validator cross-checks these; localhost names, public endpoints, hidden credentials
and upstream pin conflicts are rejected. This does **not** itself verify a running
process, socket isolation or a fork's actual behavior: perform workflow.md's gate.

## Exact-amount reconciliation

Input: `{assets: [...]}` with entries keyed by chain_id, asset address or `native`,
unit, opening, closing, unexplained_bound, and entries of
`{id, kind: inflow|outflow|adjustment, amount, evidence: [...], explanation}`.
Amounts are decimal strings; adjustments alone may be signed. IDs must be unique
within each asset. Opening includes prefunding/carryover; never force distributions
to be less than current purchases. The output retains signed unexplained delta and
checks its absolute value against the declared bound. A large chosen bound can hide
material uncertainty: disclose the amount and justify it, never describe it as zero.

## Validation limits

Input files and rendered output are limited to 32 MiB each. Chain IDs, block numbers
and ordinary RPC quantities use u64; token units and reconciliation use arbitrary
precision integers/decimals. Decimal strings are limited to 2,048 characters and
exponent notation is rejected. File size limits are not an aggregate resource quota.
Do not run the validator on concurrently mutable case directories. Artifact paths
are confined via canonicalization, not an OS-level sandbox.

Passing establishes internal consistency of the structured packet, file digests,
query identities, metadata decoding, captured pins, declarations, and exact generated
Markdown. It does **not** establish RPC honesty, cryptographic header authenticity,
discovery completeness, ABI correctness, source equivalence, real-world identity,
onchain economic conservation, claim semantics in arbitrary prose, live fork
isolation, or protocol safety. A coherent fabricated header/response cannot be
distinguished from an honest RPC by this offline checker. It rejects placeholders
and header/pin conflicts; authenticate material state independently when required.
Human review must check the ledger propositions, units and coverage against the raw
evidence. Unknown findings can legitimately pass packet validation while the token
assessment remains inconclusive. Do not confuse those two meanings of validation.

## Backend and frontend integration

The `evm-diligence` crate exports validation, rendering and exact reconciliation
functions. A backend can depend on the crate by path and call it in-process, without
shelling out. It validates already-collected evidence; it is not a live collector or
an autonomous token scanner. An API still needs scoped collection, evidence storage,
job limits, authentication where appropriate, and a structured response contract.

Prefer React → backend API → evidence collector → Rust validation. Accept exact
chain/address pairs or server-owned case IDs, not arbitrary client filesystem paths,
RPC URLs, shell commands or manifest paths. Restrict RPC upstreams, protect secrets,
bound aggregate work, isolate immutable case snapshots, and return only appropriate
evidence/provenance fields. Build the binary during deployment, not per request.

React can display structured findings and perform input checks, but cannot directly
execute a native Rust library or this filesystem-based CLI. A browser/WASM variant
would require an in-memory I/O adapter and separate build/tests; it is not included.
Client-side validation is not a trusted backend attestation. Render plain text/escaped
Markdown without raw HTML; do not undo the renderer's escaping. Never put privileged
provider credentials or real signing operations in the browser diligence flow.
