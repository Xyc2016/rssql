pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

#[derive(Debug)]
pub struct Entity {
    pub id: u32,
    pub username: String,
    pub email: String,
}

pub enum Statement {
    StatementInsert(Entity),
    StatementSelect,
}
