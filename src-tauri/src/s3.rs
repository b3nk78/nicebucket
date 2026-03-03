mod s3_service;

use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;
use uuid::Uuid;

pub use s3_service::S3Service;

use crate::s3::s3_service::S3ServiceConfig;

pub type ConnectionMap = Arc<Mutex<HashMap<String, ConnectionConfig>>>;

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct Connection {
    id: String,
    label: String,
    provider: BucketProvider,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct CommonConfig {
    pub label: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
    pub mfa_arn: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct R2Config {
    pub common: CommonConfig,
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct S3Config {
    pub common: CommonConfig,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct CustomConfig {
    pub common: CommonConfig,
    pub endpoint_url: String,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct S3AssumeRoleConfig {
    pub label: String,
    pub master_connection_uuid: String,
    pub role_arn: String,
    pub region: Option<String>,
    pub temp_access_key_id: Option<String>,
    pub temp_secret_access_key: Option<String>,
    pub temp_session_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum ConnectionConfig {
    S3(S3Config),
    R2(R2Config),
    Custom(CustomConfig),
    S3AssumeRole(S3AssumeRoleConfig),
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct SavedS3Config {
    pub common: CommonConfig,
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct SavedR2Config {
    pub common: CommonConfig,
    pub account_id: String,
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct SavedCustomConfig {
    pub common: CommonConfig,
    pub endpoint_url: String,
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct SavedS3AssumeRoleConfig {
    pub label: String,
    pub master_connection_uuid: String,
    pub role_arn: String,
    pub region: Option<String>,
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum SavedConnectionConfig {
    S3(SavedS3Config),
    R2(SavedR2Config),
    Custom(SavedCustomConfig),
    S3AssumeRole(SavedS3AssumeRoleConfig),
}

#[derive(Serialize, Deserialize, Type, Debug, Clone, PartialEq)]
pub enum BucketProvider {
    S3,
    R2,
    Custom,
    S3AssumeRole,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct BucketInfo {
    pub provider: BucketProvider,
    pub name: String,
    pub region: String,
    pub endpoint_url: String,
    pub creation_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ObjectInfo {
    pub key: String,
    pub size: Option<i64>,
    pub last_modified: Option<String>,
    pub storage_class: Option<String>,
    pub is_folder: bool,
    pub url: String,
}

#[derive(Serialize, Deserialize, Type)]
pub struct CommonOperationOptions {
    connection: Connection,
    bucket_region: Option<String>,
}

fn build_service_config(
    config: ConnectionConfig,
    bucket_region: Option<String>,
) -> S3ServiceConfig {
    // Normalize empty strings to None so the fallback logic works correctly
    let bucket_region = bucket_region.and_then(|r| if r.is_empty() { None } else { Some(r) });

    match config {
        ConnectionConfig::S3(s3_config) => {
            let region = bucket_region.unwrap_or_else(|| "us-east-1".to_string());

            S3ServiceConfig {
                config: s3_config,
                region: region.clone(),
                endpoint_url: format!("https://s3.{}.amazonaws.com", region),
                provider: BucketProvider::S3,
            }
        }
        ConnectionConfig::R2(r2_config) => {
            let account_id = r2_config.account_id.clone();

            S3ServiceConfig {
                config: S3Config {
                    common: r2_config.common,
                },
                region: "auto".to_string(),
                endpoint_url: format!("https://{}.r2.cloudflarestorage.com", account_id),
                provider: BucketProvider::R2,
            }
        }
        ConnectionConfig::Custom(custom_config) => S3ServiceConfig {
            config: S3Config {
                common: custom_config.common,
            },
            region: "auto".to_string(),
            endpoint_url: custom_config.endpoint_url,
            provider: BucketProvider::Custom,
        },
        ConnectionConfig::S3AssumeRole(assume_role_config) => {
            let region = assume_role_config
                .region
                .clone()
                .and_then(|r| if r.is_empty() { None } else { Some(r) })
                .or(bucket_region)
                .unwrap_or_else(|| "us-east-1".to_string());

            S3ServiceConfig {
                config: S3Config {
                    common: CommonConfig {
                        label: assume_role_config.label,
                        access_key_id: assume_role_config.temp_access_key_id.unwrap_or_default(),
                        secret_access_key: assume_role_config.temp_secret_access_key.unwrap_or_default(),
                        session_token: assume_role_config.temp_session_token,
                        mfa_arn: None,
                    },
                },
                region: region.clone(),
                endpoint_url: format!("https://s3.{}.amazonaws.com", region),
                provider: BucketProvider::S3AssumeRole,
            }
        }
    }
}

async fn create_s3_service(
    opts: &CommonOperationOptions,
    state: State<'_, ConnectionMap>,
) -> Result<S3Service, String> {
    let connections = state.lock().await;
    let config = connections
        .get(&opts.connection.id)
        .ok_or_else(|| "Connection not found".to_string())?
        .clone();

    let service_config = build_service_config(config, opts.bucket_region.clone());

    S3Service::new(service_config)
        .await
        .map_err(|e| format!("Failed to init S3 service: {}", e))
}

async fn create_service_from_config(
    config: ConnectionConfig,
    bucket_region: Option<String>,
) -> Result<S3Service, String> {
    let service_config = build_service_config(config, bucket_region);

    S3Service::new(service_config)
        .await
        .map_err(|e| format!("Failed to init S3 service: {}", e))
}

#[tauri::command]
#[specta::specta]
pub async fn connect_to_s3(
    app: tauri::AppHandle<tauri::Wry>,
    config: ConnectionConfig,
    mfa_token: Option<String>,
    state: State<'_, ConnectionMap>,
) -> Result<Connection, String> {
    let id = Uuid::new_v4().to_string();

    let mut resolved_config = config.clone();
    let mut sts_authenticated = false;

    match resolved_config.clone() {
        ConnectionConfig::S3AssumeRole(mut assume_role) => {
            let saved_connections = crate::keyring::load_saved_connections(app.clone()).await?;
            let master_conn = saved_connections.iter().find(|c| match c {
                crate::s3::SavedConnectionConfig::S3(s) => s.uuid == assume_role.master_connection_uuid,
                _ => false,
            }).ok_or_else(|| "Master connection not found".to_string())?;

            let master_s3 = match master_conn {
                crate::s3::SavedConnectionConfig::S3(s) => s,
                _ => unreachable!(),
            };

            let temp_creds = S3Service::assume_role_with_mfa(
                &master_s3.common.access_key_id,
                &master_s3.common.secret_access_key,
                assume_role.region.as_deref().unwrap_or("us-east-1"),
                &assume_role.role_arn,
                master_s3.common.mfa_arn.as_deref(),
                mfa_token.as_deref(),
            ).await?;

            assume_role.temp_access_key_id = Some(temp_creds.0);
            assume_role.temp_secret_access_key = Some(temp_creds.1);
            assume_role.temp_session_token = Some(temp_creds.2);

            resolved_config = ConnectionConfig::S3AssumeRole(assume_role);
            sts_authenticated = true;
        }
        ConnectionConfig::S3(mut s3_config) => {
            if let (Some(mfa_arn), Some(token)) = (s3_config.common.mfa_arn.clone(), mfa_token.clone()) {
                let temp_creds = S3Service::get_session_token_with_mfa(
                    &s3_config.common.access_key_id,
                    &s3_config.common.secret_access_key,
                    "us-east-1",
                    &mfa_arn,
                    &token,
                ).await?;

                s3_config.common.access_key_id = temp_creds.0;
                s3_config.common.secret_access_key = temp_creds.1;
                s3_config.common.session_token = Some(temp_creds.2);

                resolved_config = ConnectionConfig::S3(s3_config);
                sts_authenticated = true;
            }
        }
        _ => {}
    }

    let connection = match &resolved_config {
        ConnectionConfig::S3(s3_config) => Connection {
            id: id.clone(),
            label: s3_config.common.label.clone(),
            provider: BucketProvider::S3,
        },
        ConnectionConfig::R2(r2_config) => Connection {
            id: id.clone(),
            label: r2_config.common.label.clone(),
            provider: BucketProvider::R2,
        },
        ConnectionConfig::Custom(custom_config) => Connection {
            id: id.clone(),
            label: custom_config.common.label.clone(),
            provider: BucketProvider::Custom,
        },
        ConnectionConfig::S3AssumeRole(assume_role) => Connection {
            id: id.clone(),
            label: assume_role.label.clone(),
            provider: BucketProvider::S3AssumeRole,
        },
    };

    let service = create_service_from_config(resolved_config.clone(), None).await?;

    match service.list_buckets().await {
        Ok(_) => {}
        Err(e) => {
            if !sts_authenticated {
                return Err(format!("Connection failed: {}", e));
            }
            // If STS authenticated successfully, we allow the connection even if S3 list_buckets fails
            // (e.g., due to IAM policies restricting Master accounts to only AssumeRole)
        }
    }

    let mut connections = state.lock().await;
    connections.insert(id, resolved_config);

    Ok(connection)
}

#[tauri::command]
#[specta::specta]
pub async fn list_buckets(
    connection: Connection,
    state: State<'_, ConnectionMap>,
) -> Result<Vec<BucketInfo>, String> {
    let options = CommonOperationOptions {
        connection,
        bucket_region: None,
    };

    let service = create_s3_service(&options, state).await?;

    service
        .list_buckets()
        .await
        .map_err(|e| format!("Failed to list buckets: {}", e))
}

#[derive(Serialize, Deserialize, Type)]
pub struct ListObjectsOptions {
    common: CommonOperationOptions,
    bucket_name: String,
    prefix: Option<String>,
}

#[tauri::command]
#[specta::specta]
pub async fn list_objects(
    opts: ListObjectsOptions,
    state: State<'_, ConnectionMap>,
) -> Result<Vec<ObjectInfo>, String> {
    let service = create_s3_service(&opts.common, state).await?;

    service
        .list_objects(
            &opts.bucket_name,
            opts.prefix.as_deref(),
            false,
            opts.common.bucket_region,
        )
        .await
        .map_err(|e| format!("Failed to list objects: {}", e))
}

#[derive(Serialize, Deserialize, Type)]
pub struct DownloadObjectOptions {
    common: CommonOperationOptions,
    bucket_name: String,
    key: String,
}

#[tauri::command]
#[specta::specta]
pub async fn download_object(
    opts: DownloadObjectOptions,
    state: State<'_, ConnectionMap>,
) -> Result<Vec<u8>, String> {
    let service = create_s3_service(&opts.common, state).await?;

    service
        .download_object(&opts.bucket_name, &opts.key)
        .await
        .map_err(|e| format!("Failed to download object: {}", e))
}

#[derive(Serialize, Deserialize, Type)]
pub struct DownloadObjectsOptions {
    common: CommonOperationOptions,
    bucket_name: String,
    keys: Vec<String>,
}

#[tauri::command]
#[specta::specta]
pub async fn download_objects(
    opts: DownloadObjectsOptions,
    state: State<'_, ConnectionMap>,
) -> Result<Vec<(String, Vec<u8>)>, String> {
    let service = create_s3_service(&opts.common, state).await?;

    service
        .download_objects(&opts.bucket_name, opts.keys)
        .await
        .map_err(|e| format!("Failed to download objects: {}", e))
}

#[derive(Serialize, Deserialize, Type)]
pub struct DeleteObjectsOptions {
    common: CommonOperationOptions,
    bucket_name: String,
    keys: Vec<String>,
}

#[tauri::command]
#[specta::specta]
pub async fn delete_objects(
    opts: DeleteObjectsOptions,
    state: State<'_, ConnectionMap>,
) -> Result<(), String> {
    let service = create_s3_service(&opts.common, state).await?;

    service
        .delete_objects(&opts.bucket_name, opts.keys)
        .await
        .map_err(|e| format!("Failed to delete objects: {}", e))
}

#[derive(Serialize, Deserialize, Type)]
pub struct DownloadFolderOptions {
    common: CommonOperationOptions,
    bucket_name: String,
    prefix: String,
}

#[tauri::command]
#[specta::specta]
pub async fn download_folder(
    opts: DownloadFolderOptions,
    state: State<'_, ConnectionMap>,
) -> Result<Vec<u8>, String> {
    let service = create_s3_service(&opts.common, state).await?;

    service
        .download_folder(&opts.bucket_name, &opts.prefix, opts.common.bucket_region)
        .await
        .map_err(|e| format!("Failed to download folder: {}", e))
}

#[derive(Serialize, Deserialize, Type)]
pub struct UploadObjectsOptions {
    common: CommonOperationOptions,
    bucket_name: String,
    prefix: Option<String>,
    file_paths: Vec<PathBuf>,
}

#[tauri::command]
#[specta::specta]
pub async fn upload_objects(
    opts: UploadObjectsOptions,
    state: State<'_, ConnectionMap>,
) -> Result<(), String> {
    let service = create_s3_service(&opts.common, state).await?;

    service
        .upload_objects(&opts.bucket_name, opts.prefix, opts.file_paths)
        .await
        .map_err(|e| format!("Failed to download objects: {}", e))
}

#[derive(Serialize, Deserialize, Type)]
pub struct CreateFolderOptions {
    common: CommonOperationOptions,
    bucket_name: String,
    folder_key: String,
}

#[tauri::command]
#[specta::specta]
pub async fn create_folder(
    opts: CreateFolderOptions,
    state: State<'_, ConnectionMap>,
) -> Result<(), String> {
    let service = create_s3_service(&opts.common, state).await?;

    service
        .create_folder(&opts.bucket_name, &opts.folder_key)
        .await
        .map_err(|e| format!("Failed to create folder: {}", e))
}

#[derive(Serialize, Deserialize, Type)]
pub struct DeleteFolderOptions {
    common: CommonOperationOptions,
    bucket_name: String,
    prefix: String,
}

#[tauri::command]
#[specta::specta]
pub async fn delete_folder(
    opts: DeleteFolderOptions,
    state: State<'_, ConnectionMap>,
) -> Result<(), String> {
    let service = create_s3_service(&opts.common, state).await?;

    service
        .delete_folder(&opts.bucket_name, &opts.prefix, opts.common.bucket_region)
        .await
        .map_err(|e| format!("Failed to delete folder: {}", e))
}

#[derive(Serialize, Deserialize, Type)]
pub struct MoveObjectsOptions {
    common: CommonOperationOptions,
    bucket_name: String,
    keys: Vec<String>,
    destination_prefix: String,
}

#[tauri::command]
#[specta::specta]
pub async fn move_objects(
    opts: MoveObjectsOptions,
    state: State<'_, ConnectionMap>,
) -> Result<(), String> {
    let service = create_s3_service(&opts.common, state).await?;

    service
        .move_objects(&opts.bucket_name, opts.keys, &opts.destination_prefix)
        .await
        .map_err(|e| format!("Failed to move objects: {}", e))
}
