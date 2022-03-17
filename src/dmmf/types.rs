use serde::Deserialize;

#[derive(Deserialize)]
pub struct Datamodel {
    pub models: Vec<Model>,
    pub enums: Vec<DatamodelEnum>,
    pub types: Vec<Model>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub name: String,
    pub db_name: Option<String>,
    pub fields: Vec<Field>,
    pub field_map: Option<serde_json::Value>,
    pub unique_fields: Option<Vec<Vec<String>>>,
    pub unique_indexes: Option<Vec<Uniqueindex>>,
    pub documentation: Option<String>,
    pub primary_key: Option<PrimaryKey>,
    // There can be more
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatamodelEnum {
    pub name: String,
    pub values: Vec<EnumValue>,
    pub db_name: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub name: String,
    pub db_name: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub kind: FieldKind,
    pub name: String,
    pub is_required: bool,
    pub is_list: bool,
    pub is_unique: bool,
    pub is_id: bool,
    pub is_read_only: bool,
    pub is_generated: Option<bool>,
    pub is_updated_at: Option<bool>,
    pub r#type: String,
    pub db_names: Option<Vec<String>>,
    pub has_default_value: bool,
    pub default: Option<serde_json::Value>,
    pub relation_from_fields: Option<Vec<String>>,
    pub relation_to_fields: Option<serde_json::Value>,
    pub relation_on_delete: Option<String>,
    pub relation_name: Option<String>,
    pub documentation: Option<String>,
    // There can be more
}

#[derive(Deserialize)]
pub struct Uniqueindex {
    pub name: String,
    pub fields: Vec<String>,
}

#[derive(Deserialize)]
pub struct PrimaryKey {
    pub name: Option<String>,
    pub fields: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldKind {
    Scalar,
    Object,
    Enum,
    Unsupported,
}
