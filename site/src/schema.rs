table! {
    users (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
        pass_strength -> Varchar,
        crack_time -> Varchar,
        token -> Varchar,
        activated -> Bool,
        super_user -> Bool,
    }
}