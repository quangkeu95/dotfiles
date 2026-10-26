# Reward epochs, distributions, and backlog

**Trigger:** rewards are economically material, unexplained vault/distribution
balances exist, payouts allegedly exceed funding, or backlog threatens liveness.

**Minimum evidence:** epoch boundaries, opening/closing inventory and liabilities,
eligibility/weight/claim rules and units, deposits/prefunding/donations/carryover/
minting, payout receipts, cumulative paid counters/caps, admin changes, and actual
processing transactions with gas and timing.

For each asset and epoch separate available inventory from obligations and enforceable
claims. Reconcile conservation across buys, funding, minting, donations, burns, payouts
and retained balance. Distributed > purchased is not an error if explained by other
sources. Verify entitlement calculations, integer rounding, snapshot/rebase units,
duplicate recipients or repeated claim identifiers, cumulative caps, unpaid amounts,
and whether manual discretionary routing can bypass the documented rules.

Model queue growth with observed arrival and processing rates, service frequency,
per-item/transaction gas, block capacity assumptions, skip/failure/retry behavior,
keeper funding/authorization and censorship dependencies. Distinguish a temporary
backlog from an unserviceable one; provide explicit horizons and sensitivity ranges.
A simulation's throughput is counterfactual and may omit live network competition.

**Stop:** material inventory/liabilities/payouts reconcile and observed liveness is
bounded at the pin; or missing eligibility/history/processing evidence prevents it.
**Unfinished means:** reward conservation, entitlement, or sustainable processing
remain unknown separately; do not infer token solvency from reward inventory.
Deliver epoch tables, residuals, unpaid claims, duplicate/cap checks and backlog model.
