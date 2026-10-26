# Broad screening surfaces

For each surface record what was checked, exact contracts and pins, evidence,
current executable powers, powers introducible by upgrades, exclusions, and unknowns.
Deepen only when a material claim depends on the result. Do not infer a pass from
the absence of scanner flags or a verified-source badge.

## A. Token controls

Inspect mint/burn/rebase, balance rewrites/seizure, pause/blacklist/whitelist,
taxes/exemptions, cooldowns, transaction/holding limits, trading gates, external
calls/delegatecall, and every upgrade path. Resolve roles/owners and ability to grant
roles or change the grant administrator. Resolve multisig owners/threshold,
modules/guards, timelock delay/executors/cancellers/bypass, and who changes these.
Renunciation is a local claim; implementations, external registries, other roles,
hooks, lockers, or reward layers may retain control. Use the bytecode track if needed.

## B. Liquidity-principal custody

Discover exact pools or complete pool keys; classify canonical, side, inactive, and
unverified pools. Record discovery sources/ranges, liquidity materiality and omissions.
For every material position identify manager, position/NFT ID, tick range, liquidity,
owner, approval, operators, locker/hook control, and all indirect removal paths:
decrease, burn, withdraw, rescue, arbitrary call, approval, transfer, and upgrade.
Separate fee collection from principal removal; position “tokens owed” can include
previously decreased principal. Locked canonical liquidity is not all liquidity,
an in-range guarantee, price support, or a whole-system rug-resistance finding.

## C. Sellability and exit depth

Find a historical successful ordinary-holder sale if available; verify receipt,
pool/curve mechanics, input sender and ultimate output recipient. Then obtain
current pinned read-only quotes for a small amount and relevant holder-sized sizes.
Choose sizes using balances and the user's question, not only nominal USD values.
Record exact route/contracts/pools, from/value, input raw units/decimals, quote asset,
expected net output, token/pool fees, gas assumptions, failures, and per-unit output.
Report degradation as `1 - (output_large/input_large)/(output_small/input_small)`
using consistent units and disclose the baseline. Do not mix gas-adjusted and
unadjusted observations. Distinguish pool impact, slippage tolerance, gas, spot,
and executable depth; v3/v4 depth depends on traversed ticks and hooks.
Historical execution only proves that state. Quotes/previews may miss transfer
rules, sender exemptions, callbacks, or downstream delivery. Decisive fork exits
must satisfy workflow.md's receipt and underlying-balance-delta gate.

## D. Supply and ownership

Reconcile raw supply and balances using the token's accounting model. Distinguish
asset units, rebase shares, synthetic claims, bond/NFT claims, LP units, custody,
total supply, and circulating float. Classify pools, protocol custody, lockers,
treasuries, burn addresses, creator allocations, and investor-like holdings separately.
State denominator, exclusions, and whether concentration measures accounts,
beneficial owners, or a declared cohort. Contract custody is not automatically an
independent investor. Use full replay only when discrepancies/history justify it;
rebases or balance rewrites can occur without ordinary Transfer logs.

## E. Launch integrity

Verify deployment sequence, transaction calldata, factory version/runtime, salts/
deterministic addresses, launch parameters, allocations, fees/exemptions, direct buy
recipients, funding, early transfers/sales. Distinguish defaults automatically applied
by the exact platform version from caller-supplied exceptions. Freeze a cohort rule
before measurement. Track allocation, transfers, sales, rebuys, downstream inventory,
proceeds, fees, retained assets. Zero token balance is not cash-out proof; a router
transfer is not sale proof. For Pons-style curves inspect the direct buy recipient
and its tax treatment, not just the transaction signer. See launch-cohort.md.

## F. Fees, treasury, proceeds

Map basis, denomination, splits, escrow, claim authority, recipient changes, routes,
and observed use. A percentage of a fee bucket is not that percentage of gross
trade value. Separate current configured rates from realized historical rates.
For each asset reconcile opening + inflows + explained adjustments = outflows +
closing + bounded unexplained delta. Handle wraps, burns, bridge legs, gas, and
reverts without double counting. Trace bridges only with linked source execution,
message/nonce identifiers, destination chain/recipient, delivered amount and evidence.
Stop exact attribution at commingling; exchange deposits do not prove sales, fiat
withdrawals, or final beneficiaries. Do not call proceeds profit without cost basis,
fees, material flows, and retained inventory.

## G. Rewards, vaults, backing, redemption

Separate inventory from liabilities and marketing promises from enforceable rights.
Identify entitlement, claimant, actual received asset, conversion units/rounding,
fees/timing/caps, approvals, admin/keeper dependence, and underlying exit. Vault assets
may be encumbered or synthetic; face value is not liquid backing. For fee-origin
questions reconcile holdings and forwarding to receipt-level collections, excluding
decreased principal, donations, prefunding, and unrelated inflows. Separate current
vault assets, pool inventory, and unclaimed fees. Distribution checks cover
conservation, eligibility, cumulative caps, duplicates, unpaid amounts, retained
inventory, processing order and liveness. Distributed amounts can exceed purchases
through prefunding, donations, carryover, or minting; reconcile these before alleging
an inconsistency. Load reward-accounting.md or dependencies-redemption.md as needed.

## H. Utility, dependencies, development

Test whether advertised functionality is live, specifically linked to holding this
token, and enforceable without discretionary intervention. Trace material assets,
oracles, bridges, APIs, keepers, lending/collateral systems, custody and redemption.
Assess source correspondence, reproducible builds/compiler settings, meaningful
tests, audit scope/version/date/open issues, release/upgrade controls, governance,
disclosures against actual operations. Copied templates, polished marketing, or
awkward code do not establish safety, fraud, or AI authorship. A protocol-wide
exploit campaign belongs in an explicitly scoped audit, not this screen.
