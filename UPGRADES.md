# Upgrades

## 0.3.3 — translator 0.3.2

The dependency on translator moves from 0.3.1 (`1b0e`) to the published 0.3.2 (`5719`), so that the
ordinary contract, the meta contract and the Nexus select one translator. No change to the authored
ethos, the generated Rust, the archive or the Datom text, and no new lock package.

## 0.3.2 — translator 0.3.1

The dependency on translator moves from 0.3.0 (`eeaa`) to the published 0.3.1
(`1b0e`), so that the ordinary contract, the meta contract and the Nexus select one
translator. No change to the authored ethos, the generated Rust, the archive or the
Datom text; the lock gains the `rustix` closure translator 0.3.1 declares on unix.

## 0.3.1 — Datom text coverage

No API change from 0.3.0 (`0e8d`). The crate gains `tests/text_roundtrip.rs`, which
writes the ordinary contract as Datom text and reads it back through the public
API: a `Furnish` query over scalar wrappers, `PublicationRevision` and
`GenerationId`, a translator `Caller` inside a refusal, and an `Owner`. The flake
gains `test-default`, the default feature selection, and `test-datom`, the `datom`
feature selection running that file.

## 0.3.0 — translator 0.3.0, Signal 10.3.0, Datom 0.33.0, Protos 0.33.0, ethos-zero 19.2.0

This is the first upgrade of the text contract from 0.2.0, so it takes a minor
version. The dependencies are repinned to translator 0.3.0 (`eeaa`), Signal 10.3.0
(`7afb`), datom-codec 0.33.0 (`eabe`) and protos 0.33.0 (`3c7a`), and the flake
generator to ethos-zero 19.2.0 (`5a5d`). The committed generated Rust is
regenerated: the integer and `String` newtypes additionally derive `PartialOrd`
and `Ord`, and the integer ones also `Copy`. The authored ethos file is unchanged.

A consumer selects this contract with translator 0.3.0, datom-codec 0.33.0 and
protos 0.33.0, so that one Signal and one translator meet in its graph.
