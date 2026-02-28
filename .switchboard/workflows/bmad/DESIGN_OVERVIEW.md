# BMAD Method for Switchboard — Design Overview

## What This Is

A fully autonomous implementation of [BMAD Method](https://github.com/bmad-code-org/BMAD-METHOD) concepts for Switchboard's cron-scheduled agent system. Given a single `PRD.md` as input, the system produces architecture decisions, epic/story breakdowns, and working code — all without human intervention.

## The Only Human Input

Drop a `PRD.md` into `.switchboard/input/PRD.md` and run `switchboard up`.

The PRD should contain: project name, problem statement, target users, functional requirements, non-functional requirements, and scope boundaries. The system handles everything from there.

## Pipeline Overview

```
Human drops PRD.md
       │
       ▼
┌─────────────────────────────────┐
│  PHASE 1: SOLUTIONING           │  (runs once)
│                                  │
│  sb-solution-architect           │
│  └─ Reads PRD.md                │
│  └─ Produces:                   │
│     • architecture.md            │
│     • project-context.md         │
│     • epics/epic-*.md            │
│     • sprint-status.yaml         │
│  └─ Signals: .solutioning_done  │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│  PHASE 2: IMPLEMENTATION         │  (repeating sprint loop)
│                                  │
│  sb-sprint-planner               │
│  └─ Selects stories for sprint  │
│  └─ Creates story files          │
│  └─ Distributes to DEV_TODOs    │
│  └─ Signals: .stories_ready     │
│                                  │
│  sb-dev-1..N  (parallel)         │
│  └─ Implements stories          │
│  └─ Queues for review           │
│  └─ Signals: .dev_done_{N}     │
│                                  │
│  sb-code-reviewer                │
│  └─ Reviews implementations     │
│  └─ Approves or rejects         │
│                                  │
│  sb-scrum-master                 │
│  └─ Detects sprint completion   │
│  └─ Writes metrics/reports      │
│  └─ Cleans up for next sprint   │
│  └─ Signals: .sprint_complete   │
│                                  │
│  [loop until all stories done]   │
└─────────────────────────────────┘
```

## Agent Roster

| Agent | Schedule | Role | Phase |
|-------|----------|------|-------|
| `sb-solution-architect` | `*/30 * * * *` | PRD → architecture + epics | Solutioning (once) |
| `sb-sprint-planner` | `*/30 * * * *` | Epics → story files → DEV_TODOs | Implementation |
| `sb-dev-1..N` | `*/30 * * * *` | Story → working code + tests | Implementation |
| `sb-code-reviewer` | `*/45 * * * *` | Quality gate, approve/reject | Implementation |
| `sb-scrum-master` | `0 * * * *` | Sprint lifecycle + metrics | Implementation |

## Directory Layout

```
.switchboard/
├── input/                                 # HUMAN INPUT (the only thing you provide)
│   └── PRD.md
│
├── planning/                              # AGENT-GENERATED planning artifacts
│   ├── architecture.md                    # System design, ADRs, tech stack
│   ├── project-context.md                 # Conventions, patterns, rules
│   └── epics/                             # Epic files with story definitions
│       ├── epic-01-{slug}.md
│       ├── epic-02-{slug}.md
│       └── ...
│
├── state/                                 # RUNTIME state (agent coordination)
│   ├── sprint-status.yaml                 # Sprint tracking
│   ├── stories/                           # Detailed story files for current sprint
│   │   ├── story-1.1-{slug}.md
│   │   └── ...
│   ├── DEV_TODO1.md .. DEV_TODON.md       # Per-agent work queues
│   ├── review/
│   │   └── REVIEW_QUEUE.md
│   ├── BLOCKERS.md
│   ├── SPRINT_REPORT.md
│   │
│   │  # Signal files (coordination)
│   ├── .solutioning_done                  # Phase 1 complete
│   ├── .stories_ready                     # Sprint planned, devs can start
│   ├── .dev_done_1 .. .dev_done_N         # Individual agent completion
│   ├── .sprint_complete                   # All devs done
│   └── .project_complete                  # All stories done
│
├── prompts/
│   └── workflows/
│       └── bmad/
│           ├── SOLUTION_ARCHITECT.md
│           ├── SPRINT_PLANNER.md
│           ├── DEV_PARALLEL.md
│           ├── CODE_REVIEWER.md
│           └── SCRUM_MASTER.md
│
├── logs/
└── skills/

switchboard.toml
```

## Signal Protocol

All coordination happens through signal files. No agent reads another agent's prompt
or internal state. The protocol is:

```
.solutioning_done     ← Solution Architect creates when architecture + epics ready
.stories_ready        ← Sprint Planner creates when DEV_TODOs are written
.dev_done_{N}         ← Dev Agent N creates when all its stories are done
.sprint_complete      ← Last dev agent creates when ALL .dev_done_* exist
.project_complete     ← Scrum Master creates when all stories in all epics are done
```

**Guard rails:**
- Sprint Planner won't start until `.solutioning_done` exists
- Dev agents won't start until `.stories_ready` exists
- Code Reviewer only reviews when REVIEW_QUEUE has PENDING entries
- Scrum Master cleans up `.sprint_complete` and `.stories_ready` between sprints
- Solution Architect stops running once `.solutioning_done` exists

## Scaling

Adjust `AGENT_COUNT` in switchboard.toml and add more `sb-dev-N` entries. The file-based
protocol means agents coordinate through state files, not direct communication. Adding
a dev agent is: add a `[[agent]]` block, bump `AGENT_COUNT`, done.

## Running Both Workflows

The BMAD implementation workflow and the existing codebase-maintenance workflow
(Auditor → Planner → Refactor agents) can run simultaneously. They operate on
different state files and don't conflict. The maintenance workflow improves code
the BMAD workflow already shipped.