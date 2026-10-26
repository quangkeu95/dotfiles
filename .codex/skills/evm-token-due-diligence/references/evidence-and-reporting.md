# Evidence and output contract

## Ledger and evidence preservation

Use one case directory and one target-bound ledger. Preserve raw requests and
responses, runtime, calldata, receipts, headers, source snapshots/commit identifiers,
decoding ABIs, computation inputs and outputs. Artifact hashes attest to local file
integrity, not truth. Redact credentials before storage (headers, query strings, URL
paths, userinfo, command lines); record the redaction. For bundled validation,
RPC wrappers store endpoint as `[REDACTED]` plus a non-secret provider label. Keep
the endpoint secret outside the case, supplied at execution by the authorized user.
Do not read unrelated secrets, seed phrases, wallet files or private infrastructure.

Every finding row contains: ID; exact proposition; chain; subject address; pin or
transaction; artifact/query references; decoding basis (ABI signature + provenance,
runtime branch, raw offset, source correspondence, or error parsing); evidence type;
severity; confidence; alternatives; coverage; time basis; and stale conditions.
For executed trades/redemptions record receipt status and intended asset deltas with
route/costs. Log emitters/topics/indexed fields, token ordering, signed swap amounts,
and receipt block hashes must be decoded against the actual implementation/version.
Never infer economic success from an event name or a returned Boolean alone.

Evidence classes:

- **Fact:** directly observed at an exact pin/transaction, with decoding support.
- **Strongly supported:** multiple matched observations close material alternatives.
- **Inference:** a model or incomplete linkage; list assumptions and alternatives.
- **Unknown:** failed, missing, ambiguous, skipped, or insufficient evidence.

A bounded negative requires adequate coverage of its declared scope. “No removal
seen in the sample” is not “no executable removal path.” `not_applicable` requires
evidence or a justified scope explanation, never a substitute for an untested claim.
Nonstandard metadata may stay unresolved without blocking analysis of independently
identified runtime, but must not be replaced by an explorer's same-symbol metadata.

For discovery claims record search universe (chains, factories, pool managers,
fee tiers/hooks, APIs/indexers), addresses, block ranges/pages, inclusion rules,
exclusions, empty/failed ranges, cap/dedup handling, and materiality with units.
Absolute coverage assertions require a defensible universe; multiple dashboards
can share the same incomplete index. Source text can be cited as a project claim
without treating it as deployed behavior.

## Layered output

Lead with the answer to the actual decision question, conditioned on requirements,
evidence time and scope. Suitable forms include:

- “No current executable removal path found at the pinned block, within the inspected
  canonical-position authority chain; side-pool coverage is incomplete.”
- “Sellable at the tested sizes under the quoted state; this does not prove realized
  proceeds or an exit for larger holders.”
- “Unknown because historical state was unavailable.”
- “NO-GO under the stated requirement for rug resistance.”

Never give an unconditional safe verdict or a price/return implication. Separately
rate token controls; canonical LP-principal custody; side-pool removal; sellability
and exit depth; current concentration; historical launch integrity; admin/treasury/
reward custody; reward accounting/liveness; utility/redemption; external dependencies;
and development/disclosure. These are eleven dimensions, not a numerical average.
For each include severity, likelihood, confidence, coverage and time basis.
Likelihood is qualitative and evidence-bounded, not an invented probability. A
critical custody finding remains critical even if other surfaces look favorable.

Explain main reasons, strongest contrary evidence, open questions, and concrete
evidence that could change the verdict. Recommendations address observed deficiencies
(for example an upgrade timelock or independently provable underlying exit), not a
generic checklist or unsolicited trading strategy. Describe proceeds rather than
profit unless the cost basis, fees, flows and retained inventory are defensible.

For focused answers use the same ledger for the requested proposition and relevant
dependencies, with a concise direct answer; do not require all eleven investigations.
For broad answers state unassessed dimensions honestly. Formal reports reuse this
same frozen source and produce the requested file/visual format after reconciliation.
Use a minimal authority graph, position table, size/output depth table, or asset
flow/epoch chart only where it exposes a material relationship. Label exclusions,
units and timestamps; do not invent chart series to fill space.

## Release check

1. Reconcile identity, pins, source/runtime correspondence, units, balances and
   material flows. Resolve contradictions before polishing language.
2. Ensure every headline matches its evidence, alternatives and weakest decisive
   dependency. Review attribution and the strongest evidence against the conclusion.
3. Build manifest and report-source JSON according to manifest.md. Run the validator,
   render, then validate the exact generated Markdown with `--report`.
4. For PDF/Word or another format, use a suitable artifact skill if available.
   Record its source report hash and verify target, pins, figures and limitations
   survived conversion. The bundled validator checks only its Markdown renderer,
   not arbitrary PDF/DOCX files. Do not claim those formats were validated by it.
5. Deliver artifact paths, limitations, checks actually completed, and stale triggers.
   No declaration or test result upgrades a token verdict to safe.
