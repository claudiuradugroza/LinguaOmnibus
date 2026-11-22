SPEC-13: Governance

Version: 0.1 (Draft)
Status: Draft — open for RFC review
Dependencies: spec/00-conventions.md (spec-0.1)
Applies to: the Lingua Omnibus Foundation, all contributors, all RFC authors

1. Motivation

Governance ensures the project remains community-controlled, not BDFL-driven (Principle 9). This document defines:

    the RFC process for proposing changes,

    maintainer election,

    trademark policy,

    explicit fork rights.

2. Normative specification

GOV-001. New rule identifiers (CONV-NNN, LEX-NNN, etc.) MUST be proposed via public RFC.

GOV-002. An RFC MUST be reviewed and accepted by elected maintainers before becoming normative.

GOV-003. Maintainers MUST be elected by the community; elections are held annually or as needed per the foundation charter (governance/CHARTER.md).

GOV-004. The foundation MUST guarantee explicit fork rights: any fork MUST be allowed, but MUST use a distinct name (TRADEMARK.md).

GOV-005. The specification is normative in English only. Translations exist in docs/spec-translations/ but are informative (Q5 in SPEC-00).

GOV-006. RFCs are numbered sequentially: RFC-0000 (template), RFC-0001 (language pack derivation), etc.

GOV-007. Deprecation proposals MUST follow the RFC process and reference the rule identifiers being deprecated.

GOV-008. Breaking changes (spec-MAJOR bumps) MUST be approved via RFC and accompanied by a MIGRATION-{MAJOR}.md (SPEC-12).

3. Formal grammar or tables

Table — RFC lifecycle:

| Status | Meaning |
|--------|---------|
| Proposed | RFC submitted, under review |
| Approved | Accepted by maintainers, becomes normative |
| Deprecated | Superseded by a newer RFC |
| Withdrawn | Author withdrew before approval |

4. Examples

4.1 RFC-0001: Language Pack Derivation

See rfcs/0001-language-pack-derivation.md — defines how a new language is added via a pure-data language pack (CONV-061).

5. Conformance tests

Minimum required at spec-0.1. Every file carries a # covers: GOV-NNN header per CONV-013.

| Rule | Test path |
|------|-----------|
| GOV-001 … GOV-008 | conformance/gov/001/governance.lo |

6. Interactions with other sections

SPEC-00: GOV governs how new CONV-NNN identifiers are proposed and approved.

SPEC-12: Deprecation and migration rules are proposed via RFC (GOV-007, GOV-008).

7. Open questions

Q1. Vendor extension sunset. CONV-041 allows extensions but does not define sunset. Proposal: extensions older than two spec-MINOR cycles without an accepted RFC MUST be dropped (Q2 in SPEC-00).

Q2. Spec text translation certification. Should the foundation certify normative translations? (Q5 in SPEC-00) Open for community vote after spec-1.0.

Q3. Maintainer recall. Can the community recall a maintainer between elections? Proposal: yes, via RFC with 2/3 community vote.

8. Change log

| Version | Date | Summary |
|---------|------|---------|
| 0.1 | 2026-11-20 | Initial draft. Defines RFC process, maintainer election, trademark, fork rights. |

End of SPEC-13 (spec-0.1).