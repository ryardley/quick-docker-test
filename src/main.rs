use quinn::{Endpoint, ServerConfig, ClientConfig};
use tokio;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;
use std::net::ToSocketAddrs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() {
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    if std::env::var("ROLE").unwrap_or_default() == "server" {
        println!("[{}] Server starting...", start_time);
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        let cert_der = cert.serialize_der().unwrap();
        let priv_key = cert.serialize_private_key_der();
        
        let endpoint = Endpoint::server(
            ServerConfig::with_single_cert(
                vec![rustls::Certificate(cert_der)],
                rustls::PrivateKey(priv_key)
            ).unwrap(),
            "[::]:4433".parse().unwrap()
        ).unwrap();
        
        println!("[{}] Server listening on :4433", start_time);
        
        while let Some(conn) = endpoint.accept().await {
            println!("Server received connection attempt");
            if let Ok(connection) = conn.await {
                println!("Server established connection");
                let (mut _send, mut recv) = connection.accept_bi().await.unwrap();
                let mut buf = vec![];
                while let Ok(Some(chunk)) = recv.read_chunk(2048, false).await {
                    buf.extend_from_slice(&chunk.bytes);
                }
                println!("Got: {}", String::from_utf8_lossy(&buf));
            }
        }
    } else {
        println!("[{}] Client starting...", start_time);
        
        let crypto = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(std::sync::Arc::new(danger::NoCertificateVerification))
            .with_no_client_auth();
        let client_config = ClientConfig::new(std::sync::Arc::new(crypto));
        let mut endpoint = Endpoint::client("[::]:0".parse().unwrap()).unwrap();
        endpoint.set_default_client_config(client_config);
        
        let server_addr = ("server", 4433)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();
        
        println!("[{}] Client attempting connection to {}", start_time, server_addr);
        let conn = endpoint.connect(server_addr, "localhost")
            .unwrap()
            .await
            .unwrap();
        println!("[{}] Client connected", start_time);
        let (mut send, _recv) = conn.open_bi().await.unwrap();
        println!("[{}] Client sending message", start_time);
        send.write_all(b"Hello!").await.unwrap();
        send.finish().await.unwrap();
        println!("[{}] Client finished sending", start_time);
        loop {
            sleep(Duration::from_secs(10)).await
        }
    }
}

mod danger {
    use rustls::client::ServerCertVerifier;
    use rustls::{Certificate, ServerName};
    use std::time::SystemTime;

    pub struct NoCertificateVerification;

    impl ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &Certificate,
            _intermediates: &[Certificate],
            _server_name: &ServerName,
            _scts: &mut dyn Iterator<Item = &[u8]>,
            _ocsp_response: &[u8],
            _now: SystemTime,
        ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
            Ok(rustls::client::ServerCertVerified::assertion())
        }

        fn verify_tls12_signature(
            &self,
            _message: &[u8],
            _cert: &Certificate,
            _dss: &rustls::DigitallySignedStruct,
        ) -> Result<rustls::client::HandshakeSignatureValid, rustls::Error> {
            Ok(rustls::client::HandshakeSignatureValid::assertion())
        }

        fn verify_tls13_signature(
            &self,
            _message: &[u8],
            _cert: &Certificate,
            _dss: &rustls::DigitallySignedStruct,
        ) -> Result<rustls::client::HandshakeSignatureValid, rustls::Error> {
            Ok(rustls::client::HandshakeSignatureValid::assertion())
        }
    }
}
