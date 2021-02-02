use diesel::{MysqlConnection, RunQueryDsl};

use crate::database::models::DatabaseCommunity;

pub(crate) fn get_communities(
    conn: &MysqlConnection,
) -> Result<Vec<DatabaseCommunity>, diesel::result::Error> {
    use crate::database::schema::Communities::dsl::*;

    Communities.load::<DatabaseCommunity>(conn)
}
