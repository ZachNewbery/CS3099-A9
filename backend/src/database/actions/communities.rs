use diesel::prelude::*;
use diesel::BelongingToDsl;

use crate::database::models::*;
use either::Either;
use either::Either::{Left, Right};

pub(crate) fn get_communities(
    conn: &MysqlConnection,
) -> Result<Vec<DatabaseCommunity>, diesel::result::Error> {
    use crate::database::schema::Communities::dsl::*;

    Communities.load::<DatabaseCommunity>(conn)
}

pub(crate) fn get_community_admins(
    conn: &MysqlConnection,
    community: &DatabaseCommunity,
) -> Result<
    Vec<(
        DatabaseUser,
        Either<DatabaseLocalUser, DatabaseFederatedUser>,
    )>,
    diesel::result::Error,
> {
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
    v.append(&mut local_admins.into_iter().map(|l| (l.0, Left(l.1))).collect());
    v.append(
        &mut federated_admins
            .into_iter()
            .map(|l| (l.0, Right(l.1)))
            .collect(),
    );

    Ok(v)
}

pub(crate) fn get_community(
    conn: &MysqlConnection,
    id_: &str,
) -> Result<Option<DatabaseCommunity>, diesel::result::Error> {
    use crate::database::schema::Communities::dsl::*;
    Communities
        .filter(name.eq(id_))
        .first::<DatabaseCommunity>(conn)
        .optional()
}
