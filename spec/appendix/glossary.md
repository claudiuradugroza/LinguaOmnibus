# Appendix — Glossary

Normative glossary for the Lingua Omnibus specification set. English terms are normative; translations are normative for human-facing messages.

| Term | Definition | RO | FR | DE | ES | JA |
|------|------------|-----|-----|-----|-----|-----|
| program | A .lo source file or a set of them linked into a build unit. | program | programme | Programm | programa | プログラム |
| language pack | A JSON+test directory enabling one natural language (CONV-060). | pachet lingvistic | pack linguistique | Sprachpaket | paquete lingüístico | 言語パック |
| LIR | Lingua Intermediate Representation — the universal semantic form (SPEC-06). | LIR | LIR | LIR | LIR | LIR |
| unified AST | The tree produced by every language pack's parser, structurally identical across languages. | AST unificat | AST unifié | vereinheitlichter AST | AST unificado | 統一AST |
| conforming implementation | Defined in CONV-040. | implementare conformă | implémentation conforme | konforme Implementierung | implementación conforme | 適合実装 |
| conformance test | A .lo file under conformance/ exercising one or more rule identifiers. | test de conformanță | test de conformité | Konformitätstest | prueba de conformidad | 適合性テスト |
| language directive | The #lingua: XX comment declaring source language. | directivă de limbă | directive de langue | Sprachdirektive | directiva de idioma | 言語ディレクティブ |
| keyword slot | An abstract token identifier such as KW_IF mapped by a language pack. | slot de cuvânt-cheie | emplacement de mot-clé | Schlüsselwort-Slot | ranura de palabra clave | キーワードスロット |
| language pack version | ISO date of a pack release (CONV-032). | versiune de pachet | version de pack | Pack-Version | versión de paquete | パックバージョン |
| extension | Implementation-specific behaviour outside the spec, gated by CONV-041. | extensie | extension | Erweiterung | extensión | 拡張 |
| capability | A declared permission required by effects (SPEC-05, SPEC-11). | capabilitate | capacité | Fähigkeit | capacidad | 権限 |
| qubit | A linear quantum resource (SPEC-03, SPEC-06). | qubit | qubit | Qubit | cúbit | 量子ビット |
| effect | A declared side-effect class in a function signature (SPEC-05). | efect | effet | Effekt | efecto | エフェクト |
| lifetime | A lexical region governing a value's validity (SPEC-04). | durată de viață | durée de vie | Lebensdauer | tiempo de vida | ライフタイム |
| ownership | The single-owner rule for non-Copy values (SPEC-04). | proprietate | propriété | Eigentum | propiedad | 所有権 |
| borrow | A scoped non-owning reference (SPEC-04). | împrumut | emprunt | Ausleihe | préstamo | 借用 |
| trait/interface | A named set of method signatures. | interfață | interface | Schnittstelle | interfaz | インターフェース |
| generic | A parameter over types or lifetimes (SPEC-03). | generic | générique | generisch | genérico | ジェネリック |
| module | A named compilation unit inside a package (SPEC-09). | modul | module | Modul | módulo | モジュール |
| package | A distributable unit tracked by lopm (SPEC-09). | pachet | paquet | Paket | paquete | パッケージ |
| registry | A federated index of packages (Principle 5, CONV-080). | registru | registre | Registrierung | registro | レジストリ |
| reproducible build | Bit-identical rebuild from the same source and toolchain (Principle 3). | build reproductibil | build reproductible | reproduzierbarer Build | build reproducible | 再現可能ビルド |
| bootstrap chain | The minimal path from a bootstrap compiler to a self-hosted loc (Principle 4). | lanț de bootstrap | chaîne d'amorçage | Bootstrap-Kette | cadena de arranque | ブートストラップ連鎖 |
| offline-first | Tooling must function with no network (Principle 6). | offline-first | hors-ligne d'abord | Offline-First | offline-first | オフラインファースト |
| telemetry | Any data sent by a tool to a remote endpoint. Default: forbidden. | telemetrie | télémétrie | Telemetrie | telemetría | テレメトリ |
| self-hosted registry | A lopm registry run by anyone (CONV-080). | registru self-hosted | registre auto-hébergé | selbstgehostete Registrierung | registro autoalojado | セルフホストレジストリ |