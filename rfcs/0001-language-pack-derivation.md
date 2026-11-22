# RFC-0001: Language Pack Derivation

Status: Approved
Author: Lingua Omnibus Foundation
Created: 2026-11-20
Updated: 2026-11-20

## Abstract
Defines how a new natural language is added to Lingua Omnibus via a language pack: a JSON+test directory, zero compiler changes.

## Motivation
Principle 13 (language packs are pure data) requires that adding a language never requires modifying loc or any other tool.

## Proposed Design
1. Create langpacks/{iso}/ directory.
2. Add keywords.json conforming to spec/appendix/langpack-schema.json with all 40 KW_X slots (CONV-063).
3. Add types.json mapping TY_X slots.
4. Add grammar-notes.md (non-normative).
5. Add tests/ with at least one test per keyword.
6. Register version (ISO 8601 date) in keywords.json (CONV-032).

## Backwards Compatibility
Fully backwards compatible — no changes to existing packs or compiler.

## Reference Implementation
langpacks/en/, langpacks/ro/ shipped at spec-0.1.

## Open Questions
None.

## Change Log
| Version | Date | Summary |
|---------|------|---------|
| 0.1 | 2026-11-20 | Initial draft. |