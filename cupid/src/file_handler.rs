use crate::*;

build_struct! {
    #[derive(Debug, Clone, Default)]
    pub FileHandlerBuilder => pub FileHandler {
        pub path: String,
        pub contents: String,
        pub parser: CupidParser,
        pub scope: Env,
        pub errors: Vec<Error>,
        pub files: Vec<String>,
    }
}

impl FileHandler {
    
}