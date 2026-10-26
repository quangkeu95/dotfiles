# Proxy authority and critical bytecode reconstruction

**Trigger:** unverified or mismatched source, unresolved proxy/control paths, external
delegatecall, or critical runtime ambiguity affects balances, custody, or exits.

**Minimum evidence:** runtime at the pin, storage and code of implementations/beacons/
facets/delegation targets, deployment/upgrade calldata and events, source provenance,
compiler/version/settings/link and immutable references, and current roles/authority.

Escalate in stages:
1. Fingerprint runtime with labeled hash, scan selectors/opcodes and proxy/delegation
   patterns. Selectors alone neither prove functions exist nor exhaust behavior.
2. Locate authenticated verified predecessors/build artifacts and compiler metadata;
   reconstruct version lineage and initialization. Similar code is a lead only.
3. Inspect relevant storage and historical successful calls, admin slot/beacon
   implementation, UUPS authorization, diamond facets, minimal clones, and custom
   routers. Empty ERC-1967 slots do not disprove a custom proxy. Follow controllers
   into multisig modules, role grant paths, timelock bypass and upgrade-of-authority.
4. Reconstruct critical branches or run a gated local-fork experiment only where
   needed to resolve a material proposition. Preserve assumptions and limits.

Verify source correspondence by compiling with exact dependencies/settings, linking,
immutables and deployed runtime comparison. Explain every differing byte region;
do not casually strip metadata and declare equivalence. Decompiler output is a
hypothesis, not verified source. Claim executable equivalence only when compilation
and byte comparison close meaningful differences. Without a compiler match, narrow
claims to proven runtime/storage/execution observations.

**Stop:** critical authority propositions are resolved at the pin or the remaining
ambiguity requires an explicitly scoped exploit/protocol audit.
**Unfinished means:** unknown control/removal path coverage; “renounced” or lack of
a known selector cannot rescue that conclusion. Distinguish current executable
permissions from behavior an administrator could introduce via replacement.
References: [ERC-1967](https://eips.ethereum.org/EIPS/eip-1967),
[ERC-1822](https://eips.ethereum.org/EIPS/eip-1822),
[ERC-2535](https://eips.ethereum.org/EIPS/eip-2535).
