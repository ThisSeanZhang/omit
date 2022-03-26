mod window;

use std::thread;
use crossbeam::channel::unbounded;

use tauri::Window;
use crate::action::ChannelAction;

#[tauri::command]
pub fn create_pty(window: Window) -> Result<(), String>{
    let window_label = window.label().to_string();

    let (action_send, mut action_get) = unbounded();
    
    let line_send = action_send.clone();
    let front_listen_id = window.listen("ssh-data-from-frontend", move |event| {
        // trx.send(event.payload().unwrap().to_string());
        let message = event.payload().unwrap().to_string();
        println!("message {}", message);
        line_send.send(ChannelAction::Message(message));
        // println!("got window event-name with payload {:?}", event.payload());
    });
    let resize_listen_id = window.listen("ssh-resize-from-front", move|event | {
        if let Some(size_data) = event.payload() {
            action_send.send(serde_json::from_str(size_data).unwrap());
        }
    });
    thread::spawn(move || {
      if let Ok(pty) = window::Pty::new((129, 33)) {
        let window = pty.start_spin(action_get, window);
        window.unlisten(front_listen_id);
        window.unlisten(resize_listen_id);
      }
    });
  Ok(())
}