use diesel::{MysqlConnection, RunQueryDsl};

use crate::database::models::Community;

pub(crate) fn get_communities(
    conn: &MysqlConnection,
) -> Result<Vec<Community>, diesel::result::Error> {
    use crate::database::schema::Communities::dsl::*;

    Communities.load::<Community>(conn)
}
