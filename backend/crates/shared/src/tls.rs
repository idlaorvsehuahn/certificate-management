use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

use axum::Router;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;

/// Start an HTTPS server using tokio-rustls and hyper.
pub async fn serve_tls(
    addr: std::net::SocketAddr,
    router: Router,
    tls_config: Arc<ServerConfig>,
    mut shutdown: impl std::future::Future<Output = ()> + Unpin + Send + 'static,
) -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let acceptor = tokio_rustls::TlsAcceptor::from(tls_config);
    let make_service = router.into_make_service_with_connect_info::<std::net::SocketAddr>();

    loop {
        tokio::select! {
            res = listener.accept() => {
                let (stream, peer_addr) = match res {
                    Ok(val) => val,
                    Err(err) => {
                        tracing::error!("Failed to accept TCP connection: {}", err);
                        continue;
                    }
                };

                let acceptor = acceptor.clone();
                let make_service = make_service.clone();

                tokio::spawn(async move {
                    let tls_stream = match acceptor.accept(stream).await {
                        Ok(s) => s,
                        Err(e) => {
                            tracing::error!("TLS handshake failed: {}", e);
                            return;
                        }
                    };

                    // Extract peer certificates for client cert verification (mTLS)
                    let (_, session) = tls_stream.get_ref();
                    let peer_certs = session.peer_certificates().map(|certs| certs.to_vec());

                    let io = hyper_util::rt::TokioIo::new(tls_stream);

                    // Call the axum Router service
                    let service = tower::service_fn(move |mut req: axum::http::Request<hyper::body::Incoming>| {
                        let peer_certs = peer_certs.clone();
                        let mut make_service = make_service.clone();
                        async move {
                            if let Some(ref certs) = peer_certs {
                                req.extensions_mut().insert(certs.clone());
                            }
                            req.extensions_mut().insert(peer_addr);
                            let req = req.map(axum::body::Body::new);
                            
                            use tower::Service;
                            let mut svc = make_service.call(peer_addr).await.unwrap();
                            svc.call(req).await
                        }
                    });

                    if let Err(err) = hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new())
                        .serve_connection(io, hyper_util::service::TowerToHyperService::new(service))
                        .await
                    {
                        tracing::debug!("Error serving TLS connection: {}", err);
                    }
                });
            }
            _ = &mut shutdown => {
                tracing::info!("Shutdown signal received. Stopping TCP listener.");
                break;
            }
        }
    }
    Ok(())
}

pub fn load_certs<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<CertificateDer<'static>>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let certs = rustls_pemfile::certs(&mut reader)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(certs)
}

pub fn load_key<P: AsRef<Path>>(path: P) -> std::io::Result<PrivateKeyDer<'static>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let key = rustls_pemfile::private_key(&mut reader)?
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "private key not found"))?;
    Ok(key)
}
