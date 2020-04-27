table! {
    users (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
        token -> Varchar,
        activated -> Bool,
    }
}