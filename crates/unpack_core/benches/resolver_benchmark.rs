use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rspack_resolver::ResolveOptions;
use std::path::PathBuf;
use unpack_core::resolver::UnpackResolver;

fn benchmark_resolver_creation(c: &mut Criterion) {
    c.bench_function("resolver creation", |b| {
        b.iter(|| {
            let options = ResolveOptions {
                extensions: vec![".js", ".ts", ".mjs", ".jsx"]
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>(),
                ..Default::default()
            };
            black_box(UnpackResolver::new(options))
        })
    });
}

fn benchmark_simple_resolve(c: &mut Criterion) {
    let options = ResolveOptions {
        extensions: vec![".js", ".ts", ".mjs", ".jsx"]
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>(),
        ..Default::default()
    };
    let resolver = UnpackResolver::new(options);
    
    // Use a temporary directory for testing
    let temp_dir = std::env::temp_dir();
    let context = temp_dir.to_str().unwrap();
    
    c.bench_function("simple resolve", |b| {
        b.iter(|| {
            // Attempt to resolve a module (will fail but we're benchmarking the operation)
            let _ = resolver.resolve(
                black_box(camino::Utf8Path::new(context)),
                black_box("./test")
            );
        })
    });
}

criterion_group!(benches, benchmark_resolver_creation, benchmark_simple_resolve);
criterion_main!(benches);
