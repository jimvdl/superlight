// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod message;
mod socket;

use message::{Message, Method, Payload};
use socket::Socket;
use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent};

fn main() -> anyhow::Result<()> {
    let tray = SystemTray::new();

    let app = tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(handler)
        .on_window_event(|e| match e.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                e.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let app_handle = app.handle();

    std::thread::spawn(move || -> anyhow::Result<()> {
        let mut socket = Socket::from_ws("ws://localhost:9010")?;

        socket.send(Message::with(Method::Get, "/devices/list"))?;

        loop {
            let message = match socket.receive() {
                Ok(message) => message,
                Err(e) => return Err(e),
            };
            println!("{:?}", message);

            match message.payload() {
                Some(Payload::Devices(payload)) => {
                    if let Some(device) = payload
                        .devices
                        .iter()
                        .find(|&device| device.display_name == "PRO X SUPERLIGHT")
                    {
                        socket.send(Message::with(
                            Method::Get,
                            format!("/battery/{}/state", device.id),
                        ))?;
                    }
                }
                Some(Payload::Battery(battery)) => {
                    app_handle.emit_all("battery-update", battery)?;
                }
                _ => {}
            }
        }
    });

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });

    Ok(())
}

fn handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let window = app.get_window("main").unwrap();

            match window.is_visible() {
                Ok(true) => {
                    window.hide().unwrap();
                }
                Ok(false) => {
                    window.show().unwrap();
                }
                Err(_) => unimplemented!(),
            }
        }
        _ => {}
    }
}
