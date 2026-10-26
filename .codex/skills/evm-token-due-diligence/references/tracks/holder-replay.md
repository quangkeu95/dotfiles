# Full holder and Transfer replay

**Trigger:** material supply/balance discrepancies, launch/current concentration
comparisons, unexplained burns/rebases, or a historical cohort requiring full coverage.

**Minimum evidence:** deployment/start bound; complete paginated/chunked Transfer
logs through the chosen pin; successful receipts and block hashes; runtime/accounting
model across upgrades; totalSupply and material balanceOf reads at reconciliation
pins. Capture issuance, burns, rebases, shares, balance rewrites, and external balance
dependencies even where no ordinary Transfer exists.

Replay ordered by block/transaction/log index, deduplicate exact log identity, exclude
removed/reorged logs, distinguish mint/burn conventions from transfers to a labeled
burn address, and record every empty or failed range. Never silently treat a provider
log cap as the end. Reconcile replay balances against pinned reads; track residual
by account and asset unit. For rebasing tokens replay the appropriate shares and
conversion factors; if the model cannot be reconstructed, use pinned snapshots and
bound historical uncertainty instead of forcing an ERC20 ledger.

Define account classes and denominator before ranking: custody, pools, lockers,
treasury, burns, creator allocations, investor-like balances. Label EOA/controller
claims conservatively. State exclusions and avoid subtracting the same custody
twice from circulating float.

**Stop:** covered ranges close and material balances/supply reconcile to declared
tolerances, or documented missing history/model ambiguity prevents closure.
**Unfinished means:** missing historical balances, affected denominator and
concentration remain unknown; current reads do not repair a missing launch history.
Deliver replay parameters, ranges, residuals, classifications and evidence rows.
