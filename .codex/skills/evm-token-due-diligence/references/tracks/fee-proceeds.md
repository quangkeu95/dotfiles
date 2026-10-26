# Fee wallets, LP-fee origin, and cross-chain proceeds

**Trigger:** “did this vault balance come from LP fees?”, treasury leakage, fee
economics, recipient routing, or bridged proceeds could change the answer.

**Minimum evidence:** opening/closing asset balances at explicit pins, successful
receipts/logs for collections/forwarding/transfers, fee basis and denominations,
historical configurations and change points, principal decreases, wraps/burns,
native-gas costs, and matching bridge legs when relevant.

For LP-fee origin, start at the queried vault and period. Enumerate all material
inflows, distinguish pool inventory, collectible/unclaimed fees and vault inventory.
Tie each proposed fee inflow to a receipt-level fee collection and forwarding path.
Collections may include owed principal: reconcile position decreases and accounting,
not only Collect event names. Reconcile each asset:
opening + inflows + signed explained adjustments
= outflows + closing + signed unexplained residual.
State a residual bound and basis. Use integer base units (or exact decimals with
documented conversion); the bundled `reconcile` command checks this equation, not provenance.

Map fee rate of gross trade value separately from share of fee bucket. Historical
realized rate = observed eligible fee / observed eligible basis over the declared
period, with exclusions. Current configuration is not historical evidence.
Wrap/unwrap and bridges are transformations, not two independent income events.
Reverted transfers contribute no asset flow, but a reverted transaction can consume
native gas. Avoid double-counting forwarding between scoped accounts.

For bridges preserve source successful execution, message/nonce/transfer identifiers,
destination chain pin, recipient, fees, delivered asset/amount, and destination
receipt. Stop at missing delivery, commingling or exchange deposit. Report attributable
legs or bounds; do not assign exact provenance through commingled funds, infer final
beneficiaries, fiat withdrawals, or sales at an exchange.

**Stop:** fee-origin/proceeds reconciliation closes materially and alternatives are
tested, or a documented provenance boundary is reached. Do not automatically expand
a focused balance-origin question into a full token audit.
**Unfinished means:** distinguish proven fee-origin amounts, other explained sources,
and unattributed holdings. A positive balance alone proves no origin or entitlement.
