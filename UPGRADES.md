# Upgrades

## 0.3.0 — translator 0.3.0, Signal 10.3.0, Datom 0.33.0, Protos 0.33.0, ethos-zero 19.2.0

This is the first upgrade of the text contract from 0.2.0, so it takes a minor
version. The dependencies are repinned to translator 0.3.0 (`eeaa`), Signal 10.3.0
(`7afb`), datom-codec 0.33.0 (`eabe`) and protos 0.33.0 (`3c7a`), and the flake
generator to ethos-zero 19.2.0 (`5a5d`). The committed generated Rust is
regenerated: the integer and `String` newtypes additionally derive `PartialOrd`
and `Ord`, and the integer ones also `Copy`. The authored ethos file is unchanged.

A consumer selects this contract with translator 0.3.0, datom-codec 0.33.0 and
protos 0.33.0, so that one Signal and one translator meet in its graph.
