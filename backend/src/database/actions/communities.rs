use diesel::prelude::*;
use diesel::BelongingToDsl;

use crate::database::models::{DatabaseCommunity, DatabaseLocalUser, DatabaseUser, DatabaseFederatedUser, DatabaseCommunitiesUser};
use crate::federation::schemas::Community;
use crate::database::schema::Users::dsl::Users;
use crate::database::schema::LocalUsers::dsl::LocalUsers;
use either::Either;
use crate::database::schema::CommunitiesUsers::dsl::CommunitiesUsers;
use either::Either::{Left, Right};

pub(crate) fn get_communities(
    conn: &MysqlConnection,
) -> Result<Vec<DatabaseCommunity>, diesel::result::Error> {
    use crate::database::schema::Communities::dsl::*;

    Communities.load::<DatabaseCommunity>(conn)
}

pub(crate) fn get_community_by_id(
    conn: &MysqlConnection,
    id_: &str
) -> Result<(DatabaseCommunity, Vec<(DatabaseUser, Either<DatabaseLocalUser, DatabaseFederatedUser>)>), diesel::result::Error> {
    let community = {
        use crate::database::schema::Communities::dsl::*;
        Communities
            .filter(name.eq(id_))
            .first::<DatabaseCommunity>(conn)
    }?;

    let local_admins: Vec<(DatabaseUser, DatabaseLocalUser)> = {
        use crate::database::schema::CommunitiesUsers::dsl::*;
        use crate::database::schema::Users::dsl::*;
        use crate::database::schema::LocalUsers::dsl::*;
        use crate::database::models::DatabaseCommunitiesUser;

        DatabaseCommunitiesUser::belonging_to(&community)
            .inner_join(
                Users.inner_join(LocalUsers)
            )
            .select((
                Users::all_columns(),
                LocalUsers::all_columns()
            ))
            .load(conn)
    }?;

    let federated_admins: Vec<(DatabaseUser, DatabaseFederatedUser)> = {
        use crate::database::schema::CommunitiesUsers::dsl::*;
        use crate::database::schema::Users::dsl::*;
        use crate::database::schema::FederatedUsers::dsl::*;
        use crate::database::models::DatabaseCommunitiesUser;

        DatabaseCommunitiesUser::belonging_to(&community)
            .inner_join(
                Users.inner_join(FederatedUsers)
            )
            .select((
                Users::all_columns(),
                FederatedUsers::all_columns()
            ))
            .load(conn)
    }?;

    let mut v = vec![];
    v.append(&mut
        local_admins
            .into_iter()
            .map(|l| (l.0, Left(l.1)))
            .collect()
    );
    v.append(&mut
        federated_admins
            .into_iter()
            .map(|l| (l.0, Right(l.1)))
            .collect()
    );

    Ok((community, v))

}
