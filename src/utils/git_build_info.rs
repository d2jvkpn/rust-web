// println!("~~~ {:?}", option_env!("GIT_BRNACH"));
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct GitBuildInfo {
    build_time: Option<String>,
    git_branch: Option<String>,
    git_commit: Option<String>,
    git_time: Option<String>,
    git_tree_state: Option<String>,
}

#[allow(dead_code)]
fn unknown() -> Option<String> {
    Some("unknown".into())
}

//
pub static GIT_BUILD_INFO: Lazy<GitBuildInfo> = Lazy::new(|| GitBuildInfo {
    build_time: option_env!("BUILD_Time").map(|v| v.to_string()),
    git_branch: option_env!("GIT_Branch").map(|v| v.to_string()),
    git_commit: option_env!("GIT_Commit").map(|v| v.to_string()),
    git_time: option_env!("GIT_Time").map(|v| v.to_string()),
    git_tree_state: option_env!("GIT_TreeState").map(|v| v.to_string()),
});

/* GIT_BUILD_INFO.lock().unwrap_or(GitBuildInfo::default()) */

//
static OC_GBI: OnceCell<GitBuildInfo> = OnceCell::new();

impl GitBuildInfo {
    pub fn set(yaml: &str) -> Result<(), String> {
        let data: Self = serde_yaml::from_str(yaml).map_err(|e| format!("{:}", e))?;

        // let data = Self {
        //     build_time: option_env!("BUILD_Time").map(|v| v.to_string()),
        //    git_branch: option_env!("GIT_Branch").map(|v| v.to_string()),
        //    git_commit: option_env!("GIT_Commit").map(|v| v.to_string()),
        //    git_time: option_env!("GIT_Time").map(|v| v.to_string()),
        //    git_tree_state: option_env!("GIT_TreeState").map(|v| v.to_string()),
        // };

        OC_GBI.set(data).map_err(|_| "can't set git build info".to_string())
    }

    pub fn get() -> Option<&'static GitBuildInfo> {
        OC_GBI.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_git_build_info() {
        println!("~~~ {:?}", GIT_BUILD_INFO.clone());

        GitBuildInfo::set().unwrap();
        println!("~~~ {:?}", GitBuildInfo::get().unwrap());
    }
}
