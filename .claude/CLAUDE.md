# Work Knowledge Wikis

I maintain two LLM-managed knowledge wikis (Obsidian vaults, same Karpathy-style "LLM wiki" schema):

| Wiki | Location | Scope |
| --- | --- | --- |
| **Meteora wiki** — shared with the team, **canonical for Meteora/Solana** | `~/workspaces/meteora-llm-wiki` (GitHub `MeteoraAg/meteora-llm-wiki`) | Meteora programs, backend services, infra, protocol/domain concepts |
| **Personal vault** | `~/workspaces/obsidian-vault` | Everything else (EVM/Solidity, personal), plus pre-split copies of Meteora pages |

Both use the same entry points: `index.md` (catalog of every page), `log.md` (chronological activity), `wiki/` (the pages), and a `CLAUDE.md` holding that wiki's schema and conventions. The two `CLAUDE.md` files have started to diverge (e.g. the Meteora wiki cites repos by GitHub remote, not local path) — follow the one belonging to the wiki you are reading or editing.

## Which wiki, in what order

For a **Meteora/Solana project** (check the Meteora wiki's `index.md` — currently: **datapipe**, **token-service**, **blacklist-rs** (met-blacklist), **ds-adapter-api** (DEXScreener adapter), **meteora-interface-rs**, **meteora-infra** (GitOps/deployment monorepo), **meteora-cli**, **DLMM** (program), **dlmm-pro** (program), **damm-v2** (program), **dynamic-bonding-curve** (DBC program)):

1. **Meteora wiki first** — `~/workspaces/meteora-llm-wiki`. Start from its `index.md`.
2. **Personal vault second** — only for what the Meteora wiki lacks. The vault still holds Meteora pages from before the split and a few later ingests not yet ported; where a page exists in both, the Meteora wiki's copy is authoritative and the vault's is a stale hypothesis.
3. **Codebase last**, via agents (next section).

For anything that is not Meteora/Solana, use the personal vault only.

## Read from the wiki first — codebase second, via agents

1. Read `index.md` to find relevant pages, then read those pages (project hub → entities/concepts). Grep across `wiki/` if the index isn't enough.
2. Prefer the wiki for architecture, cross-service data flow, domain concepts (protocols, pricing, PnL), conventions, and decision rationale — it synthesizes knowledge that is scattered or absent in any single repo.
3. **Only when neither wiki has the info** (or a load-bearing fact needs verification against current code), go to the codebase — and do it by **spawning a subagent** (Explore for lookups/searches, general-purpose for multi-step investigation) rather than reading repo files into the main context. Give the agent the wiki page's cited repo paths as starting points; keep only its conclusions.
4. Wiki pages cite their sources (repo paths or remotes, `raw/web/` snapshots) and frontmatter `updated:` / `read_at_commit:` tell you how fresh a page is. Code-derived facts drift — treat stale pages as hypotheses to verify (via an agent), not truth.

Exception: when the task itself is editing code in that repo, work on the files directly as normal — the agent-first rule is for *answering questions/gathering context*, not for doing the edits.

## Keep it compounding

If a session uncovers significant new knowledge (a design decision, a gotcha, an architecture change, a resolved open question), suggest filing it into the wiki:

- **Meteora/Solana knowledge → the Meteora wiki**, never only the vault — that is what keeps the two from drifting apart. The shared repo is git-managed by PR: make the edits in the repo, and commit / push / `gh pr create` only when asked.
- **Everything else → the personal vault.**

Each wiki's own `CLAUDE.md` defines the ingest/update workflow — apply updates from within that repo, following its conventions (update `index.md`, append to `log.md`).
