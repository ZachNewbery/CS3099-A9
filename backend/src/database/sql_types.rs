use diesel_derive_enum::*;

#[derive(DbEnum, Debug, Copy, Clone)]
// WARNING: Super evil. What this does is rename the enum to be called "Enum" inside Diesel.
// When the schema is generated the column type will be "Enum". However, they do not actually exist
// in diesel::mysql::MysqlType. Therefore, without manual renaming each time it will fail to compile.
// **THIS WILL EXPLODE IF WE EVER USE ANOTHER ENUM!**
#[DieselType = "Enum"]
pub enum DatabaseContentType {
    Text,
    Markdown
}
