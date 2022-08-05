#[derive(thiserror::Error, Debug)]
pub(crate) enum PlRustError {
    #[error("Failed pg_sys::CheckFunctionValidatorAccess")]
    CheckFunctionValidatorAccess,
    #[error("pgx::pg_sys::FunctionCallInfo was Null")]
    NullFunctionCallInfo,
    #[error("pgx::pg_sys::FmgrInfo was Null")]
    NullFmgrInfo,
    #[error("The Procedure Tuple was NULL")]
    NullProcTuple,
    #[error("The source code of the function was NULL")]
    NullSourceCode,
    #[error("libloading error: {0}")]
    LibLoading(#[from] libloading::Error),
    #[cfg(any(
        all(target_os = "macos", target_arch = "x86_64"),
        feature = "force_enable_x86_64_darwin_generations"
    ))]
    #[error("Generation error (Mac OS x86_64 specific): {0}")]
    Generation(#[from] crate::generation::Error),
    #[error("`cargo build` failed")]
    CargoBuildFail,
    #[error("Generating `Cargo.toml`")]
    GeneratingCargoToml,
    #[error("Function `{0}` was not a PL/Rust function")]
    NotPlRustFunction(pgx::pg_sys::Oid),
    #[error("Oid `{0}` was not mappable to a Rust type")]
    NoOidToRustMapping(pgx::pg_sys::Oid),
    #[error("Generated Rust type (`{1}`) for `{0}` was unparsable: {2}")]
    ParsingRustMapping(pgx::pg_sys::Oid, String, syn::Error),
    #[error("Parsing `[dependencies]` block: {0}")]
    ParsingDependenciesBlock(toml::de::Error),
    #[error("Parsing `[code]` block: {0}")]
    ParsingCodeBlock(syn::Error),
    #[error("Parsing error at span `{:?}`", .0.span())]
    Parse(#[from] syn::Error),
    #[error("Detected unsafe code in user function `{}`", .0)]
    UnsafeUserFn(String),
}
