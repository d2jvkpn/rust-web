// println!("~~~ {:?}", option_env!("GIT_BRNACH"));
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
// use std::sync::Mutex;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitBuildInfo {
    build_time: Option<String>,
    git_branch: Option<String>,
    git_commit: Option<String>,
    git_time: Option<String>,
    git_tree_state: Option<String>,
}

//
pub static GIT_BUILD_INFO: Lazy<GitBuildInfo> = Lazy::new(|| {
    let data = GitBuildInfo {
        build_time: option_env!("BUILD_Time").map(|v| v.to_string()),
        git_branch: option_env!("GIT_Branch").map(|v| v.to_string()),
        git_commit: option_env!("GIT_Commit").map(|v| v.to_string()),
        git_time: option_env!("GIT_Time").map(|v| v.to_string()),
        git_tree_state: option_env!("GIT_TreeState").map(|v| v.to_string()),
    };

    data
});

/* GIT_BUILD_INFO.lock().unwrap_or(GitBuildInfo::default()) */

//
static OC_GBI: OnceCell<GitBuildInfo> = OnceCell::new();

impl GitBuildInfo {
    pub fn set() -> Result<(), &'static str> {
        let build_time = option_env!("BUILD_Time").map(|v| v.to_string());
        let git_branch = option_env!("GIT_Branch").map(|v| v.to_string());
        let git_commit = option_env!("GIT_Commit").map(|v| v.to_string());
        let git_time = option_env!("GIT_Time").map(|v| v.to_string());
        let git_tree_state = option_env!("GIT_TreeState").map(|v| v.to_string());

        OC_GBI
            .set(Self { build_time, git_branch, git_commit, git_time, git_tree_state })
            .map_err(|_| "can't set GitBuildInfo")
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
