use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FieldType {
    ShortText,
    LongText,
    Number,
    Boolean,
    Date,
    Image,
    Relation(String), // Name of the related ContentType
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FieldDefinition {
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub help_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContentType {
    pub id: Option<String>,
    pub name: String,
    pub slug: String,
    pub fields: Vec<FieldDefinition>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Entry {
    pub id: Option<String>,
    pub content_type_slug: String,
    pub fields: HashMap<String, serde_json::Value>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserRole {
    Admin,
    Poster,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: Option<String>,
    pub username: String,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CMSSchema {
    pub content_type: ContentType,
}
