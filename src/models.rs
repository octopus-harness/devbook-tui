use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Stats {
    pub total_prompts: u64,
    pub standalone_prompts: u64,
    pub pinned_prompts: u64,
    pub total_projects: u64,
    pub total_attachments: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Project {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub prompts_count: u64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectInput {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub limit: u64,
    pub offset: u64,
    pub total_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedProjects {
    pub projects: Vec<Project>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Prompt {
    pub id: u64,
    pub project_id: Option<u64>,
    pub project_name: Option<String>,
    pub title: String,
    pub body: Option<String>,
    #[serde(default)]
    pub pinned: bool,
    pub pinned_position: Option<u64>,
    #[serde(default)]
    pub attachments_count: u64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptDetail {
    pub id: u64,
    pub project_id: Option<u64>,
    pub project_name: Option<String>,
    pub title: String,
    pub body: Option<String>,
    #[serde(default)]
    pub pinned: bool,
    pub pinned_position: Option<u64>,
    #[serde(default)]
    pub attachments_count: u64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub rendered_html: Option<String>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
}

impl From<PromptDetail> for Prompt {
    fn from(detail: PromptDetail) -> Self {
        Self {
            id: detail.id,
            project_id: detail.project_id,
            project_name: detail.project_name,
            title: detail.title,
            body: detail.body,
            pinned: detail.pinned,
            pinned_position: detail.pinned_position,
            attachments_count: detail.attachments_count,
            created_at: detail.created_at,
            updated_at: detail.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromptInput {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Option<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedPrompts {
    pub prompts: Vec<Prompt>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Attachment {
    pub id: u64,
    pub filename: String,
    pub content_type: Option<String>,
    pub byte_size: Option<u64>,
    pub url: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentsResponse {
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TogglePinResponse {
    pub status: String,
    pub pinned: bool,
    pub pinned_position: Option<u64>,
    pub prompt: Prompt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownRenderResponse {
    pub html: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusOkResponse {
    pub status: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub error: String,
    pub details: Option<Vec<String>>,
}
