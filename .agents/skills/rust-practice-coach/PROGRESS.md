# Rust Practice Progress

Keep this file brief and evidence-based. Update it after meaningful milestones; do not turn it into a journal or mark an exercise complete before its Rustacean review.

## Learner profile

- **Background:** Experienced software engineer learning Rust application idioms
- **Typical timebox:** 30–90 minutes
- **Current interests:** Not recorded yet
- **Preferred comparison languages:** Not recorded yet

## Current exercise

- **Exercise:** A02 — Reduce a mixed event stream
- **Path:** `practice/a02-event-stream/`
- **Started:** 2026-08-22
- **Current step:** Public API and acceptance tests prepared; reducer implementation pending
- **Next action belongs to:** Learner — implement `reduce_events`

## Completed exercises

| Date | Exercise | Concepts practiced | Rustacean review | Revisit |
| --- | --- | --- | --- | --- |

## Skipped exercises

| Date | Exercise | Reason | Reconsider when |
| --- | --- | --- | --- |

## Concept evidence

Use `new`, `practiced`, or `comfortable`; record a concrete exercise or review observation rather than intuition alone.

| Concept | Status | Evidence | Last practiced |
| --- | --- | --- | --- |
| Data-carrying enums | practiced | A01 models mutually exclusive lifecycle states with variant-specific data | 2026-08-22 |
| Consuming transitions | practiced | A01 moves owned payloads between states without cloning | 2026-08-22 |
| Ownership recovery on error | practiced | A01 returns the unchanged `Job` inside `RejectedTransition` | 2026-08-22 |

## Difficulties observed

| Date | Exercise | Concept | What caused difficulty | Hint level reached |
| --- | --- | --- | --- | --- |

## Recurring review comments

Promote a comment here only after it recurs or reveals a meaningful design habit.

| Theme | Count | Latest evidence | Exercise to revisit it |
| --- | ---: | --- | --- |

## Revisit queue

| Priority | Topic | Why revisit | Candidate exercise/context |
| --- | --- | --- | --- |
| Low | Failure-path assertions | A01 failure test checks only the reason, not timestamps and carried state | Strengthen A01 test after A02 |

## More practice requested

- None recorded.

## Coach notes

- Keep `programming-rust/` for book-derived exercises.
- Put this curriculum's independent Cargo packages under `practice/<exercise-id>-<slug>/`.
- A01 Rustacean review is complete; its active-recall reflection remains optional carryover while A02 proceeds.
