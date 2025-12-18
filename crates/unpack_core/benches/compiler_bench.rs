use criterion::{criterion_group, criterion_main, Criterion};
use rspack_resolver::ResolveOptions;
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::runtime::Builder;
use unpack_core::{
    compiler::{Compiler, CompilerOptions, EntryItem},
    memory_manager::MemoryManager,
};

fn compiler_benchmark(c: &mut Criterion) {
    let rt = Builder::new_multi_thread()
        .enable_all()
        .disable_lifo_slot()
        .max_blocking_threads(8)
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("tokio-{}", id)
        })
        .build()
        .unwrap();

    c.bench_function("compile_react_10k", |b| {
        b.to_async(&rt).iter(|| async {
            let root = env!("CARGO_MANIFEST_DIR");
            let context = PathBuf::from(root)
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("build-tools-performance/cases/react-10k")
                .canonicalize()
                .unwrap();
            let compiler_options: CompilerOptions = CompilerOptions {
                context: context.clone().try_into().expect("expect utf8 path"),
                entry: vec![EntryItem {
                    name: "main".to_string(),
                    import: "./src/index.jsx".to_string(),
                }],
                resolve: ResolveOptions {
                    extensions: vec![".js", ".ts", ".mjs", ".jsx"]
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>(),
                    ..Default::default()
                },
                output_dir: context.join("dist").try_into().expect("expect utf8 path"),
            };
            let mut compiler = Compiler::new(Arc::new(compiler_options), vec![]);
            compiler.build().await;
        });
    });
}

criterion_group!(benches, compiler_benchmark);
criterion_main!(benches);
