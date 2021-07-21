table! {
    module_category (uuid) {
        uuid -> Varchar,
        page_uuid -> Varchar,
        title -> Varchar,
    }
}

table! {
    modules (uuid) {
        uuid -> Varchar,
        page_uuid -> Varchar,
        category_uuid -> Nullable<Varchar>,
        title -> Varchar,
        content -> Text,
    }
}

table! {
    pages (uuid) {
        uuid -> Varchar,
        page_name -> Varchar,
        page_url -> Varchar,
        page_title -> Varchar,
        time_created -> Timestamp,
    }
}

joinable!(module_category -> pages (page_uuid));
joinable!(modules -> module_category (category_uuid));
joinable!(modules -> pages (page_uuid));

allow_tables_to_appear_in_same_query!(
    module_category,
    modules,
    pages,
);
