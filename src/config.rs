pub const PRESET_HOST_LOCAL: &str = "http://localhost:3000";
pub const PRESET_HOST_DOCKER: &str = "http://localhost:3008";

pub fn get_preset_hosts() -> Vec<String> {
    let mut hosts = Vec::new();
    if let Ok(remote) = std::env::var("DEVBOOK_REMOTE_URL").or_else(|_| std::env::var("DEVBOOK_URL")) {
        let norm = normalize_url(&remote);
        if !norm.is_empty() {
            hosts.push(norm);
        }
    }

    for preset in [PRESET_HOST_LOCAL, PRESET_HOST_DOCKER] {
        let p = preset.to_string();
        if !hosts.contains(&p) {
            hosts.push(p);
        }
    }

    hosts
}

#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub api_token: Option<String>,
    pub allow_host_switcher: bool,
}

pub fn normalize_url(raw: &str) -> String {
    let mut s = raw.trim().to_string();
    if s.is_empty() {
        return PRESET_HOST_LOCAL.to_string();
    }
    if !s.starts_with("http://") && !s.starts_with("https://") {
        s = format!("http://{}", s);
    }
    // Trim trailing slashes
    while s.ends_with('/') {
        s.pop();
    }
    // Strip trailing /api/v1 if provided
    if s.ends_with("/api/v1") {
        s.truncate(s.len() - 7);
    }
    while s.ends_with('/') {
        s.pop();
    }
    s
}

impl Config {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: normalize_url(base_url),
            api_token: std::env::var("DEVBOOK_API_KEY")
                .or_else(|_| std::env::var("DEVBOOK_API_TOKEN"))
                .ok(),
            allow_host_switcher: std::env::var("DEVBOOK_REMOTE_URL").is_err(),
        }
    }

    pub fn from_env_and_args() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let env_remote = std::env::var("DEVBOOK_REMOTE_URL")
            .or_else(|_| std::env::var("DEVBOOK_URL"))
            .ok();
        Self::parse_from_args_and_env(&args, env_remote.as_deref())
    }

    pub fn parse_from_args_and_env(args: &[String], env_remote: Option<&str>) -> Self {
        let mut chosen_url: Option<String> = None;
        let mut has_allow_switch = false;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--url" | "--host" | "-u" => {
                    if i + 1 < args.len() {
                        chosen_url = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                arg if arg.starts_with("--url=") => {
                    chosen_url = Some(arg.trim_start_matches("--url=").to_string());
                }
                arg if arg.starts_with("--host=") => {
                    chosen_url = Some(arg.trim_start_matches("--host=").to_string());
                }
                "--local" => {
                    chosen_url = Some(PRESET_HOST_LOCAL.to_string());
                }
                "--remote" => {
                    if let Some(remote) = env_remote {
                        chosen_url = Some(remote.to_string());
                    }
                }
                "--allow-switch" => {
                    has_allow_switch = true;
                }
                _ => {}
            }
            i += 1;
        }

        let base_url = chosen_url
            .or_else(|| env_remote.map(|s| s.to_string()))
            .unwrap_or_else(|| PRESET_HOST_LOCAL.to_string());

        let allow_host_switcher = env_remote.is_none() || has_allow_switch;

        Self {
            base_url: normalize_url(&base_url),
            api_token: std::env::var("DEVBOOK_API_KEY")
                .or_else(|_| std::env::var("DEVBOOK_API_TOKEN"))
                .ok(),
            allow_host_switcher,
        }
    }

    pub fn set_url(&mut self, new_url: &str) {
        self.base_url = normalize_url(new_url);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_url() {
        assert_eq!(normalize_url("http://localhost:3000/"), "http://localhost:3000");
        assert_eq!(normalize_url("http://remote-server:3008/"), "http://remote-server:3008");
        assert_eq!(normalize_url("localhost:3008"), "http://localhost:3008");
        assert_eq!(normalize_url("http://localhost:3008/api/v1"), "http://localhost:3008");
        assert_eq!(normalize_url("http://localhost:3008/api/v1/"), "http://localhost:3008");
    }

    #[test]
    fn test_default_without_env_or_flags() {
        let args = vec!["devbook-tui".to_string()];
        let cfg = Config::parse_from_args_and_env(&args, None);
        assert_eq!(cfg.base_url, "http://localhost:3000");
        assert!(cfg.allow_host_switcher);
    }

    #[test]
    fn test_default_with_devbook_remote_url() {
        let args = vec!["devbook-tui".to_string()];
        let cfg = Config::parse_from_args_and_env(&args, Some("http://remote-server:3008"));
        assert_eq!(cfg.base_url, "http://remote-server:3008");
        // Locked by default when DEVBOOK_REMOTE_URL is set
        assert!(!cfg.allow_host_switcher);
    }

    #[test]
    fn test_allow_switch_override_with_remote_url() {
        let args = vec!["devbook-tui".to_string(), "--allow-switch".to_string()];
        let cfg = Config::parse_from_args_and_env(&args, Some("http://remote-server:3008"));
        assert_eq!(cfg.base_url, "http://remote-server:3008");
        assert!(cfg.allow_host_switcher);
    }

    #[test]
    fn test_local_flag_overrides_remote_env() {
        let args = vec!["devbook-tui".to_string(), "--local".to_string()];
        let cfg = Config::parse_from_args_and_env(&args, Some("http://remote-server:3008"));
        assert_eq!(cfg.base_url, "http://localhost:3000");
    }

    #[test]
    fn test_custom_url_flag_overrides_remote_env() {
        let args = vec![
            "devbook-tui".to_string(),
            "--url".to_string(),
            "http://custom:8080".to_string(),
        ];
        let cfg = Config::parse_from_args_and_env(&args, Some("http://remote-server:3008"));
        assert_eq!(cfg.base_url, "http://custom:8080");
    }
}
