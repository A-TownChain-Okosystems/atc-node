# SYNC-STATUS — atc-node

**Stand:** 08.11.2026

## Status: Mirror ohne kanonischen Code

`atc-node` ist **nicht** mehr die Quelle der Node-Implementierung. Kanonisch ist:

```
A-TownChain-Okosystems/a-townchain → components/node
```

## Drift (belegt am 08.11.2026)

| Prüfpunkt | kanonisch (`a-townchain/components/node`) | hier (`atc-node`) |
|---|---|---|
| src-Dateien | 13 | 9 |
| `mempool.rs` (Tx-Prüfung, Nonce, Replay) | vorhanden | fehlt |
| `did.rs` (DID-Registry, Rotation) | vorhanden | fehlt |
| `validator.rs` (ECDSA-Proposals) | vorhanden | fehlt |
| `discovery.rs` | vorhanden | fehlt |
| `chain.rs` | 256-Bit-Hashes, `produce_tx_block`/`import_tx_block` | 208 Zeilen, alte Fassung |
| Sicherheits-Tests | 17 (Tx-Blöcke, Hashbreite, DID) | keine |

## Warum hier nicht portiert wird

Die Node-Härtung (SEC-P0-001/003/004/007) setzt auf dem Wallet-Crate auf
(`verify_bound`, `address_from_pubkey`, ECDSA über den kanonischen Tx-Digest).
Dieses Repo bezieht Abhängigkeiten per Git-Pin, nicht per Pfad:

```toml
atc-algorithm = { git = ".../atc-algorithm", rev = "a9f81887..." }
```

Das passende Pendant `atc-wallet` ist **archiviert** (read-only) und enthält die
gehärtete Wallet-Fassung nicht. Eine Git-Abhängigkeit auf den Monorepo-Pfad ist
mit Cargo nicht auflösbar, solange `a-townchain` kein Root-Workspace-Manifest hat.
Ein Port müsste den Wallet-Code hier duplizieren — genau die Doppelablage, die im
Ablage-Audit als Risiko geführt wird.

## Reaktivierung (falls Standalone-Releases wieder gewollt sind)

1. `a-townchain` um ein Root-`[workspace]` mit `members = ["components/*"]` erweitern.
2. In diesem Repo: `atc-wallet = { git = ".../a-townchain", package = "atc_wallet", rev = "<sha>" }`.
3. Danach `mempool.rs`, `did.rs`, `validator.rs`, `discovery.rs`, die gehärteten
   `chain.rs`/`bootstrap.rs`/`gossip.rs`/`main.rs` und die drei Testdateien übernehmen.

## Referenzen

- Befund: `a-townchain-ecosystem/docs/audit/2026-11-08-placement-audit.md`
- Fix-Stand: `a-townchain` main, Commits bis `e1c6b40`
- Register: `a-townchain/FILE_REGISTER.md`, Nachtrag 2026-11-08
