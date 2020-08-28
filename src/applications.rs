use crate::schema::dkp_applications;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Applications {
    pub aid: i32,
    pub app_id: String,
    pub app_name: String
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="dkp_applications"]
pub struct NewApplications {
    pub app_id: String,
    pub app_name: String
}