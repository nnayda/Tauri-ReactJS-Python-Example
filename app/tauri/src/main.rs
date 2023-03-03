
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn start_backend(receiver: std::sync::mpsc::Receiver<i32>) {
    let t = tauri::api::process::Command::new_sidecar("test")
      .expect("python service command not created");
    let mut group = std::process::Command::from(t).spawn().expect("python service not spawned");
    std::thread::spawn(move || {
      loop{
        let mut s = receiver.recv();
        if s.unwrap()==-1 {
          group.kill().expect("python service not killed");
        }
      }
    });
  }
  
fn main() {

    let (tx,rx) = std::sync::mpsc::sync_channel(1);
    start_backend(rx);
    tauri::Builder::default()
        .on_window_event(move |event| match event.event() {
        tauri::WindowEvent::Destroyed => {
            println!("app starting shutdown");
            tx.send(-1).expect("could not send kill signal to python service");
            println!("app shutdown");
        }
        _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
