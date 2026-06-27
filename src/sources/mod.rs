mod banks;
mod cep;
mod cnpj;
mod currency;
mod ddd;
mod domain;
mod ibge;
mod ip;
mod refs;

use crate::source::Source;

pub fn all() -> Vec<Box<dyn Source>> {
    vec![
        Box::new(cnpj::BrasilApi),
        Box::new(cnpj::MinhaReceita),
        Box::new(cep::ViaCep),
        Box::new(cep::BrasilApi),
        Box::new(ddd::BrasilApi),
        Box::new(banks::BrasilApi),
        Box::new(domain::GoogleDns),
        Box::new(domain::CrtSh),
        Box::new(domain::Rdap),
        Box::new(domain::Headers),
        Box::new(ip::IpApi),
        Box::new(ip::InternetDb),
        Box::new(ip::Rdap),
        Box::new(currency::AwesomeApi),
        Box::new(ibge::Municipios),
        Box::new(refs::Holidays),
        Box::new(refs::Isbn),
        Box::new(refs::Ncm),
    ]
}
