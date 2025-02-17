use super::*;
use crate::types::{
    Config, ConfigSection, CrateInfo, EnvInfo, FlowInfo, RustVersionCondition, Step, Task,
    TaskCondition,
};
use ci_info;
use git_info::types::GitInfo;
use indexmap::IndexMap;
use rust_info::types::{RustChannel, RustInfo};

#[test]
fn validate_env_set_empty() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_set(&condition);

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_env_set_valid() {
    envmnt::set("ENV_SET1", "");
    envmnt::set("ENV_SET2", "value");

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: Some(vec!["ENV_SET1".to_string(), "ENV_SET2".to_string()]),
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_set(&condition);

    assert!(enabled);
}

#[test]
fn validate_env_set_invalid() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: Some(vec!["BAD_ENV_SET1".to_string(), "BAD_ENV_SET2".to_string()]),
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_set(&condition);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_env_set_invalid_partial_found() {
    envmnt::set("ENV_SET1", "");
    envmnt::set("ENV_SET2", "value");

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: Some(vec![
            "ENV_SET1".to_string(),
            "ENV_SET2".to_string(),
            "BAD_ENV_SET1".to_string(),
        ]),
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_set(&condition);

    assert!(!enabled);
}

#[test]
fn validate_env_not_set_empty() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_not_set(&condition);

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_env_not_set_valid() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: Some(vec!["BAD_ENV_SET1".to_string(), "BAD_ENV_SET2".to_string()]),
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_not_set(&condition);

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_env_not_set_invalid() {
    envmnt::set("ENV_SET1", "");
    envmnt::set("ENV_SET2", "value");

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: Some(vec!["ENV_SET1".to_string(), "ENV_SET2".to_string()]),
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_not_set(&condition);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_env_not_set_invalid_partial_found() {
    envmnt::set("ENV_SET1", "");
    envmnt::set("ENV_SET2", "value");

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: Some(vec![
            "ENV_SET1".to_string(),
            "ENV_SET2".to_string(),
            "BAD_ENV_SET1".to_string(),
        ]),
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_not_set(&condition);

    assert!(!enabled);
}

#[test]
fn validate_env_bool_true_empty() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_bool(&condition, true);

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_env_bool_true_valid() {
    envmnt::set_bool("ENV_TRUE1", true);
    envmnt::set_bool("ENV_TRUE2", true);

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: Some(vec!["ENV_TRUE1".to_string(), "ENV_TRUE2".to_string()]),
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_bool(&condition, true);

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_env_bool_true_invalid() {
    envmnt::set_bool("ENV_TRUE1", false);
    envmnt::set_bool("ENV_TRUE2", false);

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: Some(vec!["ENV_TRUE1".to_string(), "ENV_TRUE2".to_string()]),
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_bool(&condition, true);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_env_bool_true_invalid_partial_found() {
    envmnt::remove("ENV_TRUE1");
    envmnt::set_bool("ENV_TRUE2", true);

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: Some(vec!["ENV_TRUE1".to_string(), "ENV_TRUE2".to_string()]),
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_bool(&condition, true);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_env_bool_true_invalid_partial_valid() {
    envmnt::set_bool("ENV_TRUE2", true);
    envmnt::set_bool("ENV_TRUE2", false);

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: Some(vec!["ENV_TRUE1".to_string(), "ENV_TRUE2".to_string()]),
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_bool(&condition, true);

    assert!(!enabled);
}

#[test]
fn validate_env_bool_false_empty() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_bool(&condition, false);

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_env_bool_false_valid() {
    envmnt::set_bool("ENV_FALSE1", false);
    envmnt::set_bool("ENV_FALSE2", false);

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: Some(vec!["ENV_FALSE1".to_string(), "ENV_FALSE2".to_string()]),
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_bool(&condition, false);

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_env_bool_false_invalid() {
    envmnt::set_bool("ENV_FALSE1", true);
    envmnt::set_bool("ENV_FALSE2", true);

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: Some(vec!["ENV_FALSE1".to_string(), "ENV_FALSE2".to_string()]),
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_bool(&condition, false);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_env_bool_false_invalid_partial_found() {
    envmnt::remove("ENV_FALSE1");
    envmnt::set_bool("ENV_FALSE2", false);

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: Some(vec!["ENV_FALSE1".to_string(), "ENV_FALSE2".to_string()]),
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_bool(&condition, false);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_env_bool_false_invalid_partial_valid() {
    envmnt::set_bool("ENV_FALSE2", false);
    envmnt::set_bool("ENV_FALSE2", true);

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: Some(vec!["ENV_FALSE1".to_string(), "ENV_FALSE2".to_string()]),
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_bool(&condition, false);

    assert!(!enabled);
}

#[test]
fn validate_env_empty() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env(&condition);

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_env_valid() {
    envmnt::set("ENV_SET1", "");
    envmnt::set("ENV_SET2", "value");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "".to_string());
    env_values.insert("ENV_SET2".to_string(), "value".to_string());

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: Some(env_values),
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env(&condition);

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_env_invalid_not_found() {
    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("BAD_ENV_SET1".to_string(), "".to_string());
    env_values.insert("BAD_ENV_SET2".to_string(), "value".to_string());

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: Some(env_values),
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env(&condition);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_env_invalid_not_equal() {
    envmnt::set("ENV_SET2", "value");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET2".to_string(), "value2".to_string());

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: Some(env_values),
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env(&condition);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_env_invalid_partial_found() {
    envmnt::set("ENV_SET1", "good");
    envmnt::set("ENV_SET2", "good");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "good".to_string());
    env_values.insert("ENV_SET2".to_string(), "bad".to_string());

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: Some(env_values),
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env(&condition);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_env_contains_valid_same() {
    envmnt::set("ENV_SET1", "");
    envmnt::set("ENV_SET2", "value");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "".to_string());
    env_values.insert("ENV_SET2".to_string(), "value".to_string());

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: Some(env_values),
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_contains(&condition);

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_env_contains_valid() {
    envmnt::set("ENV_SET1", "");
    envmnt::set("ENV_SET2", "value");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "".to_string());
    env_values.insert("ENV_SET2".to_string(), "val".to_string());

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: Some(env_values),
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_contains(&condition);

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_env_contains_invalid_not_found() {
    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("BAD_ENV_SET1".to_string(), "".to_string());
    env_values.insert("BAD_ENV_SET2".to_string(), "value".to_string());

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: Some(env_values),
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_contains(&condition);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_env_contains_invalid_not_equal() {
    envmnt::set("ENV_SET2", "value");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET2".to_string(), "value2".to_string());

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: Some(env_values),
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_contains(&condition);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_env_contains_invalid_partial_found() {
    envmnt::set("ENV_SET1", "good");
    envmnt::set("ENV_SET2", "good");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "good".to_string());
    env_values.insert("ENV_SET2".to_string(), "bad".to_string());

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: Some(env_values),
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_env_contains(&condition);

    assert!(!enabled);
}

#[test]
fn validate_script_empty() {
    let enabled = validate_script(&None, None);

    assert!(enabled);
}

#[test]
fn validate_script_valid() {
    let enabled = validate_script(&Some(vec!["exit 0".to_string()]), None);

    assert!(enabled);
}

#[test]
fn validate_script_invalid() {
    let enabled = validate_script(&Some(vec!["exit 1".to_string()]), None);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_profile_valid() {
    profile::set("my_profile");

    let condition = TaskCondition {
        fail_message: None,
        profiles: Some(vec![
            "bad1".to_string(),
            "my_profile".to_string(),
            "bad2".to_string(),
        ]),
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_profile(&condition);

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_profile_invalid() {
    profile::set("my_profile");

    let condition = TaskCondition {
        fail_message: None,
        profiles: Some(vec!["bad1".to_string(), "bad2".to_string()]),
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_profile(&condition);

    assert!(!enabled);
}

#[test]
fn validate_platform_valid() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: Some(vec![
            "bad1".to_string(),
            types::get_platform_name(),
            "bad2".to_string(),
        ]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_platform(&condition);

    assert!(enabled);
}

#[test]
fn validate_platform_invalid() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: Some(vec!["bad1".to_string(), "bad2".to_string()]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_platform(&condition);

    assert!(!enabled);
}

#[test]
fn validate_channel_valid() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let mut flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    flow_info.env_info.rust_info.channel = Some(RustChannel::Stable);
    let mut condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: Some(vec![
            "bad1".to_string(),
            "stable".to_string(),
            "bad2".to_string(),
        ]),
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };
    let mut enabled = validate_channel(&condition, Some(&flow_info));
    assert!(enabled);

    flow_info.env_info.rust_info.channel = Some(RustChannel::Beta);
    condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: Some(vec![
            "bad1".to_string(),
            "beta".to_string(),
            "bad2".to_string(),
        ]),
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };
    enabled = validate_channel(&condition, Some(&flow_info));

    assert!(enabled);

    flow_info.env_info.rust_info.channel = Some(RustChannel::Nightly);
    condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: Some(vec![
            "bad1".to_string(),
            "nightly".to_string(),
            "bad2".to_string(),
        ]),
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };
    enabled = validate_channel(&condition, Some(&flow_info));

    assert!(enabled);
}

#[test]
fn validate_channel_invalid() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let mut flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    flow_info.env_info.rust_info.channel = Some(RustChannel::Stable);
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: Some(vec!["bad1".to_string(), "bad2".to_string()]),
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };
    let enabled = validate_channel(&condition, Some(&flow_info));

    assert!(!enabled);
}

#[test]
fn validate_file_exists_valid() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: Some(vec![
            "${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml".to_string()
        ]),
        files_not_exist: None,
    };

    let enabled = validate_files_exist(&condition);

    assert!(enabled);
}

#[test]
fn validate_file_exists_partial_invalid() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: Some(vec![
            "${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml".to_string(),
            "${CARGO_MAKE_WORKING_DIRECTORY}/Cargo2.toml".to_string(),
        ]),
        files_not_exist: None,
    };

    let enabled = validate_files_exist(&condition);

    assert!(!enabled);
}

#[test]
fn validate_file_exists_invalid() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: Some(vec![
            "${CARGO_MAKE_WORKING_DIRECTORY}/Cargo2.toml".to_string()
        ]),
        files_not_exist: None,
    };

    let enabled = validate_files_exist(&condition);

    assert!(!enabled);
}

#[test]
fn validate_file_not_exists_valid() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: Some(vec![
            "${CARGO_MAKE_WORKING_DIRECTORY}/Cargo2.toml".to_string()
        ]),
    };

    let enabled = validate_files_not_exist(&condition);

    assert!(enabled);
}

#[test]
fn validate_file_not_exists_partial_invalid() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: Some(vec![
            "${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml".to_string(),
            "${CARGO_MAKE_WORKING_DIRECTORY}/Cargo2.toml".to_string(),
        ]),
    };

    let enabled = validate_files_not_exist(&condition);

    assert!(!enabled);
}

#[test]
fn validate_file_not_exists_invalid() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: Some(vec![
            "${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml".to_string()
        ]),
    };

    let enabled = validate_files_not_exist(&condition);

    assert!(!enabled);
}

#[test]
fn validate_criteria_empty() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let enabled = validate_criteria(
        Some(&flow_info),
        &Some(TaskCondition {
            fail_message: None,
            profiles: None,
            platforms: None,
            channels: None,
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
    );

    assert!(enabled);
}

#[test]
fn validate_criteria_valid_platform() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let enabled = validate_criteria(
        Some(&flow_info),
        &Some(TaskCondition {
            fail_message: None,
            profiles: None,
            platforms: Some(vec![
                "bad1".to_string(),
                types::get_platform_name(),
                "bad2".to_string(),
            ]),
            channels: None,
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
    );

    assert!(enabled);
}

#[test]
fn validate_criteria_invalid_platform() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let enabled = validate_criteria(
        Some(&flow_info),
        &Some(TaskCondition {
            fail_message: None,
            profiles: None,
            platforms: Some(vec!["bad1".to_string(), "bad2".to_string()]),
            channels: None,
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
    );

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_criteria_valid_profile() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let enabled = validate_criteria(
        Some(&flow_info),
        &Some(TaskCondition {
            fail_message: None,
            profiles: Some(vec!["bad1".to_string(), profile::get(), "bad2".to_string()]),
            platforms: None,
            channels: None,
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
    );

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_criteria_invalid_profile() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let enabled = validate_criteria(
        Some(&flow_info),
        &Some(TaskCondition {
            fail_message: None,
            profiles: Some(vec!["bad1".to_string(), "bad2".to_string()]),
            platforms: None,
            channels: None,
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
    );

    assert!(!enabled);
}

#[test]
fn validate_criteria_valid_channel() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let mut flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    flow_info.env_info.rust_info.channel = Some(RustChannel::Stable);
    let mut enabled = validate_criteria(
        Some(&flow_info),
        &Some(TaskCondition {
            fail_message: None,
            profiles: None,
            platforms: None,
            channels: Some(vec![
                "bad1".to_string(),
                "stable".to_string(),
                "bad2".to_string(),
            ]),
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
    );

    assert!(enabled);

    flow_info.env_info.rust_info.channel = Some(RustChannel::Beta);
    enabled = validate_criteria(
        Some(&flow_info),
        &Some(TaskCondition {
            fail_message: None,
            profiles: None,
            platforms: None,
            channels: Some(vec![
                "bad1".to_string(),
                "beta".to_string(),
                "bad2".to_string(),
            ]),
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
    );

    assert!(enabled);

    flow_info.env_info.rust_info.channel = Some(RustChannel::Nightly);
    enabled = validate_criteria(
        Some(&flow_info),
        &Some(TaskCondition {
            fail_message: None,
            profiles: None,
            platforms: None,
            channels: Some(vec![
                "bad1".to_string(),
                "nightly".to_string(),
                "bad2".to_string(),
            ]),
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
    );

    assert!(enabled);
}

#[test]
fn validate_criteria_invalid_channel() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let mut flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    flow_info.env_info.rust_info.channel = Some(RustChannel::Stable);
    let enabled = validate_criteria(
        Some(&flow_info),
        &Some(TaskCondition {
            fail_message: None,
            profiles: None,
            platforms: None,
            channels: Some(vec!["bad1".to_string(), "bad2".to_string()]),
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
    );

    assert!(!enabled);
}

#[test]
fn validate_criteria_valid_file_exists() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let enabled = validate_criteria(
        Some(&flow_info),
        &Some(TaskCondition {
            fail_message: None,
            profiles: None,
            platforms: None,
            channels: None,
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: Some(vec![
                "${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml".to_string()
            ]),
            files_not_exist: None,
        }),
    );

    assert!(enabled);
}

#[test]
fn validate_criteria_invalid_file_exists() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let enabled = validate_criteria(
        Some(&flow_info),
        &Some(TaskCondition {
            fail_message: None,
            profiles: None,
            platforms: None,
            channels: None,
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: Some(vec![
                "${CARGO_MAKE_WORKING_DIRECTORY}/Cargo2.toml".to_string()
            ]),
            files_not_exist: None,
        }),
    );

    assert!(!enabled);
}

#[test]
fn validate_criteria_valid_file_not_exists() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let enabled = validate_criteria(
        Some(&flow_info),
        &Some(TaskCondition {
            fail_message: None,
            profiles: None,
            platforms: None,
            channels: None,
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: Some(vec![
                "${CARGO_MAKE_WORKING_DIRECTORY}/Cargo2.toml".to_string()
            ]),
        }),
    );

    assert!(enabled);
}

#[test]
fn validate_criteria_invalid_file_not_exists() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let enabled = validate_criteria(
        Some(&flow_info),
        &Some(TaskCondition {
            fail_message: None,
            profiles: None,
            platforms: None,
            channels: None,
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: Some(vec![
                "${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml".to_string()
            ]),
        }),
    );

    assert!(!enabled);
}

#[test]
fn validate_condition_for_step_both_valid() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    step.config.condition = Some(TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: Some(vec![
            "bad1".to_string(),
            types::get_platform_name(),
            "bad2".to_string(),
        ]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition_for_step(&flow_info, &step);

    assert!(enabled);
}

#[test]
fn validate_condition_for_step_valid_script_invalid() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    step.config.condition = Some(TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: Some(vec![
            "bad1".to_string(),
            types::get_platform_name(),
            "bad2".to_string(),
        ]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    });
    step.config.condition_script = Some(vec!["exit 1".to_string()]);

    let enabled = validate_condition_for_step(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_condition_for_step_invalid_script_valid() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    step.config.condition = Some(TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: Some(vec!["bad1".to_string(), "bad2".to_string()]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition_for_step(&flow_info, &step);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_condition_for_step_invalid_env_set() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    step.config.condition = Some(TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: Some(vec!["BAD_ENV_SET1".to_string()]),
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition_for_step(&flow_info, &step);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_condition_for_step_invalid_env_not_set() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    envmnt::set("ENV_SET1", "bad");

    step.config.condition = Some(TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: Some(vec!["ENV_SET1".to_string()]),
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition_for_step(&flow_info, &step);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_condition_for_step_valid_env() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    envmnt::set("ENV_SET1", "good1");
    envmnt::set("ENV_SET2", "good2");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "good1".to_string());
    env_values.insert("ENV_SET2".to_string(), "good2".to_string());

    step.config.condition = Some(TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: Some(env_values),
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition_for_step(&flow_info, &step);

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_condition_for_step_invalid_env_not_found() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("BAD_ENV_SET1".to_string(), "good".to_string());
    env_values.insert("BAD_ENV_SET2".to_string(), "bad".to_string());

    step.config.condition = Some(TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: Some(env_values),
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition_for_step(&flow_info, &step);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_condition_for_step_invalid_env_not_equal() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    envmnt::set("ENV_SET1", "good");
    envmnt::set("ENV_SET2", "good");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "good".to_string());
    env_values.insert("ENV_SET2".to_string(), "bad".to_string());

    step.config.condition = Some(TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: Some(env_values),
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition_for_step(&flow_info, &step);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_condition_for_step_valid_env_contains_same() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    envmnt::set("ENV_SET1", "good1");
    envmnt::set("ENV_SET2", "good2");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "good1".to_string());
    env_values.insert("ENV_SET2".to_string(), "good2".to_string());

    step.config.condition = Some(TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: Some(env_values),
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition_for_step(&flow_info, &step);

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_condition_for_step_valid_env_contains() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    envmnt::set("ENV_SET1", "good1");
    envmnt::set("ENV_SET2", "good2");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "good".to_string());
    env_values.insert("ENV_SET2".to_string(), "good".to_string());

    step.config.condition = Some(TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: Some(env_values),
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition_for_step(&flow_info, &step);

    assert!(enabled);
}

#[test]
#[ignore]
fn validate_condition_for_step_invalid_env_contains_not_found() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("BAD_ENV_SET1".to_string(), "good".to_string());
    env_values.insert("BAD_ENV_SET2".to_string(), "bad".to_string());

    step.config.condition = Some(TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: Some(env_values),
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition_for_step(&flow_info, &step);

    assert!(!enabled);
}

#[test]
#[ignore]
fn validate_condition_for_step_invalid_env_contains_not_contains() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    envmnt::set("ENV_SET1", "good");
    envmnt::set("ENV_SET2", "good");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "good".to_string());
    env_values.insert("ENV_SET2".to_string(), "bad".to_string());

    step.config.condition = Some(TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: Some(env_values),
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition_for_step(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_condition_for_step_valid_rust_version() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let rustinfo = rust_info::get();
    let version = rustinfo.version.unwrap();

    step.config.condition = Some(TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: Some(RustVersionCondition {
            min: None,
            max: None,
            equal: Some(version),
        }),
        files_exist: None,
        files_not_exist: None,
    });

    let enabled = validate_condition_for_step(&flow_info, &step);

    assert!(enabled);
}

#[test]
fn validate_condition_for_step_invalid_rust_version() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let rustinfo = rust_info::get();
    let mut version = rustinfo.version.unwrap();
    version.push_str("1");

    step.config.condition = Some(TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: Some(RustVersionCondition {
            min: None,
            max: None,
            equal: Some(version),
        }),
        files_exist: None,
        files_not_exist: None,
    });

    let enabled = validate_condition_for_step(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_no_rustinfo() {
    let rustinfo = RustInfo::new();

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: Some("2.0.0".to_string()),
            max: Some("1.0.0".to_string()),
            equal: Some("3.0.0".to_string()),
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_condition_empty_condition() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: None,
            equal: None,
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_condition_min_enabled() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: Some("1.9.9".to_string()),
            max: None,
            equal: None,
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_condition_min_same() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: Some("2.0.0".to_string()),
            max: None,
            equal: None,
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_condition_min_disabled_major() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: Some("3.0.0".to_string()),
            max: None,
            equal: None,
        },
    );

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_min_disabled_minor() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: Some("2.1.0".to_string()),
            max: None,
            equal: None,
        },
    );

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_min_disabled_patch() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: Some("2.0.1".to_string()),
            max: None,
            equal: None,
        },
    );

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_max_enabled() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("1.9.9".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: Some("2.0.0".to_string()),
            equal: None,
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_condition_max_same() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: Some("2.0.0".to_string()),
            equal: None,
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_condition_max_disabled_major() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("3.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: Some("2.0.0".to_string()),
            equal: None,
        },
    );

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_max_disabled_minor() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.1.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: Some("2.0.0".to_string()),
            equal: None,
        },
    );

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_max_disabled_patch() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.1".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: Some("2.0.0".to_string()),
            equal: None,
        },
    );

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_equal_same() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: None,
            equal: Some("2.0.0".to_string()),
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_condition_equal_not_same() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: None,
            max: None,
            equal: Some("3.0.0".to_string()),
        },
    );

    assert!(!enabled);
}

#[test]
fn validate_rust_version_condition_all_enabled() {
    let mut rustinfo = RustInfo::new();
    rustinfo.version = Some("2.0.0".to_string());

    let enabled = validate_rust_version_condition(
        rustinfo,
        RustVersionCondition {
            min: Some("1.0.0".to_string()),
            max: Some("3.0.0".to_string()),
            equal: Some("2.0.0".to_string()),
        },
    );

    assert!(enabled);
}

#[test]
fn validate_rust_version_no_condition() {
    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_rust_version(&condition);

    assert!(enabled);
}

#[test]
fn validate_rust_version_with_valid_condition() {
    let rustinfo = rust_info::get();
    let version = rustinfo.version.unwrap();

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: Some(RustVersionCondition {
            min: Some(version.clone()),
            max: Some(version.clone()),
            equal: Some(version.clone()),
        }),
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_rust_version(&condition);

    assert!(enabled);
}

#[test]
fn validate_rust_version_with_invalid_condition() {
    let rustinfo = rust_info::get();
    let mut version = rustinfo.version.unwrap();
    version.push_str("1");

    let condition = TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: Some(RustVersionCondition {
            min: None,
            max: None,
            equal: Some(version),
        }),
        files_exist: None,
        files_not_exist: None,
    };

    let enabled = validate_rust_version(&condition);

    assert!(!enabled);
}
