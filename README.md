# spectra

[![CI](https://github.com/masterazul/Spectra/actions/workflows/ci.yml/badge.svg)](https://github.com/masterazul/Spectra/actions/workflows/ci.yml)
[![Security](https://github.com/masterazul/Spectra/actions/workflows/security.yml/badge.svg)](https://github.com/masterazul/Spectra/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/rust-2021-orange.svg)

Looking up one Brazilian company used to mean BrasilAPI in one tab, ViaCEP in another,
crt.sh in a third, and then reconciling four different JSON shapes by hand. spectra puts
18 public sources behind a single command and hands them all back in the same envelope.

The part I reach for most is `verify`. It probes every registered source and reports what
is actually reachable, so a collection doesn't die halfway through on an endpoint that was
already down when you started.

Want to try it before cloning? `validate` runs in the browser at
[masterazul.github.io/public/projetos](https://masterazul.github.io/public/projetos/index.html).
That page loads this very `src/validate.rs` compiled to WebAssembly — 18 KB, no dependencies,
and the document never leaves the tab, because check digits are arithmetic, not a lookup.

Only public, free, legal data: government registries, postal codes, DNS, certificate
transparency, RDAP, IP metadata, exchange rates. Nothing leaked, nothing paid, nothing
behind a login.

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
[  up] cnpj-brasilapi       cnpj             243ms
[  up] cep-viacep           cep              534ms
[  up] domain-crtsh         domain         10853ms
[down] ip-rdap              ip             12019ms  transport error
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

11 tests cover the parts worth pinning: check-digit math, shape detection, the source
registry, and the host guard that keeps a lookup from being steered at an internal address.

## Hardening

The pipeline is scoped, not just green.

- Actions run from a commit digest. A tag can be moved; a digest cannot.
- Workflows declare `permissions: contents: read`, so `GITHUB_TOKEN` has nothing to write with.
- `cargo audit --deny warnings` runs on every push and again weekly. Dependabot watches the
  crates and the pinned digests alike.
- `#![forbid(unsafe_code)]` on both crate roots — enforced by the compiler, not by habit.
- Release builds keep overflow checks. Paired with `panic = "abort"`, an overflow stops the
  process instead of wrapping into a wrong answer.
- `gitleaks` reads the full history on every push.

## License

MIT
