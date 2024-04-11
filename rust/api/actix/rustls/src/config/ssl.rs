use std::{fs::File, io::BufReader};

use rustls::{pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer}, Error, server::ServerConfig};

pub struct CertificatePair {
    certs: Vec<CertificateDer<'static>>,
    key: PrivatePkcs8KeyDer<'static>,
}

impl CertificatePair {
    pub fn new(cert_path: &str, key_path: &str) -> CertificatePair {
        let mut certs_file: BufReader<File> = BufReader::new(File::open(cert_path).expect("Failed to load cert file"));
        let mut key_file: BufReader<File> = BufReader::new(File::open(key_path).expect("Failed to load cert file"));

        let tls_certs: Vec<CertificateDer<'static>> = rustls_pemfile::certs(&mut certs_file)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let tls_key: PrivatePkcs8KeyDer<'static> = rustls_pemfile::pkcs8_private_keys(&mut key_file)
        .next()
        .unwrap()
        .unwrap();

        CertificatePair { certs: tls_certs, key: tls_key }
    }

    pub fn get_server_config(mut self) -> ServerConfig {
    rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(self.certs, PrivateKeyDer::Pkcs8(self.key)).unwrap()
    }
}
