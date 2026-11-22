#![allow(dead_code)]

use std::path::Path;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use reqwest::blocking::{multipart, Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

use crate::config::Config;
use crate::models::{
    ApiErrorResponse, AttachmentsResponse, MarkdownRenderResponse, PaginatedProjects,
    PaginatedPrompts, Project, ProjectInput, Prompt, PromptDetail, PromptInput, Stats,
    TogglePinResponse,
};

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    pub base_url: String,
    pub api_token: Option<String>,
}

impl ApiClient {
    pub fn new(config: &Config) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            base_url: config.base_url.clone(),
            api_token: config.api_token.clone(),
        }
    }

    pub fn set_base_url(&mut self, url: &str) {
        self.base_url = crate::config::normalize_url(url);
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        if let Some(token) = &self.api_token {
            if let Ok(val) = HeaderValue::from_str(&format!("Bearer {}", token)) {
                headers.insert(AUTHORIZATION, val);
            }
        }
        headers
    }

    fn check_response(&self, res: Response) -> Result<Response> {
        let status = res.status();
        if status.is_success() {
            Ok(res)
        } else {
            let error_text = res.text().unwrap_or_default();
            if let Ok(err_obj) = serde_json::from_str::<ApiErrorResponse>(&error_text) {
                let detail_msg = err_obj
                    .details
                    .map(|d| d.join(", "))
                    .unwrap_or_default();
                Err(anyhow!(
                    "HTTP {}: {} {}",
                    status.as_u16(),
                    err_obj.error,
                    detail_msg
                ))
            } else {
                Err(anyhow!("HTTP {}: {}", status.as_u16(), error_text))
            }
        }
    }

    // Stats
    pub fn get_stats(&self) -> Result<Stats> {
        let url = format!("{}/api/v1/stats", self.base_url);
        let res = self
            .client
            .get(&url)
            .headers(self.headers())
            .send()
            .context("Failed to connect to Devbook server")?;
        let res = self.check_response(res)?;
        Ok(res.json()?)
    }

    // Markdown Rendering
    pub fn render_markdown(&self, text: &str) -> Result<String> {
        let url = format!("{}/api/v1/markdown/render", self.base_url);
        let body = serde_json::json!({ "text": text });
        let res = self
            .client
            .post(&url)
            .headers(self.headers())
            .json(&body)
            .send()
            .context("Failed to connect to Devbook server")?;
        let res = self.check_response(res)?;
        let parsed: MarkdownRenderResponse = res.json()?;
        Ok(parsed.html)
    }

    // Projects
    pub fn list_projects(&self, q: Option<&str>) -> Result<Vec<Project>> {
        let mut url = format!("{}/api/v1/projects?limit=100", self.base_url);
        if let Some(query) = q.filter(|s| !s.trim().is_empty()) {
            url.push_str("&q=");
            url.push_str(&urlencoding(query));
        }
        let res = self
            .client
            .get(&url)
            .headers(self.headers())
            .send()
            .context("Failed to connect to Devbook server")?;
        let res = self.check_response(res)?;
        let paginated: PaginatedProjects = res.json()?;
        Ok(paginated.projects)
    }

    pub fn create_project(&self, input: &ProjectInput) -> Result<Project> {
        let url = format!("{}/api/v1/projects", self.base_url);
        let body = serde_json::json!({ "project": input });
        let res = self
            .client
            .post(&url)
            .headers(self.headers())
            .json(&body)
            .send()?;
        let res = self.check_response(res)?;
        Ok(res.json()?)
    }

    pub fn update_project(&self, id: u64, input: &ProjectInput) -> Result<Project> {
        let url = format!("{}/api/v1/projects/{}", self.base_url, id);
        let body = serde_json::json!({ "project": input });
        let res = self
            .client
            .patch(&url)
            .headers(self.headers())
            .json(&body)
            .send()?;
        let res = self.check_response(res)?;
        Ok(res.json()?)
    }

    pub fn delete_project(&self, id: u64) -> Result<()> {
        let url = format!("{}/api/v1/projects/{}", self.base_url, id);
        let res = self.client.delete(&url).headers(self.headers()).send()?;
        self.check_response(res)?;
        Ok(())
    }

    // Prompts
    pub fn list_prompts(
        &self,
        project_id: Option<u64>,
        scope: Option<&str>,
        pinned: Option<bool>,
        search: Option<&str>,
    ) -> Result<Vec<Prompt>> {
        let mut url = format!("{}/api/v1/prompts?limit=100", self.base_url);
        if let Some(pid) = project_id {
            url.push_str(&format!("&project_id={}", pid));
        } else if let Some(sc) = scope {
            url.push_str(&format!("&scope={}", sc));
        }
        if let Some(pin) = pinned {
            url.push_str(&format!("&pinned={}", pin));
        }
        if let Some(q) = search.filter(|s| !s.trim().is_empty()) {
            url.push_str("&q=");
            url.push_str(&urlencoding(q));
        }

        let res = self
            .client
            .get(&url)
            .headers(self.headers())
            .send()
            .context("Failed to connect to Devbook server")?;
        let res = self.check_response(res)?;
        let paginated: PaginatedPrompts = res.json()?;
        Ok(paginated.prompts)
    }

    pub fn get_prompt(&self, id: u64) -> Result<PromptDetail> {
        let url = format!("{}/api/v1/prompts/{}?render_markdown=true", self.base_url, id);
        let res = self
            .client
            .get(&url)
            .headers(self.headers())
            .send()
            .context("Failed to connect to Devbook server")?;
        let res = self.check_response(res)?;
        Ok(res.json()?)
    }

    pub fn create_prompt(&self, input: &PromptInput) -> Result<PromptDetail> {
        let url = format!("{}/api/v1/prompts", self.base_url);
        let body = serde_json::json!({ "prompt": input });
        let res = self
            .client
            .post(&url)
            .headers(self.headers())
            .json(&body)
            .send()?;
        let res = self.check_response(res)?;
        Ok(res.json()?)
    }

    pub fn update_prompt(&self, id: u64, input: &PromptInput) -> Result<PromptDetail> {
        let url = format!("{}/api/v1/prompts/{}", self.base_url, id);
        let body = serde_json::json!({ "prompt": input });
        let res = self
            .client
            .patch(&url)
            .headers(self.headers())
            .json(&body)
            .send()?;
        let res = self.check_response(res)?;
        Ok(res.json()?)
    }

    pub fn delete_prompt(&self, id: u64) -> Result<()> {
        let url = format!("{}/api/v1/prompts/{}", self.base_url, id);
        let res = self.client.delete(&url).headers(self.headers()).send()?;
        self.check_response(res)?;
        Ok(())
    }

    pub fn toggle_pin(&self, id: u64) -> Result<TogglePinResponse> {
        let url = format!("{}/api/v1/prompts/{}/toggle_pin", self.base_url, id);
        let res = self.client.patch(&url).headers(self.headers()).send()?;
        let res = self.check_response(res)?;
        Ok(res.json()?)
    }

    pub fn reorder_pinned(&self, ids: &[u64]) -> Result<()> {
        let url = format!("{}/api/v1/prompts/reorder_pinned", self.base_url);
        let body = serde_json::json!({ "prompt_ids": ids });
        let res = self
            .client
            .patch(&url)
            .headers(self.headers())
            .json(&body)
            .send()?;
        self.check_response(res)?;
        Ok(())
    }

    // Attachments
    pub fn list_attachments(&self, prompt_id: u64) -> Result<Vec<crate::models::Attachment>> {
        let url = format!("{}/api/v1/prompts/{}/attachments", self.base_url, prompt_id);
        let res = self.client.get(&url).headers(self.headers()).send()?;
        let res = self.check_response(res)?;
        let resp: AttachmentsResponse = res.json()?;
        Ok(resp.attachments)
    }

    pub fn upload_attachment(&self, prompt_id: u64, file_path: &Path) -> Result<()> {
        let url = format!("{}/api/v1/prompts/{}/attachments", self.base_url, prompt_id);
        let form = multipart::Form::new().file("files[]", file_path)?;

        let mut req = self.client.post(&url);
        if let Some(token) = &self.api_token {
            req = req.header(AUTHORIZATION, format!("Bearer {}", token));
        }

        let res = req.multipart(form).send()?;
        self.check_response(res)?;
        Ok(())
    }

    pub fn delete_attachment(&self, prompt_id: u64, attachment_id: u64) -> Result<()> {
        let url = format!(
            "{}/api/v1/prompts/{}/attachments/{}",
            self.base_url, prompt_id, attachment_id
        );
        let res = self.client.delete(&url).headers(self.headers()).send()?;
        self.check_response(res)?;
        Ok(())
    }

    pub fn download_attachment(&self, raw_url: &str, dest_path: &Path) -> Result<()> {
        let full_url = if raw_url.starts_with("http://") || raw_url.starts_with("https://") {
            raw_url.to_string()
        } else {
            format!("{}{}", self.base_url, raw_url)
        };

        let mut req = self.client.get(&full_url);
        if let Some(token) = &self.api_token {
            req = req.header(AUTHORIZATION, format!("Bearer {}", token));
        }

        let res = req.send()?;
        let mut res = self.check_response(res)?;
        let mut file = std::fs::File::create(dest_path)?;
        res.copy_to(&mut file)?;
        Ok(())
    }
}

fn urlencoding(input: &str) -> String {
    let mut out = String::new();
    for b in input.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            b' ' => out.push('+'),
            _ => {
                out.push_str(&format!("%{:02X}", b));
            }
        }
    }
    out
}
