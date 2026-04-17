use rustls::ClientConfig;
use webpki_roots::TLS_SERVER_ROOTS;

pub fn build_chrome_config() -> ClientConfig {
    let mut root_cert_store = rustls::RootCertStore::empty();
    root_cert_store.extend(
        TLS_SERVER_ROOTS
            .iter()
            .cloned()
    );

    // Initializing rustls purely with default crypto providers
    // (BoringSSL/utls hook will replace this later for exact JA3 byte ordering)
    let mut config = rustls::ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    // Force ALPN to support h2 and http/1.1 (Chrome standard ALPN)
    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    config
}
