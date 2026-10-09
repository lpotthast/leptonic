//! Prepare compressed WASM before accepting requests, in every server build profile.

use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Write},
    path::Path,
};

/// Reuse fresh sidecars (including production's quality-11 Brotli files). Development bundles get gzip at level 6
/// and Brotli at quality 4 once per build, rather than expensive compression on every page load.
pub(super) fn precompress_wasm(pkg_dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(pkg_dir)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            precompress_wasm(&path)?;
        } else if path.extension().is_some_and(|ext| ext == "wasm") {
            write_sidecar(&path, "wasm.gz", |input, output| {
                let mut encoder =
                    flate2::write::GzEncoder::new(output, flate2::Compression::new(6));
                io::copy(input, &mut encoder)?;
                encoder.finish()?;
                Ok(())
            })?;
            write_sidecar(&path, "wasm.br", |input, output| {
                brotli::BrotliCompress(
                    input,
                    output,
                    &brotli::enc::BrotliEncoderParams {
                        quality: 4,
                        ..Default::default()
                    },
                )?;
                Ok(())
            })?;
        }
    }
    Ok(())
}

fn write_sidecar(
    source: &Path,
    extension: &str,
    compress: impl FnOnce(&mut BufReader<File>, &mut BufWriter<&mut File>) -> io::Result<()>,
) -> io::Result<()> {
    let destination = source.with_extension(extension);
    let mut input = BufReader::new(File::open(source)?);
    let modified = input.get_ref().metadata()?.modified()?;
    match fs::metadata(&destination) {
        Ok(metadata) if metadata.modified()? >= modified => return Ok(()),
        Ok(_) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }

    // Publish only complete streams, even if another server uses the same site directory.
    let mut temporary = tempfile::NamedTempFile::new_in(source.parent().unwrap_or(Path::new(".")))?;
    {
        let mut output = BufWriter::new(temporary.as_file_mut());
        compress(&mut input, &mut output)?;
        output.flush()?;
    }
    temporary
        .persist(destination)
        .map_err(|error| error.error)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs::FileTimes, io::Read, time::SystemTime};

    use assertr::prelude::*;
    use axum::{
        Router,
        body::{Body, to_bytes},
        http::{Request, StatusCode, header},
    };
    use leptos::prelude::LeptosOptions;
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn wasm_sidecars_are_negotiated_and_decode_to_the_bundle() {
        let site = tempfile::tempdir().unwrap();
        let pkg = site.path().join("pkg");
        fs::create_dir(&pkg).unwrap();
        let original = b"\0asm\x01\0\0\0".repeat(2048);
        fs::write(pkg.join("book.content-hash.wasm"), &original).unwrap();
        precompress_wasm(&pkg).unwrap();

        let options = LeptosOptions::builder()
            .output_name("book")
            .site_root(site.path().to_str().unwrap())
            .build();
        let app = Router::new()
            .fallback(leptos_axum::file_and_error_handler(book_ssr::app::shell))
            .with_state(options)
            .layer(axum::middleware::from_fn(crate::cache_hashed_files));

        for (accept, encoding) in [
            ("br, gzip", Some("br")),
            ("gzip", Some("gzip")),
            ("br;q=0, gzip", Some("gzip")),
            ("identity", None),
        ] {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/pkg/book.content-hash.wasm")
                        .header(header::ACCEPT_ENCODING, accept)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_that!(response.status()).is_equal_to(StatusCode::OK);
            assert_that!(response.headers()[header::CONTENT_TYPE].to_str().unwrap())
                .is_equal_to("application/wasm");
            assert_that!(response.headers()[header::CACHE_CONTROL].to_str().unwrap())
                .is_equal_to("public, max-age=31536000, immutable");
            assert_that!(
                response
                    .headers()
                    .get(header::CONTENT_ENCODING)
                    .map(|value| value.to_str().unwrap())
            )
            .is_equal_to(encoding);

            let body = to_bytes(response.into_body(), original.len())
                .await
                .unwrap();
            let mut decoded = Vec::new();
            match encoding {
                Some("br") => {
                    brotli::Decompressor::new(body.as_ref(), 4096)
                        .read_to_end(&mut decoded)
                        .unwrap();
                }
                Some("gzip") => {
                    flate2::read::GzDecoder::new(body.as_ref())
                        .read_to_end(&mut decoded)
                        .unwrap();
                }
                _ => decoded.extend_from_slice(&body),
            }
            assert_that!(decoded).is_equal_to(original.clone());
        }
    }

    #[test]
    fn fresh_sidecars_are_reused_and_stale_sidecars_are_replaced() {
        let pkg = tempfile::tempdir().unwrap();
        // Split bundles can live below pkg; prepare those too.
        let chunks = pkg.path().join("chunks");
        fs::create_dir(&chunks).unwrap();
        let wasm = chunks.join("chunk.wasm");
        let original = b"\0asm\x01\0\0\0".repeat(2048);
        fs::write(&wasm, &original).unwrap();
        precompress_wasm(pkg.path()).unwrap();

        for extension in ["wasm.gz", "wasm.br"] {
            let sidecar = wasm.with_extension(extension);
            let modified = fs::metadata(&sidecar).unwrap().modified().unwrap();
            precompress_wasm(pkg.path()).unwrap();
            assert_that!(fs::metadata(&sidecar).unwrap().modified().unwrap()).is_equal_to(modified);

            fs::write(&sidecar, "stale").unwrap();
            File::options()
                .write(true)
                .open(&sidecar)
                .unwrap()
                .set_times(FileTimes::new().set_modified(SystemTime::UNIX_EPOCH))
                .unwrap();
            precompress_wasm(pkg.path()).unwrap();
            let compressed = fs::read(&sidecar).unwrap();
            let mut decoded = Vec::new();
            if extension == "wasm.gz" {
                flate2::read::GzDecoder::new(compressed.as_slice())
                    .read_to_end(&mut decoded)
                    .unwrap();
            } else {
                brotli::Decompressor::new(compressed.as_slice(), 4096)
                    .read_to_end(&mut decoded)
                    .unwrap();
            }
            assert_that!(decoded).is_equal_to(original.clone());
        }
    }
}
