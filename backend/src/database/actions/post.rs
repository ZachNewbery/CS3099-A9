use crate::database::models::{DatabaseCommunity, DatabasePost};
use diesel::prelude::*;

pub(crate) fn get_posts_of_community(
    conn: &MysqlConnection,
    community: &DatabaseCommunity,
) -> Result<Option<Vec<DatabasePost>>, diesel::result::Error> {
    use crate::database::schema::Posts::dsl::*;
    Posts
        .filter(communityId.eq(community.id))
        .load(conn)
        .optional()
}
