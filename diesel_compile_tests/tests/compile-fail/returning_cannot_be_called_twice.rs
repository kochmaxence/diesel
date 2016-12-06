#[macro_use] extern crate diesel;

use diesel::*;
use diesel::pg::PgConnection;

table! {
    users {
        id -> Integer,
        name -> VarChar,
    }
}

pub struct NewUser(String);

impl_Insertable! {
    (users)
    pub struct NewUser(#[column_name(name)] String,);
}

fn main() {
    use self::users::dsl::*;

    let connection = PgConnection::establish("").unwrap();

    let query = delete(users.filter(name.eq("Bill")))
        .returning(id);
    query.returning(name);
    //~^ ERROR: no method named `returning`

    let query = insert(&NewUser("Hello".into()))
        .into(users)
        .returning(id);
    query.returning(name);
    //~^ ERROR: no method named `returning`

    let query = update(users)
        .set(name.eq("Bill"))
        .returning(id);
    query.returning(name);
    //~^ ERROR: no method named `returning`
}
