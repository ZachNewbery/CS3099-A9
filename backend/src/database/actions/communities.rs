//! Database actions concerning Communities
use diesel::prelude::*;
use diesel::BelongingToDsl;

use crate::database::models::*;
use crate::util::UserDetail;

/// Returns all of the stored communities in the Communities table
pub(crate) fn get_communities(
    conn: &MysqlConnection,
) -> Result<Vec<DatabaseCommunity>, diesel::result::Error> {
    use crate::database::schema::Communities::dsl::*;

    Communities.load::<DatabaseCommunity>(conn)
}

/// Returns a vector of admins for a given Community
pub(crate) fn get_community_admins(
    conn: &MysqlConnection,
    community: &DatabaseCommunity,
) -> Result<Vec<(DatabaseUser, UserDetail)>, diesel::result::Error> {
    let local_admins: Vec<(DatabaseUser, DatabaseLocalUser)> = {
        use crate::database::schema::LocalUsers::dsl::*;
        use crate::database::schema::Users::dsl::*;

        DatabaseCommunitiesUser::belonging_to(community)
            .inner_join(Users.inner_join(LocalUsers))
            .select((Users::all_columns(), LocalUsers::all_columns()))
            .load(conn)
    }?;

    let federated_admins: Vec<(DatabaseUser, DatabaseFederatedUser)> = {
        use crate::database::schema::FederatedUsers::dsl::*;
        use crate::database::schema::Users::dsl::*;

        DatabaseCommunitiesUser::belonging_to(community)
            .inner_join(Users.inner_join(FederatedUsers))
            .select((Users::all_columns(), FederatedUsers::all_columns()))
            .load(conn)
    }?;

    let mut v = vec![];
    v.append(
        &mut local_admins
            .into_iter()
            .map(|(u, l)| (u, l.into()))
            .collect(),
    );
    v.append(
        &mut federated_admins
            .into_iter()
            .map(|(u, f)| (u, f.into()))
            .collect(),
    );

    Ok(v)
}

/// Returns a Community given its name (id as per supergroup spec)
pub(crate) fn get_community_by_id(
    conn: &MysqlConnection,
    id_: &str,
) -> Result<Option<DatabaseCommunity>, diesel::result::Error> {
    use crate::database::schema::Communities::dsl::*;
    Communities
        .filter(name.eq(id_))
        .first::<DatabaseCommunity>(conn)
        .optional()
}

/// Inserts a new community into the Communities table
pub(crate) fn put_community(
    conn: &MysqlConnection,
    new_community: DatabaseNewCommunity,
) -> Result<DatabaseCommunity, diesel::result::Error> {
    use crate::database::schema::Communities::dsl::*;

    let community_name = new_community.name.clone();

    diesel::insert_into(Communities)
        .values(new_community)
        .execute(conn)?;

    Communities
        .filter(name.eq(community_name))
        .first::<DatabaseCommunity>(conn)
}

/// Sets the admins for a new community
pub(crate) fn set_community_admins(
    conn: &MysqlConnection,
    community: &DatabaseCommunity,
    admin_list: Vec<DatabaseLocalUser>,
) -> Result<(), diesel::result::Error> {
    use crate::database::schema::CommunitiesUsers::dsl::*;

    let admins = admin_list
        .into_iter()
        .map(|a| DatabaseNewCommunitiesUser {
            community_id: community.id,
            user_id: a.user_id,
        })
        .collect::<Vec<DatabaseNewCommunitiesUser>>();

    diesel::delete(CommunitiesUsers)
        .filter(communityId.eq(community.id))
        .execute(conn)?;

    diesel::insert_into(CommunitiesUsers)
        .values(admins)
        .execute(conn)?;

    Ok(())
}

/// Removes a Community from the database
pub(crate) fn remove_community(
    conn: &MysqlConnection,
    community: DatabaseCommunity,
) -> Result<(), diesel::result::Error> {
    // Remove all Contents

    // Text
    {
        use crate::database::schema::Posts::dsl::*;

        diesel::delete(DatabaseText::belonging_to(
            &Posts.filter(communityId.eq(community.id)).load(conn)?,
        ))
        .execute(conn)?;
    }

    // Markdown
    {
        use crate::database::schema::Posts::dsl::*;

        diesel::delete(DatabaseMarkdown::belonging_to(
            &Posts.filter(communityId.eq(community.id)).load(conn)?,
        ))
        .execute(conn)?;
    }

    // Remove all comments
    {
        use crate::database::schema::Posts::dsl::*;
        diesel::delete(Posts)
            .filter(communityId.eq(community.id))
            .filter(parentId.is_not_null())
            .execute(conn)?;
    }

    // Remove all posts
    {
        use crate::database::schema::Posts::dsl::*;
        diesel::delete(Posts)
            .filter(communityId.eq(community.id))
            .execute(conn)?;
    }

    // Remove all admins
    {
        use crate::database::schema::CommunitiesUsers::dsl::*;
        diesel::delete(CommunitiesUsers)
            .filter(communityId.eq(community.id))
            .execute(conn)?;
    }

    // Remove community itself
    {
        diesel::delete(&community).execute(conn)?;
    }

    Ok(())
}

/// Updates the title of a given Community
pub(crate) fn update_community_title(
    conn: &MysqlConnection,
    mut community: DatabaseCommunity,
    new_title: &str,
) -> Result<DatabaseCommunity, diesel::result::Error> {
    use crate::database::schema::Communities::dsl::*;

    diesel::update(&community)
        .set(title.eq(new_title))
        .execute(conn)?;

    community.title = new_title.to_string();

    Ok(community)
}

/// Updates the description of a given Community
pub(crate) fn update_community_description(
    conn: &MysqlConnection,
    mut community: DatabaseCommunity,
    new_description: &str,
) -> Result<DatabaseCommunity, diesel::result::Error> {
    use crate::database::schema::Communities::dsl::*;

    diesel::update(&community)
        .set(description.eq(new_description))
        .execute(conn)?;

    community.description = new_description.to_string();

    Ok(community)
}
