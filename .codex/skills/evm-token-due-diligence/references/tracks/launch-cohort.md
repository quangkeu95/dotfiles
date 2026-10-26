# Launch-cohort accounting

**Trigger:** launch fairness, concentration, insider-like allocations, exemptions,
or sale/rebuy routing is material to the user's question.

**Minimum evidence:** launch transaction/receipt/calldata, deployment sequence,
factory version/runtime, allocation events, quote/token ordering and curve/pool
mechanics, direct recipients, applicable exemptions, relevant early funding and
transfers, successful swap executions and end-state balances.

Before calculating, freeze cohort inclusion (for example direct launch recipients,
not “suspicious wallets”), time bounds, sources, exclusions and coverage. Keep
transaction signer, direct curve buyer/recipient, funder and beneficiary distinct.
For Pons-style launches inspect direct recipient tax treatment and manually supplied
exemptions versus automatic platform defaults in the deployed factory version.

Use an asset ledger for allocation, purchases, transfers, sales, rebuys, downstream
inventory, proceeds, fees and retained assets. Prove direction/output using pool or
curve execution and successful receipts, not router transfers alone. Match the
actual destination recipient; do not stop at a router or settlement account.
A balance reaching zero could be custody transfer, distribution, burn, or bridge.
Track transformed assets without double-counting. Do not claim profit unless cost
basis, material fees/flows, and retained inventory are accounted for.

**Stop:** the predeclared cohort and horizon reconcile within bounded residuals;
expand only if new evidence changes the decision and record the revised rule.
**Unfinished means:** historical concentration/routing/beneficiary amounts remain
bounded or unknown. Describe observed selling/rebuying for new recipients as
“market-mediated redistribution.” Shared funding, timing, routers or deterministic
deployment alone do not prove coordination, identity, fraud, or intent.
