lalrpop_mod!(
    #[allow(clippy)]
    equalizer_apo,
    "/parsing/equalizer_apo.rs"
);

pub use self::equalizer_apo::MainParser as EqualizerApoParser;

// TODO: test parsing filters with real examples
