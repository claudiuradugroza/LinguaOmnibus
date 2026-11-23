# Lingua Omnibus — Language Packs

## Status

| Language | ISO | Script | Tested | Status |
|----------|-----|--------|--------|--------|
| English | en | Latin | ✅ | Complete |
| Romanian | ro | Latin | ✅ | Complete |
| French | fr | Latin | ❌ | Stub |
| German | de | Latin | ❌ | Stub |
| Spanish | es | Latin | ❌ | Stub |
| Japanese | ja | CJK | ❌ | Stub |
| Portuguese | pt | Latin | ❌ | Stub |
| Italian | it | Latin | ❌ | Stub |
| Dutch | nl | Latin | ❌ | Stub |
| Polish | pl | Latin | ❌ | Stub |
| Russian | ru | Cyrillic | ❌ | Stub |
| Chinese | zh | CJK | ❌ | Stub |
| Korean | ko | Hangul | ❌ | Stub |
| Arabic | ar | Arabic | ❌ | Stub (RTL) |
| Hindi | hi | Devanagari | ❌ | Stub |
| Turkish | tr | Latin | ❌ | Stub |
| Swedish | sv | Latin | ❌ | Stub |
| Danish | da | Latin | ❌ | Stub |
| Norwegian | no | Latin | ❌ | Stub |
| Finnish | fi | Latin | ❌ | Stub |
| Czech | cs | Latin | ❌ | Stub |
| Hungarian | hu | Latin | ❌ | Stub |
| Greek | el | Greek | ❌ | Stub |
| Hebrew | he | Hebrew | ❌ | Stub (RTL) |
| Thai | th | Thai | ❌ | Stub |
| Vietnamese | vi | Latin | ❌ | Stub |
| Indonesian | id | Latin | ❌ | Stub |
| Ukrainian | uk | Cyrillic | ❌ | Stub |
| Bulgarian | bg | Cyrillic | ❌ | Stub |
| Croatian | hr | Latin | ❌ | Stub |

## Note

**EN and RO are complete and tested.** The remaining 28 packs are stubs that demonstrate the schema. If you speak one of these languages, you can contribute by completing `keywords.json` and adding tests.

## Contributing a language pack

1. Create `langpacks/{iso}/` directory
2. Add `keywords.json` conforming to `spec/appendix/langpack-schema.json`
3. Add `grammar.json` (see existing packs for template)
4. Add `grammar-notes.md` (non-normative)
5. Add `tests/` with at least one test per keyword
6. Register version (ISO 8601 date) in keywords.json

See RFC-0001 for details.