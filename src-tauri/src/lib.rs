// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn toggle_window(window: tauri::Window) -> Result<(), String> {
    if window.is_visible().unwrap_or(false) {
        window.hide().map_err(|e| e.to_string())?;
    } else {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn set_window_always_on_top(window: tauri::Window) -> Result<(), String> {
    // 设置窗口始终在顶层
    // 先取消底层设置，再设置顶层
    window.set_always_on_bottom(false).map_err(|e| e.to_string())?;
    window.set_always_on_top(true).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn set_window_always_on_bottom(window: tauri::Window) -> Result<(), String> {
    // 设置窗口始终在底层
    // 先取消顶层设置，再设置底层
    window.set_always_on_top(false).map_err(|e| e.to_string())?;
    window.set_always_on_bottom(true).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn set_window_normal_level(window: tauri::Window) -> Result<(), String> {
    // 设置窗口为正常层级（取消置顶和置底）
    window.set_always_on_top(false).map_err(|e| e.to_string())?;
    window.set_always_on_bottom(false).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn start_drag(window: tauri::Window) -> Result<(), String> {
    window.start_dragging().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn toggle_expanded(window: tauri::Window, is_expanded: bool) -> Result<(), String> {
    if is_expanded {
        // 展开：设置窗口高度为570px
        window.set_size(tauri::Size::Physical(tauri::PhysicalSize { width: 400, height: 570 }))
            .map_err(|e| e.to_string())?;
    } else {
        // 收缩：设置窗口高度为56px（仅标题栏）
        window.set_size(tauri::Size::Physical(tauri::PhysicalSize { width: 400, height: 56 }))
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn set_window_pinned(_window: tauri::Window, _pinned: bool) -> Result<(), String> {
    // 固定功能只控制拖拽，不影响大小调整
    // 窗口大小调整始终可用
    Ok(())
}

#[tauri::command]
async fn exit_app(app_handle: tauri::AppHandle) -> Result<(), String> {
    app_handle.exit(0);
    Ok(())
}

#[tauri::command]
async fn open_repository(url: String) -> Result<(), String> {
    use std::process::Command;
    
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", &url])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[tauri::command]
async fn save_data_to_file(_data: String, _file_path: Option<String>) -> Result<String, String> {
    // 这个函数现在只作为占位符，实际的文件保存在前端处理
    Ok("文件保存功能已移至前端".to_string())
}

#[tauri::command]
async fn load_data_from_file(file_path: String) -> Result<String, String> {
    use std::fs::read_to_string;
    let content = read_to_string(&file_path).map_err(|e| e.to_string())?;
    Ok(content)
}

#[tauri::command]
async fn select_save_file() -> Result<Option<String>, String> {
    // 在Tauri 2.0中，文件对话框功能需要在前端处理
    // 这里返回一个占位符，实际的文件选择在前端处理
    Ok(None)
}

#[tauri::command]
async fn select_load_file() -> Result<Option<String>, String> {
    // 在Tauri 2.0中，文件对话框功能需要在前端处理
    // 这里返回一个占位符，实际的文件选择在前端处理
    Ok(None)
}

#[tauri::command]
async fn show_main_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        let _ = window.unminimize();
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, toggle_window, set_window_always_on_bottom, set_window_always_on_top, set_window_normal_level, start_drag, toggle_expanded, set_window_pinned, exit_app, open_repository, save_data_to_file, load_data_from_file, select_save_file, select_load_file, show_main_window])
        .setup(|app| {
            let _app_handle = app.handle().clone();
            
            // 创建右键菜单
            let show_item = tauri::menu::MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
            let quit_item = tauri::menu::MenuItem::with_id(app, "quit", "关闭", true, None::<&str>)?;
            
            let menu = tauri::menu::Menu::with_items(app, &[
                &show_item,
                &quit_item,
            ])?;
            
            // 创建系统托盘
            let _app_handle = app.handle().clone();
            let _tray = tauri::tray::TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("T-DoList - 轻量级任务清单")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| {
                    match event.id().as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(move |_app, event| {
                    match event {
                        tauri::tray::TrayIconEvent::Click { button, .. } => {
                            // 只处理左键点击，右键点击让系统处理菜单
                            if button == tauri::tray::MouseButton::Left {
                                // 左键点击托盘图标时直接显示窗口
                                if let Some(window) = _app_handle.get_webview_window("main") {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                    let _ = window.unminimize();
                                }
                            }
                        }
                        _ => {}
                    }
                })
                .build(app)?;
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
