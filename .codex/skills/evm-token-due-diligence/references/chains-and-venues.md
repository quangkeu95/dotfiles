# Conditional chain and venue guidance

Official references below are discovery starting points, checked during package
authoring on 2026-09-06, not immutable registries. Re-check the actual network,
version, deployments, ABI, and runtime when used. This package deliberately does
not embed a chain allowlist, RPC credentials, deployment addresses, or current fees.

## Any EVM chain and Robinhood Chain

Verify `eth_chainId`, mainnet versus testnet, chain-native asset, finality, log/archive
limits, source verification availability, and actual deployed DEX/bridge contracts.
For Robinhood Chain use [official connection documentation](https://docs.robinhood.com/chain/connecting/)
and [official chain support](https://robinhood.com/us/en/support/articles/robinhood-chain-testnet/)
to discover endpoints; the article URL itself does not identify the intended network.
Do not infer network environment from an endpoint name. Verify each endpoint by RPC,
including settlement/destination chains if relevant. Public sources may be enough;
paid/indexed services are optional and their absence is coverage, not a risk flag.

Examine rollup/sequencer/bridge dependencies only to the depth material to custody,
finality or redemption. Onchain representations of financial assets do not establish
legal ownership, backing, issuer eligibility, transferability, or redemption rights.
Look for issuer terms plus enforceable operational exits; separate terms from code.

## Uniswap v3

Authenticate factory/pool/position-manager provenance and deployed version. Read
token ordering, fee tier, slot0, liquidity, relevant ticks, and position state at the
pin. Link manager NFT ID to token pair, fee, ticks, liquidity, and fee-growth fields.
Resolve `ownerOf`, token approval, operator approvals, locker beneficiaries, and
upgrades. Direct core positions may exist outside the familiar NFT manager: include
them in custody discovery. Trace actual ability to decrease/collect/transfer/burn.
Collection receipts alone do not prove all collected assets were fees: reconcile
prior decreases and owed principal, fee growth, and balance changes.

Reference: [v3 position manager interface](https://docs.uniswap.org/contracts/v3/reference/periphery/interfaces/INonfungiblePositionManager).
Do not reuse a deployment address from a different chain based on its familiar ABI.

## Uniswap v4

Record PoolManager address and the complete `PoolKey`: ordered currency0/currency1,
fee, signed tickSpacing, hooks address. Derive PoolId with the exact deployed
library/ABI (`keccak256(abi.encode(poolKey))` for the canonical implementation),
verify against initialization logs, and preserve encoded bytes and decoding source.
PoolId alone is insufficient without chain and PoolManager. Native currency, dynamic
fees, permission bits and hooks require version-specific handling; don't improvise
a hash using concatenated text or NIST SHA3.

The singleton's token balances are **not one pool's reserves**. Identify core
positions (owner, tick bounds, salt) and any periphery NFT representation; map them
through the actual manager implementation. Inspect approvals/operators, manager
command decoding, hooks' callbacks/fees/deltas and upgradeability, lock/unlock
settlement and principal-removal paths. Separate active tick liquidity from all
positions and an asset balance from executable output. Include claims/settlement
accounting when used. Callback restrictions can change quote/exit behavior.

References: [PoolId implementation](https://github.com/Uniswap/v4-core/blob/main/src/types/PoolId.sol),
[reading state](https://developers.uniswap.org/docs/protocols/v4/guides/read-pool-state),
[position manager](https://developers.uniswap.org/docs/protocols/v4/guides/position-manager).
Pin a source commit when attributing behavior to a deployed version.

## Pons and other launch platforms

First determine the exact factory/version from launch execution and deployed
runtime. [Pons v1 documentation](https://docs.ponsfamily.com/) and
[Pons v2 documentation](https://docs.ponsfamily.com/v2) describe different launch
architectures; do not apply one version's curve, pool, graduation, limits, or fees
to another. Follow the version proven by the target's deployment, not the current UI.

If a curve exists, decode launch-and-buy calldata and internal calls, quote input,
minimum output, **direct buy recipient**, actual transferred tokens, and applicable
tax/exemption behavior. Record separate launch signer, funder, recipient, creator,
fee recipient, supplied exemptions, and platform-applied defaults. Verify recipient
tax treatment against runtime/receipts; sender identity is insufficient. Trace any
graduation/migration into its destination pool and custody. If there is no curve or
migration, record that bounded finding and inspect the direct-pool launch mechanics.

Distinguish creator fees, protocol shares, pool fees, and quote denomination.
Compare historical realized splits with launch-snapshotted/current configuration.
Automatic liquidity custody or a deterministic deployment does not establish the
creator's identity, control of other wallets, or the safety of side pools/rewards.
