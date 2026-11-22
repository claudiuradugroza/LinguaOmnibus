SPEC-08: LinguaDB

Version: 0.1 (Draft)
Status: Draft — open for RFC review
Dependencies: spec/00-conventions.md (spec-0.1), spec/01-lexical.md (spec-0.1), spec/02-grammar.md (spec-0.1), spec/03-types.md (spec-0.1), spec/06-lir.md (spec-0.1)
Applies to: every LinguaDB implementation, every .lo source file using database operations

1. Motivation

LinguaDB is the integrated ACID database bundled with Lingua Omnibus. It provides create/insert/select/update operations with full ACID transaction guarantees. This document defines:

    the LinguaDB query language,

    the database schema,

    the storage layer,

    the client protocol (how LO programs talk to LinguaDB).

2. Normative specification

LDB-001. LinguaDB MUST provide create, insert, select, update operations on tables.

LDB-002. Transactions MUST be ACID: Atomicity, Consistency, Isolation, Durability.

LDB-003. A table schema MUST declare column names and types (SPEC-03 types).

LDB-004. The query language MUST support SELECT with WHERE, INSERT, UPDATE, DELETE.

LDB-005. Database operations require the KW_DATABASE effect (SPEC-05).

LDB-006. Concurrent access MUST be handled with row-level or table-level locking.

LDB-007. The database file format MUST be documented and open (Principle 8).

LDB-008. LinguaDB MUST support embedded (in-process) mode and client-server mode.

3. Formal grammar or tables

query          = select_stmt | insert_stmt | update_stmt | delete_stmt ;
select_stmt    = "SELECT" column_list "FROM" table_name ( "WHERE" condition )? ;
insert_stmt    = "INSERT" "INTO" table_name "(" column_list ")" "VALUES" "(" value_list ")" ;
update_stmt    = "UPDATE" table_name "SET" assignment_list "WHERE" condition ;
delete_stmt    = "DELETE" "FROM" table_name "WHERE" condition ;

Table — ACID properties:

| Property | Requirement |
|----------|-------------|
| Atomicity | Transaction either fully commits or fully rolls back |
| Consistency | Transaction brings DB from one valid state to another |
| Isolation | Concurrent transactions do not interfere |
| Durability | Committed transactions survive crashes |

4. Examples

4.1 Create table and insert — EN and RO

#lingua: EN
db create table users (id Integer, name String)
db insert into users (id, name) values (1, "Alice")

#lingua: RO
db create table utilizatori (id Integer, name String)
db insert into utilizatori (id, name) values (1, "Alice")

4.2 Select with where

#lingua: EN
var result = db select * from users where id = 1

5. Conformance tests

Minimum required at spec-0.1. Every file carries a # covers: LDB-NNN header per CONV-013.

| Rule | Test path |
|------|-----------|
| LDB-001 … LDB-008 | conformance/ldb/001/linguadb.lo |

6. Interactions with other sections

SPEC-00: LDB rules reify the LDB prefix (CONV-020).

SPEC-03: Table column types are SPEC-03 types.

SPEC-05: KW_DATABASE effect required for database operations (EFF-005).

SPEC-06 (LIR): Database operations compile to LIR instructions.

SPEC-09 (TOOL): lopm manages database dependencies; loc compiles db blocks.

7. Open questions

Q1. SQLite underneath. Should LinguaDB use SQLite as the storage engine? Proposal: embedded mode uses a custom storage format; SQLite compatibility as an option.

Q2. Query language. Is SQL-like syntax appropriate, or should we define a LO-native query language? Proposal: SQL-like for familiarity; LO-native in spec-1.0.

Q3. Distributed transactions. Should LinguaDB support distributed ACID across nodes? Proposal: defer to spec-1.0.

8. Change log

| Version | Date | Summary |
|---------|------|---------|
| 0.1 | 2026-11-20 | Initial draft. Defines LinguaDB query language, schema, ACID, client protocol. |

End of SPEC-08 (spec-0.1).