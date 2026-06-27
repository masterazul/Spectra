# spectra

OSINT toolkit for Brazilian public-record sources. One CLI, one JSON shape, every
source behind the same interface — plus a `verify` command that tells you which sources
are actually up before you run a collection.

It only touches **public, free, legal** data: government registries (CNPJ), postal codes,
bank registry, DNS, certificate transparency and infrastructure metadata. No leaked
databases, no paid "consulta" services.

## Build

```
cargo build --release
```

The binary lands at `target/release/spectra`.

## Usage

```
spectra cnpj 00000000000191
spectra cep 01310100
spectra ddd 21
spectra bank 001
spectra domain github.com
spectra ip 8.8.8.8
spectra lookup 11222333000181     # auto-detects the query type
spectra validate 11144477735      # CPF/CNPJ check digits, offline
spectra verify                    # health-check every source
spectra sources                   # list registered sources
```

Add `--json` to any command for machine-readable output.

## Sources

| kind   | sources                          |
|--------|----------------------------------|
| cnpj   | BrasilAPI, Minha Receita         |
| cep    | ViaCEP, BrasilAPI                |
| ddd    | BrasilAPI                        |
| bank   | BrasilAPI                        |
| domain | Google DNS (DoH), crt.sh         |
| ip     | ip-api, Shodan InternetDB        |

New sources are a single file implementing the `Source` trait, added to
`src/sources/mod.rs`.

## License

MIT
