# Behavioral acceptance cases (all synthetic)

These are reasoning exercises, not live token findings. No scenario requests a
transaction, paid service, attribution campaign, or protocol-wide exploit audit.
Evaluate whether the skill preserves evidence boundaries rather than matching a
particular reassuring verdict. Deterministic validator tests are in scripts/rust/tests/regression.rs;
the narrative acceptance cases below additionally require analyst judgment.

## 1. Locked canonical liquidity, removable side liquidity

**Request:** “Is liquidity rug resistant?”
**Given:** at pin P, canonical position C is owned by a locker with inspected code
and no current executable principal-removal path. Side position S is owned by an
ordinary controller with decrease/collect authority. Discovery covers two named
factories through P; other factories were not searched.
**Expected:** separately qualify canonical custody and side-pool removal risk.
Identify who controls S and relevant asset amounts; explain unsearched universe.
If the user's rule forbids material principal withdrawal, conditional NO-GO is
appropriate. Do not conclude all liquidity is locked or that C supports the price.

## 2. Fixed supply, poor holder-sized exit

**Request:** “Can a holder with 100,000 units exit?”
**Given:** pinned quotes return 10 quote units for 100 input and 1,000 quote units
for 100,000 input on the same route. Fixed supply is separately proven; no actual
sell receipt at the current pin is available. Inputs/outputs exclude gas consistently.
**Expected:** show per-unit output 0.1 versus 0.01, a 90% degradation from the small
baseline, and identify taxes/fees included or unresolved. Quotes only, not a realized
exit or guaranteed sellability. Fixed supply does not establish economic exit depth.

## 3. Immutable token, upgradeable rewards

**Request:** “Does renounced ownership remove admin risk?”
**Given:** token runtime has no discovered upgrade path in complete inspected scope.
A reward vault paying a promised utility token is proxy-controlled by a 2-of-3
multisig with no observed timelock; implementation replacement can redirect rewards.
**Expected:** separate token controls from reward/admin custody and replacement
capability. Resolve signers/modules and the right to change them as necessary.
No whole-system renunciation or immutable-economics conclusion.

## 4. Launch sellers and rebuys to new recipients

**Request:** “Did this declared launch cohort cash out?”
**Given:** successful sales deliver 50 quote units; successful rebuys spend 40,
deliver tokens to new recipients, and 10 quote units remain before unresolved gas
and costs. The cohort was defined as direct launch-buy recipients over a fixed
window. Shared initial funding is the only alleged identity link.
**Expected:** track receipts, quote costs, retained assets and downstream inventory.
Describe market-mediated redistribution; 50 gross proceeds is not 50 profit or
proof of final cash-out. Shared funding does not establish common ownership or
coordination. If Pons-style curve buys are used, inspect direct recipients and tax.

## 5. Synthetic claims in a vault

**Request:** “Are rewards backed by the underlying asset?”
**Given:** vault holds 1,000 synthetic units. A preview returns 1:1; the only simulated
receipt mints another claim token, and no intended underlying balance increase was
observed. Admin redemption eligibility is unresolved.
**Expected:** distinguish inventory, liabilities and enforceable redemption. State
that underlying backing/exit is unproven. The preview, nominal peg and success event
cannot establish an exit. Deepen only the relevant dependencies and redemption path.

## 6. RPC coverage failure

**Request:** “Was the launch concentrated?”
**Given:** current balances are readable; launch-era archive calls time out and
bounded log queries still fail on public alternatives. A scanner reports green.
**Expected:** current concentration may be measured separately; historical launch
concentration is unknown due to unavailable evidence. Preserve failed parameters.
Neither the timeout nor the scanner establishes token misconduct or a historical pass.

## 7. Focused LP-fee origin

**Request:** “Did this vault's 100 units come from LP fees?”
**Given:** opening inventory 20; observed receipt-level fee collections forwarded 50;
an unrelated donation adds 40; observed payout 10; closing balance 100. Pool principal
decreases have been separately excluded from the 50 in the investigated period.
**Expected:** reconcile 20 + 50 + 40 = 10 + 100. Report 50 gross identified fee inflows,
40 donation, 20 opening stock of unresolved earlier origin, and 10 outflow. Do not
label the entire closing balance fee-origin or assign which commingled source funded
the payout. Do not automatically audit the whole token.

## Example invocations

- Focused: “Use $evm-token-due-diligence for chain ID [ID], token [full 0x address].
  Determine whether vault [full address] holdings of [asset] came from LP fees over
  blocks [start]–[end]. Reconcile sources and explicitly bound unexplained amounts.”
- Broad: “Use $evm-token-due-diligence for chain ID [ID], token [full 0x address].
  Assess rug resistance, a holder-sized exit of [raw or decimal units], and enforceable
  reward rights. Screen all eleven surfaces and deliver pinned evidence and manifest.”
- Formal: “Using this already validated case directory, use $evm-token-due-diligence
  to produce the requested formal report and a custody/rights diagram. Reuse the
  evidence; identify any formatting conversion checks separately.”

Bracketed invocation inputs are deliberate user inputs, never acceptable artifact
pins. If absent at execution, obtain or resolve exact identity before querying.
