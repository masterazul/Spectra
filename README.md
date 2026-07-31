# spectra

[![CI](https://github.com/masterazul/Spectra/actions/workflows/ci.yml/badge.svg)](https://github.com/masterazul/Spectra/actions/workflows/ci.yml)
[![Security](https://github.com/masterazul/Spectra/actions/workflows/security.yml/badge.svg)](https://github.com/masterazul/Spectra/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/rust-2021-orange.svg)

OSINT toolkit for Brazilian public-record sources. One CLI, one JSON shape, every
source behind the same interface — plus a `verify` command that tells you which sources
are actually up *before* you run a collection.

It only touches **public, free, legal** data: government registries (CNPJ), postal codes,
bank registry, DNS, certificate transparency, RDAP, IP infrastructure metadata, exchange
rates and reference data. No leaked databases, no paid "consulta" services, no scraping
behind logins.

## Install

```
cargo build --release
```

The static binary lands at `target/release/spectra` — no runtime, no dependencies to ship.

## Usage

```
spectra <command> [value] [--json]

  cnpj <cnpj>       company registry
  cep <cep>         postal address
  ddd <ddd>         phone area code
  bank <code>       bank registry
  domain <name>     dns records, certificate transparency, rdap, headers
  ip <addr>         geolocation, asn, exposed services
  currency <pair>   exchange rate, e.g. USD-BRL
  uf <state>        municipalities of a state, e.g. RJ
  holidays <year>   national holidays
  isbn <code>       book metadata
  ncm <code>        customs product code
  lookup <value>    auto-detect the query type by shape
  validate <doc>    cpf/cnpj check digits, fully offline
  verify            health-check every registered source
  sources           list registered sources
```

Add `--json` to any command for machine-readable output — the same shape across every
source, so it drops straight into a pipeline.

## Example

```console
$ spectra validate 11144477735
cpf 11144477735: valid

$ spectra verify
[  up] cnpj-brasilapi       cnpj             142ms
[  up] cep-viacep           cep               89ms
[  up] domain-crtsh         domain           311ms
[down] ip-internetdb        ip                 0ms  connection timed out
...

$ spectra ip 8.8.8.8 --json
{"source":"ip-ipapi","kind":"ip","status":"ok","data":{ ... }}
```

`verify` and each multi-source collection probe their sources **concurrently**, so a
slow or dead endpoint never blocks the healthy ones. Every source reports a normalized
`{source, kind, status, data}` envelope — success or failure, always the same shape.

## Sources

18 sources across 11 record types, all free and public:

| kind     | sources                                              | category       |
|----------|------------------------------------------------------|----------------|
| cnpj     | BrasilAPI, Minha Receita                             | corporate      |
| cep      | ViaCEP, BrasilAPI                                    | address        |
| ddd      | BrasilAPI                                            | telecom        |
| bank     | BrasilAPI                                            | financial      |
| domain   | Google DNS (DoH), crt.sh, RDAP, HTTP headers         | infrastructure |
| ip       | ip-api, Shodan InternetDB, RDAP                      | geo / infra    |
| currency | AwesomeAPI                                           | financial      |
| uf       | IBGE (municipalities)                                | geo            |
| holidays | BrasilAPI                                            | reference      |
| isbn     | BrasilAPI                                            | reference      |
| ncm      | BrasilAPI                                            | reference      |

## Design

- **One trait, many sources.** Every source implements `Source` (`name`, `kind`,
  `category`, `probe`, `collect`). Adding a provider is a single file registered in
  `src/sources/mod.rs` — no changes to the engine.
- **Health before harvest.** `verify` runs the probe of every source and reports
  latency and reachability, so a collection run isn't wasted on a dead endpoint.
- **Offline where it can be.** `validate` checks CPF/CNPJ verifier digits with pure
  arithmetic — no network, no leak of the document to a third party.
- **Shape safety.** Results serialize through one envelope; `--json` output is stable
  across every command.

Tested end to end: `cargo test` covers document validation, shape detection, and the
source registry (unique names, every source exposes a probe).

## License

MIT
