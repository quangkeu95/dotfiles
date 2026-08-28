# Work Knowledge Wiki

I maintain an LLM-managed knowledge wiki (Obsidian vault) centralizing knowledge across my work projects:

- **Location**: `@~/working-spaces/obsidian-vault`
- **Entry points**: `index.md` (catalog of every page), `log.md` (chronological activity), `wiki/` (the pages), `CLAUDE.md` in the vault (the wiki's schema and conventions)

## Read from the wiki first — codebase second, via agents

Before starting work on a project covered by the wiki (check `index.md` — currently: **datapipe**, **token-service**, **meteora-interface-rs**, **DLMM** (program), **dlmm-pro** (program), **damm-v2** (program), **dynamic-bonding-curve** (DBC program)), answer questions from the wiki first:

1. Read `index.md` to find relevant pages, then read those pages (project hub → entities/concepts). Grep across `wiki/` if the index isn't enough.
2. Prefer the wiki for architecture, cross-service data flow, domain concepts (protocols, pricing, PnL), conventions, and decision rationale — it synthesizes knowledge that is scattered or absent in any single repo.
3. **Only when the wiki doesn't have the info** (or a load-bearing fact needs verification against current code), go to the codebase — and do it by **spawning a subagent** (Explore for lookups/searches, general-purpose for multi-step investigation) rather than reading repo files into the main context. Give the agent the wiki page's cited repo paths as starting points; keep only its conclusions.
4. Wiki pages cite their sources (repo paths, `raw/web/` snapshots) and frontmatter `updated:` dates tell you how fresh a page is. Code-derived facts drift — treat stale pages as hypotheses to verify (via an agent), not truth.

Exception: when the task itself is editing code in that repo, work on the files directly as normal — the agent-first rule is for *answering questions/gathering context*, not for doing the edits.

## Keep it compounding

If a session uncovers significant new knowledge (a design decision, a gotcha, an architecture change, a resolved open question), suggest filing it into the wiki. The wiki's own `CLAUDE.md` defines the ingest/update workflow — apply updates from within the vault, following its conventions (update `index.md`, append to `log.md`).
