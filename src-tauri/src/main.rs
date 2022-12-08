mod algorithms;
mod aux_data_structures;
mod graph;
mod mesh;

// Reference: https://rfdonnelly.github.io/posts/tauri-async-rust-process/

use app::protobufs;
use mesh::serial_connection::{MeshConnection, SerialConnection};
use std::sync::{Arc, Mutex};
use tauri::Manager;

struct ActiveSerialConnection {
    inner: Arc<Mutex<Option<mesh::serial_connection::SerialConnection>>>,
}

mod aux_functions;
use aux_functions::commands::{run_articulation_point, run_global_mincut, run_stoer_wagner};
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .manage(ActiveSerialConnection {
            inner: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            run_articulation_point,
            run_global_mincut,
            run_stoer_wagner
        ])
        .invoke_handler(tauri::generate_handler![
            get_all_serial_ports,
            connect_to_serial_port,
            send_text
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_all_serial_ports() -> Vec<String> {
    SerialConnection::get_available_ports()
}

#[tauri::command]
fn connect_to_serial_port(
    port_name: String,
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, ActiveSerialConnection>,
) -> bool {
    let mut connection: SerialConnection = MeshConnection::new();

    // ? Might be able to return the listener from here
    match connection.connect(port_name, 115_200) {
        Ok(_h) => (),
        Err(_e) => {
            eprintln!("Could not connect to radio");
            return false;
        }
    };

    let mut decoded_listener = match connection.on_decoded_packet.as_ref() {
        Some(l) => l.resubscribe(),
        None => {
            eprintln!("Decoded packet listener not open");
            return false;
        }
    };

    let handle = app_handle.app_handle().clone();

    tauri::async_runtime::spawn(async move {
        loop {
            if let Ok(message) = decoded_listener.recv().await {
                println!("[main.rs] Decoded packet: {:?}", message.id);

                handle.emit_all("demo", message.clone()).unwrap_or(());

                let variant = match message.payload_variant {
                    Some(v) => v,
                    None => continue,
                };

                match variant {
                    protobufs::from_radio::PayloadVariant::Channel(c) => {
                        println!("Channel data: {:#?}", c);
                    }
                    protobufs::from_radio::PayloadVariant::Config(c) => {
                        println!("Config data: {:#?}", c);
                    }
                    protobufs::from_radio::PayloadVariant::ConfigCompleteId(c) => {
                        println!("Config complete id data: {:#?}", c);
                    }
                    protobufs::from_radio::PayloadVariant::LogRecord(l) => {
                        println!("Log record data: {:#?}", l);
                    }
                    protobufs::from_radio::PayloadVariant::ModuleConfig(m) => {
                        println!("Module config data: {:#?}", m);
                    }
                    protobufs::from_radio::PayloadVariant::MyInfo(m) => {
                        println!("My node info data: {:#?}", m);
                    }
                    protobufs::from_radio::PayloadVariant::NodeInfo(n) => {
                        println!("Node info data: {:#?}", n);
                    }
                    protobufs::from_radio::PayloadVariant::Packet(p) => {
                        println!("Packet data: {:#?}", p);
                    }
                    protobufs::from_radio::PayloadVariant::Rebooted(r) => {
                        println!("Rebooted data: {:#?}", r);
                    }
                };
            }
        }
    });

    {
        let mut state_connection = state.inner.lock().unwrap();
        *state_connection = Some(connection);
    }

    true
}

#[tauri::command]
fn send_text(text: String, state: tauri::State<'_, ActiveSerialConnection>) -> bool {
    let mut guard = state.inner.lock().unwrap();
    let connection = guard.as_mut().expect("Connection not initialized");
    let result = connection.send_text(text, 0);

    match result {
        Ok(()) => (),
        Err(_e) => {
            eprintln!("Could not send text to radio");
            return false;
        }
    };

    true
}

// __TAURI_INVOKE__("get_all_serial_ports").then(console.log).catch(console.error)
// __TAURI_INVOKE__("connect_to_serial_port", { portName: "/dev/ttyACM0" }).then(console.log).catch(console.error)
// __TAURI_INVOKE__("send_text", { text: "hello world" }).then(console.log).catch(console.error)
