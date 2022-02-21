use nanoserde::{DeJson, SerJson};

#[derive(SerJson, DeJson, Clone)]
pub struct Attribute {
    #[nserde(rename = "attributeId")]
    pub attribute_id: Option<String>,

    #[nserde(rename = "noteId")]
    pub note_id: String,

    #[nserde(rename = "type")]
    pub attribute_type: Option<AttributeType>,

    #[nserde(rename = "name")]
    pub name: String,

    #[nserde(rename = "value")]
    pub value: String,

    #[nserde(rename = "position")]
    pub position: usize,

    #[nserde(rename = "isInheritable")]
    pub is_inheritable: bool,
    // #[nserde(rename = "utcDateModified")]
    // utc_date_time: UtcDateTime,
}

#[derive(SerJson, DeJson, Clone)]
pub enum AttributeType {
    #[nserde(rename = "label")]
    Label,

    #[nserde(rename = "relation")]
    Relation,
}
