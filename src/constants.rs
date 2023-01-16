pub const PROMPT: &str = "rssql> ";
pub mod command {
    pub const EXIT: &str = ".exit";
}
pub mod statement {
    pub mod tag {
        pub const INSERT: &str = "insert";
        pub const SELECT: &str = "select";
    }
}
