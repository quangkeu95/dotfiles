# Complete pool and position history

**Trigger:** a lock claim, principal withdrawal, fee-origin question, migrated pool,
or side-pool omission could change the verdict.

**Minimum evidence:** authenticated factories/managers/hooks/lockers; creation and
initialization events over declared ranges; pool keys; position/NFT/core identifiers;
mint/increase/decrease/collect/burn/transfer and approval histories; successful
receipts, storage/ownership/approval state at pins, and relevant code versions.

Enumerate canonical and side pools using more than a promotional link when feasible.
Declare factories, fee tiers, hooks, address universes, block spans and pagination
searched. For v4 bind full PoolKey and PoolId to the exact singleton. Track all
principal decreases, owed assets, fee growth, fee collections, transfer/approval
changes, beneficiary routes, and locker upgrades. Separate event discovery from
code-proven executability; no observed withdrawal is not no possible withdrawal.
Check rescue, arbitrary-call, delegated approvals, and owner-changing paths.

For every material position produce an authority chain from actor to executable
removal function to beneficiary; distinguish current authority from upgrades and
future expiry. Quote depth independently of custody. A permanently locked position
can be out of range and coexist with removable positions.

**Stop:** material discovered positions and authority paths are reconciled over the
relevant history/current pin; or missing manager/hook/history prevents closure.
**Unfinished means:** qualify canonical findings separately and leave omitted
side-pool/position removal risk unknown. Never claim exhaustive pool discovery
without a declared complete universe and range. Deliver the position ledger,
discovery log, principal/fee split and authority evidence.
