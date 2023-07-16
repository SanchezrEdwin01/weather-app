// @generated automatically by Diesel CLI.

diesel::table! {
    weather (id) {
        id -> Int4,
        #[max_length = 255]
        city -> Varchar,
        temperature -> Numeric,
        #[max_length = 255]
        description -> Varchar,
        humidity -> Numeric,
        #[max_length = 255]
        wind -> Nullable<Varchar>,
        visibility -> Nullable<Numeric>,
        atmospheric_pressure -> Nullable<Numeric>,
        sunrise -> Nullable<Time>,
        sunset -> Nullable<Time>,
        date_time -> Timestamp,
    }
}
