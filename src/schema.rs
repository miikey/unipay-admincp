table! {
    dkp_applications (aid) {
        aid -> Integer,
        app_id -> Varchar,
        app_name -> Varchar,
    }
}

// table! {
//     dkp_merchant_channel (acid) {
//         acid -> Integer,
//         app_id -> Nullable<Varchar>,
//         merchant_id -> Nullable<Varchar>,
//         channel_type -> Nullable<Varchar>,
//         channel_rate -> Nullable<Double>,
//         mer_auth_token -> Nullable<Varchar>,
//         create_time -> Nullable<Datetime>,
//         is_facepay -> Nullable<Integer>,
//         remark -> Nullable<Varchar>,
//         status -> Nullable<Integer>,
//     }
// }

allow_tables_to_appear_in_same_query!(
    dkp_applications,
);
