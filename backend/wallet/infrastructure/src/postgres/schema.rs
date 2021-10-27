table! {
    wallets (id) {
        id -> Varchar,
        owner -> Varchar,
        balance -> Text,
        create_at -> Timestamptz,
        update_at -> Timestamptz,
        delete_at -> Nullable<Timestamptz>,
    }
}
