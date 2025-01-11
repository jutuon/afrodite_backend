/*
 * dating-app-backend
 *
 * Dating app backend API
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Permissions {
    #[serde(rename = "admin_ban_account", skip_serializing_if = "Option::is_none")]
    pub admin_ban_account: Option<bool>,
    #[serde(rename = "admin_delete_account", skip_serializing_if = "Option::is_none")]
    pub admin_delete_account: Option<bool>,
    #[serde(rename = "admin_delete_media_content", skip_serializing_if = "Option::is_none")]
    pub admin_delete_media_content: Option<bool>,
    #[serde(rename = "admin_find_account_by_email", skip_serializing_if = "Option::is_none")]
    pub admin_find_account_by_email: Option<bool>,
    #[serde(rename = "admin_moderate_media_content", skip_serializing_if = "Option::is_none")]
    pub admin_moderate_media_content: Option<bool>,
    #[serde(rename = "admin_moderate_profile_names", skip_serializing_if = "Option::is_none")]
    pub admin_moderate_profile_names: Option<bool>,
    #[serde(rename = "admin_moderate_profile_texts", skip_serializing_if = "Option::is_none")]
    pub admin_moderate_profile_texts: Option<bool>,
    #[serde(rename = "admin_modify_permissions", skip_serializing_if = "Option::is_none")]
    pub admin_modify_permissions: Option<bool>,
    #[serde(rename = "admin_news_create", skip_serializing_if = "Option::is_none")]
    pub admin_news_create: Option<bool>,
    #[serde(rename = "admin_news_edit_all", skip_serializing_if = "Option::is_none")]
    pub admin_news_edit_all: Option<bool>,
    #[serde(rename = "admin_profile_statistics", skip_serializing_if = "Option::is_none")]
    pub admin_profile_statistics: Option<bool>,
    #[serde(rename = "admin_request_account_deletion", skip_serializing_if = "Option::is_none")]
    pub admin_request_account_deletion: Option<bool>,
    #[serde(rename = "admin_server_maintenance_reboot_backend", skip_serializing_if = "Option::is_none")]
    pub admin_server_maintenance_reboot_backend: Option<bool>,
    #[serde(rename = "admin_server_maintenance_reset_data", skip_serializing_if = "Option::is_none")]
    pub admin_server_maintenance_reset_data: Option<bool>,
    #[serde(rename = "admin_server_maintenance_save_backend_config", skip_serializing_if = "Option::is_none")]
    pub admin_server_maintenance_save_backend_config: Option<bool>,
    #[serde(rename = "admin_server_maintenance_update_software", skip_serializing_if = "Option::is_none")]
    pub admin_server_maintenance_update_software: Option<bool>,
    #[serde(rename = "admin_server_maintenance_view_backend_config", skip_serializing_if = "Option::is_none")]
    pub admin_server_maintenance_view_backend_config: Option<bool>,
    /// View server infrastructure related info like logs and software versions.
    #[serde(rename = "admin_server_maintenance_view_info", skip_serializing_if = "Option::is_none")]
    pub admin_server_maintenance_view_info: Option<bool>,
    /// View public and private profiles.
    #[serde(rename = "admin_view_all_profiles", skip_serializing_if = "Option::is_none")]
    pub admin_view_all_profiles: Option<bool>,
    #[serde(rename = "admin_view_permissions", skip_serializing_if = "Option::is_none")]
    pub admin_view_permissions: Option<bool>,
    #[serde(rename = "admin_view_private_info", skip_serializing_if = "Option::is_none")]
    pub admin_view_private_info: Option<bool>,
    #[serde(rename = "admin_view_profile_history", skip_serializing_if = "Option::is_none")]
    pub admin_view_profile_history: Option<bool>,
}

impl Permissions {
    pub fn new() -> Permissions {
        Permissions {
            admin_ban_account: None,
            admin_delete_account: None,
            admin_delete_media_content: None,
            admin_find_account_by_email: None,
            admin_moderate_media_content: None,
            admin_moderate_profile_names: None,
            admin_moderate_profile_texts: None,
            admin_modify_permissions: None,
            admin_news_create: None,
            admin_news_edit_all: None,
            admin_profile_statistics: None,
            admin_request_account_deletion: None,
            admin_server_maintenance_reboot_backend: None,
            admin_server_maintenance_reset_data: None,
            admin_server_maintenance_save_backend_config: None,
            admin_server_maintenance_update_software: None,
            admin_server_maintenance_view_backend_config: None,
            admin_server_maintenance_view_info: None,
            admin_view_all_profiles: None,
            admin_view_permissions: None,
            admin_view_private_info: None,
            admin_view_profile_history: None,
        }
    }
}

