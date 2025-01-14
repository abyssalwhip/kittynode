use eyre::Result;
use kittynode_core::domain::package::Package;
use kittynode_core::domain::system_info::SystemInfo;
use std::collections::HashMap;
use std::sync::LazyLock;
use tauri_plugin_http::reqwest;
use tracing::info;

pub static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

#[tauri::command]
async fn add_capability(name: String, server_url: String) -> Result<(), String> {
    info!("Adding capability: {}", name);

    if !server_url.is_empty() {
        let url = format!("{}/add_capability/{}", server_url, name);
        let res = HTTP_CLIENT
            .post(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!("Failed to add capability: {}", res.status()));
        }
        Ok(())
    } else {
        kittynode_core::application::add_capability(&name).map_err(|e| e.to_string())
    }
}

#[tauri::command]
async fn remove_capability(name: String, server_url: String) -> Result<(), String> {
    info!("Removing capability: {}", name);

    if !server_url.is_empty() {
        let url = format!("{}/remove_capability/{}", server_url, name);
        let res = HTTP_CLIENT
            .post(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!("Failed to remove capability: {}", res.status()));
        }
        Ok(())
    } else {
        kittynode_core::application::remove_capability(&name).map_err(|e| e.to_string())
    }
}

#[tauri::command]
async fn get_capabilities(server_url: String) -> Result<Vec<String>, String> {
    info!("Getting capabilities");

    if !server_url.is_empty() {
        let url = format!("{}/get_capabilities", server_url);
        let res = HTTP_CLIENT
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let status = res.status();
        let error_text = res.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(format!(
                "Failed to get capabilities: {} - {}",
                status, error_text
            ));
        }

        let res = HTTP_CLIENT
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        res.json::<Vec<String>>().await.map_err(|e| e.to_string())
    } else {
        kittynode_core::application::get_capabilities().map_err(|e| e.to_string())
    }
}

#[tauri::command]
fn get_packages() -> Result<HashMap<String, Package>, String> {
    info!("Getting packages");
    kittynode_core::application::get_packages()
        .map(|packages| {
            packages
                .into_iter()
                .map(|(name, package)| (name.to_string(), package))
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_installed_packages(server_url: String) -> Result<Vec<Package>, String> {
    info!("Getting installed packages");

    if !server_url.is_empty() {
        let url = format!("{}/get_installed_packages", server_url);
        let res = HTTP_CLIENT
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let status = res.status();
        let error_text = res.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(format!(
                "Failed to get installed packages: {} - {}",
                status, error_text
            ));
        }

        let res = HTTP_CLIENT
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        res.json::<Vec<Package>>().await.map_err(|e| e.to_string())
    } else {
        kittynode_core::application::get_installed_packages()
            .await
            .map_err(|e| e.to_string())
    }
}

#[tauri::command]
async fn is_docker_running() -> bool {
    info!("Checking if Docker is running");
    kittynode_core::application::is_docker_running().await
}

#[tauri::command]
async fn install_package(name: String, server_url: String) -> Result<(), String> {
    if !server_url.is_empty() {
        let url = format!("{}/install_package/{}", server_url, name);
        let res = HTTP_CLIENT
            .post(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!("Failed to install package: {}", res.status()));
        }
    } else {
        kittynode_core::application::install_package(&name)
            .await
            .map_err(|e| e.to_string())?;
    }

    info!("Successfully installed package: {}", name);
    Ok(())
}

#[tauri::command]
async fn delete_package(
    name: String,
    include_images: bool,
    server_url: String,
) -> Result<(), String> {
    if !server_url.is_empty() {
        let url = format!("{}/delete_package/{}", server_url, name);
        let res = HTTP_CLIENT
            .post(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!("Failed to delete package: {}", res.status()));
        }
    } else {
        kittynode_core::application::delete_package(&name, include_images)
            .await
            .map_err(|e| e.to_string())?;
    }

    info!("Successfully deleted package: {}", name);
    Ok(())
}

#[tauri::command]
async fn delete_kittynode(server_url: String) -> Result<(), String> {
    info!("Deleting .kittynode directory");

    if !server_url.is_empty() {
        let url = format!("{}/delete_kittynode", server_url);
        let res = HTTP_CLIENT
            .post(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!("Failed to delete Kittynode: {}", res.status()));
        }
        Ok(())
    } else {
        kittynode_core::application::delete_kittynode().map_err(|e| e.to_string())
    }
}

#[tauri::command]
async fn system_info(server_url: String) -> Result<SystemInfo, String> {
    info!("Getting system info");

    if !server_url.is_empty() {
        let url = format!("{}/get_system_info", server_url);
        let res = HTTP_CLIENT
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let status = res.status();
        let error_text = res.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(format!(
                "Failed to get system info: {} - {}",
                status, error_text
            ));
        }

        let res = HTTP_CLIENT
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        res.json::<SystemInfo>().await.map_err(|e| e.to_string())
    } else {
        kittynode_core::application::get_system_info().map_err(|e| e.to_string())
    }
}

#[tauri::command]
async fn init_kittynode(server_url: String) -> Result<(), String> {
    info!("Initializing Kittynode");

    if !server_url.is_empty() {
        let url = format!("{}/init_kittynode", server_url);
        let res = HTTP_CLIENT
            .post(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!("Failed to initialize Kittynode: {}", res.status()));
        }
        Ok(())
    } else {
        kittynode_core::application::init_kittynode().map_err(|e| e.to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init());

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let builder = builder.plugin(tauri_plugin_updater::Builder::new().build());

    builder
        .invoke_handler(tauri::generate_handler![
            get_packages,
            get_installed_packages,
            is_docker_running,
            install_package,
            delete_package,
            delete_kittynode,
            system_info,
            init_kittynode,
            add_capability,
            remove_capability,
            get_capabilities,
        ])
        .run(tauri::generate_context!())
        .map_err(|e| eyre::eyre!(e.to_string()))?;

    Ok(())
}
