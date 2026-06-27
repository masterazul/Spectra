use spectra::engine::detect;
use spectra::validate::{cnpj, cpf};

#[test]
fn accepts_valid_cpf() {
    assert!(cpf("111.444.777-35"));
    assert!(cpf("11144477735"));
}

#[test]
fn rejects_bad_cpf() {
    assert!(!cpf("111.111.111-11"));
    assert!(!cpf("111.444.777-30"));
    assert!(!cpf("123"));
}

#[test]
fn accepts_valid_cnpj() {
    assert!(cnpj("00.000.000/0001-91"));
    assert!(cnpj("11.222.333/0001-81"));
}

#[test]
fn rejects_bad_cnpj() {
    assert!(!cnpj("11.222.333/0001-80"));
    assert!(!cnpj("00000000000000"));
}

#[test]
fn detect_routes_by_shape() {
    assert_eq!(detect("00.000.000/0001-91"), Some("cnpj"));
    assert_eq!(detect("111.444.777-35"), Some("cpf"));
    assert_eq!(detect("01310-100"), Some("cep"));
    assert_eq!(detect("21"), Some("ddd"));
    assert_eq!(detect("001"), Some("bank"));
    assert_eq!(detect("8.8.8.8"), Some("ip"));
    assert_eq!(detect("2001:4860:4860::8888"), Some("ip"));
    assert_eq!(detect("example.com"), Some("domain"));
    assert_eq!(detect("lol"), None);
}

#[test]
fn source_names_are_unique() {
    let registry = spectra::sources::all();
    let mut names: Vec<&str> = registry.iter().map(|s| s.name()).collect();
    let total = names.len();
    names.sort_unstable();
    names.dedup();
    assert_eq!(total, names.len(), "duplicate source name registered");
}

#[test]
fn every_source_exposes_a_probe() {
    for source in spectra::sources::all() {
        assert!(source.probe().is_some(), "{} has no probe", source.name());
        assert!(!source.kind().is_empty());
    }
}
