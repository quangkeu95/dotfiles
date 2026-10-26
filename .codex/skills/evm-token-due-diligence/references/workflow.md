# Workflow and target packet

## Freeze identity before discovery

Ask for a missing address or chain when the intended target cannot be uniquely
established. Never resolve ambiguity by symbol popularity. Record requested,
queried, and observed identity. Verify `eth_chainId`; a chain name, explorer URL,
RPC hostname, or wallet configuration is not that verification. For a custom chain,
also verify official infrastructure, environment (mainnet/testnet), native asset,
finality semantics, archive capability, and deployed venue contracts when used.

Build one packet with the decision question, operating mode, requested/observed
identity, name/symbol/decimals/raw total supply or explicit unresolved states,
captured block header and UTC pin, runtime bytes and labeled hash algorithm,
proxy/implementation/beacon/admin status, deployment/launch transaction if found,
candidate pools/related contracts, materiality rules, discovery coverage, and known
limitations. Treat `0x` code as evidence of no runtime at that pin, not proof of an
ordinary EOA or absence of future/delegated behavior. Inspect delegation designators
and chain-supported account features when relevant.

Choose a sufficiently confirmed block for the question. Save `eth_getBlockByNumber`
and `eth_chainId` responses. Prefer hash-bound reads using EIP-1898 with
`requireCanonical` when supported. Otherwise use an explicit block number and
re-fetch the header before completion; invalidate mixed results if the hash changed.
For `eth_getLogs`, freeze ranges, record returned pagination/chunk boundaries,
detect omissions, and retain transaction/block/log indexes. Historical receipts
need matching block hashes and their own captured headers. Additional chains each
need their own independent RPC verification and pins; do not imply atomic timing.

Metadata: call the target's selectors for name, symbol, decimals, totalSupply at the
pin. Decode with a documented ABI. Record reverts and nonstandard encodings;
do not assume 18 decimals or invent a name. Resolve proxy storage at the same pin.
Runtime SHA-256 is an artifact fingerprint; Ethereum code hash and PoolId derivation
require Ethereum Keccak-256, **not** `hashlib.sha3_256`.

## Cheap architecture pass

Enumerate contracts and keys able to change balances, restrict transfers, remove
principal, upgrade behavior, collect/route fees, allocate rewards, or enforce utility.
For every edge record caller, callee, role/authorization, adjustable parameter or
asset, evidence, and whether active now or introducible through upgrades. Follow
owner/role changes into multisigs, timelocks, modules, guards, executors, beacons,
lockers, reward layers, and rescue/arbitrary-call mechanisms. No-owner token code
does not answer who controls the economic system.

Screen the core surfaces in broad mode. Define pool, asset, balance, and time-range
materiality before expensive accounting, with units and reason. **Never** apply a
monetary cutoff to discovering mint, upgrade, seizure, transfer restriction,
arbitrary-call, or LP-removal authority. A low-balance authority can later control
all value. Preserve an exclusions list and reassess thresholds if new evidence
could change the conclusion.

## Efficient execution

Batch independent reads within RPC limits; preserve per-call errors (multicall
semantics can differ from direct calls). Cache by verified chain, subject address,
block hash, method, canonicalized arguments, calldata, from/value, and relevant
state overrides. Cache discovery separately with range/page/capture time. Include
fork configuration in simulation cache keys. Deduplicate identical runtimes by
code hash, but never deduplicate storage, authorities, initializer state, immutables,
deployment provenance, or economic claims on code similarity alone.

Deepen only on a conclusion-changing trigger. Keep one ledger and reuse it for
reports. If parallel agents are already authorized and available, assign bounded
lanes from the same immutable packet (for example custody, fees, distribution).
Require evidence rows, findings, and unresolved questions, not separate reports.
Resolve inconsistent pins/interpretations centrally. This is not an instruction to
spawn agents without authorization; the identical lanes work sequentially.

Stop a focused track once its proposition is reconciled or the remaining uncertainty
is explicitly bounded. Do not turn “could this fee balance be LP revenue?” into an
unrequested full audit. Provider outages justify trying another public/authorized
source or a smaller range; after useful alternatives are exhausted, report missing
coverage. Never substitute fabricated state.

## Simulation gate (only when decisive and in scope)

No real wallet/provider injection, private key, seed phrase, signing service, public
RPC write, or broadcast mode. Read-only quotes use `eth_call`, not real trades.
If a local-fork simulation is warranted, require all of:

1. A disposable process/directory you control, bound to literal loopback, with a
   verified local process identity and RPC method/config inspection. A localhost
   URL or `eth_chainId` alone does not prove it is a fork or prevent forwarding.
2. Recorded upstream chain ID, upstream block number/hash, fork creation/config,
   local chain ID (which may differ), and verified fork state. No public exposure,
   upstream write forwarding, production backend, relayed transaction submission,
   or real credentials. Stop if provenance/isolation cannot be established.
3. Synthetic accounts only, synthetic gas funding, fork-only impersonation if
   explicitly needed, and declared balance/storage overrides. Use acquired tokens
   through realistic routes when possible; state overrides can bypass restrictions
   and invalidate an ordinary-holder claim. An impersonated privileged account
   tests authority, not ordinary-holder sellability.
4. Before/after balances of input, output, native gas, and intermediates; exact
   route, calldata, approvals, receipt status, costs, and logs. A success return,
   event, or quote alone does not establish delivery of the intended underlying.
5. Counterfactual label, limitations (MEV/order flow/oracles/bridges/keepers/finality),
   and disposal/no-persistence declaration. Fork transactions cannot prove live
   execution or a future exit. Never export a signed raw transaction to broadcast.

Bundled helpers do not send RPC or perform simulations. Their safety declarations
are assertions to check against actual process/config evidence, not enforcement
of another tool's network behavior.
