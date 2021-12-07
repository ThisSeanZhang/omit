use ssh2::{Channel, Session};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{io::prelude::*};
use std::net::TcpStream;
use std::{env, thread};
use crossbeam::channel::{Receiver, Sender, TryRecvError, unbounded};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use io::prelude::{conpty, Process};

use tauri::{AppHandle, Window, WindowUrl, command, State};

pub struct SSHState(pub Arc<Mutex<HashMap<String, SSHSessionManage>>>);

pub struct SSHSessionManage {
    sess: Session,
}

#[derive(Serialize, Deserialize)]
struct SSHMessage {
  data: Vec<u8>,
}
#[derive(Serialize, Deserialize, Debug)]
enum ChannelAction {
    Message(String),
    SizeChange {
        width: u32,
        height: u32,
        width_px: Option<u32>,
        height_px: Option<u32>
    }
}

// fn read_channel(channel: &mut Channel, action_get: Receiver<ChannelAction>, window: Window) {
//     println!("stare read channel");
//     let mut buf = [0; 4096];
//     while !channel.eof() {
//         match channel.read(&mut buf) {
//             Ok(size) => {
//                 let s = String::from_utf8(buf[0..size].to_vec()).unwrap();
//                 println!("size: {}, s len: {} {}", size, s.len(),s);
//                 window.emit("ssh-data-from-backend", SSHMessage { data: buf[0..size].to_vec() }).unwrap();
//             }
//             Err(e) => {
//                 if e.kind() != std::io::ErrorKind::WouldBlock {
//                     println!("err {}", e);
//                     break;
//                 }
//             }
//         }
//         if !action_get.is_empty() {
//             match action_get.try_recv() {
//                 Ok(action) => {
//                     match action {
//                         ChannelAction::Message(line) => {
//                             channel.write(line.as_bytes()).unwrap();
//                             channel.flush().unwrap();
//                         },
//                         ChannelAction::SizeChange{ width, height, width_px, height_px } => {
//                             channel.request_pty_size(width, height, width_px, height_px);
//                         },
//                         _ => println!("")
//                     }
//                 }
//                 Err(TryRecvError::Empty) => {
//                     println!("{}", "empty");
//                 }
//                 Err(TryRecvError::Disconnected) => {
//                     println!("{}", "disconnected");
//                 }
//             }
//         }
//         // if !receiver.is_empty() {
//         //     match receiver.try_recv() {
//         //         Ok(line) => {
                    
//         //             // let cmd_string = if line == "\n" {
//         //             //     "\n".to_string()
//         //             // } else {
//         //             //     line + "\n"
//         //             // };
//         //             // println!("{:?}", line.bytes());
//         //             // let cmd_string = line + "\n";
//         //             channel.write(line.as_bytes()).unwrap();
//         //             channel.flush().unwrap();
//         //             // channel.write_fmt(format_args!("{}\n", line)).unwrap();
//         //         }
  
//         //         Err(TryRecvError::Empty) => {
//         //             println!("{}", "empty");
//         //         }
  
//         //         Err(TryRecvError::Disconnected) => {
//         //             println!("{}", "disconnected");
//         //         }
//         //         // Err(e) => {
//         //         //     println!("{:?}", e);
//         //         // }
//         //         // _=> ()
//         //     }
//         // }
//         std::thread::sleep(std::time::Duration::from_millis(5));
//     }
//     window.emit("ssh-data-from-backend", SSHMessage { data: "lose connection".into() }).unwrap();
// }

fn read_pty_output(process: &mut Process, action_get: Receiver<ChannelAction>, window: Window) {
    println!("stare read channel");
    let mut buf = [0; 4096];
    let output = process.output().unwrap();
    let mut input = process.input().unwrap();
    while process.is_alive() {
        match output.read(&mut buf) {
            Ok(size) => {
                let s = String::from_utf8(buf[0..size].to_vec()).unwrap();
                println!("size: {}, s len: {} {}", size, s.len(),s);
                window.emit("ssh-data-from-backend", SSHMessage { data: buf[0..size].to_vec() }).unwrap();
            }
            Err(e) => {
                if e.kind() != std::io::ErrorKind::WouldBlock {
                    println!("err {}", e);
                    break;
                }
            }
        }
        if !action_get.is_empty() {
            match action_get.try_recv() {
                Ok(action) => {
                    match action {
                        ChannelAction::Message(line) => {
                            input.write(line.as_bytes()).unwrap();
                            input.flush().unwrap();
                        },
                        ChannelAction::SizeChange{ width, height, width_px, height_px } => {
                            process.resize(width, height);
                        },
                        _ => println!("")
                    }
                }
                Err(TryRecvError::Empty) => {
                    println!("{}", "empty");
                }
                Err(TryRecvError::Disconnected) => {
                    println!("{}", "disconnected");
                }
            }
        }
        // if !receiver.is_empty() {
        //     match receiver.try_recv() {
        //         Ok(line) => {
                    
        //             // let cmd_string = if line == "\n" {
        //             //     "\n".to_string()
        //             // } else {
        //             //     line + "\n"
        //             // };
        //             // println!("{:?}", line.bytes());
        //             // let cmd_string = line + "\n";
        //             channel.write(line.as_bytes()).unwrap();
        //             channel.flush().unwrap();
        //             // channel.write_fmt(format_args!("{}\n", line)).unwrap();
        //         }
  
        //         Err(TryRecvError::Empty) => {
        //             println!("{}", "empty");
        //         }
  
        //         Err(TryRecvError::Disconnected) => {
        //             println!("{}", "disconnected");
        //         }
        //         // Err(e) => {
        //         //     println!("{:?}", e);
        //         // }
        //         // _=> ()
        //     }
        // }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    window.emit("ssh-data-from-backend", SSHMessage { data: "lose connection".into() }).unwrap();
}

#[derive(Deserialize)]
pub struct SSHInfo<'a> {
    ip: &'a str,
    port: u8,
    username: &'a str,
    passwd: &'a str,
}
#[command]
pub fn current_path(window: Window) -> String {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    // tauri::api::path::document_dir()
    // .unwrap()
    // .to_str()
    // .unwrap()
    // .to_string()
    path.display().to_string()
}
#[command]
pub fn add_listen(window: Window) {
    window.listen("event", |event| {
        println!("got window event-name with payload {:?}", event.payload());
    });
}
#[command]
pub fn create_ssh(window: Window, ssh_state: State<SSHState>, SSHInfo { ip, port, username, passwd }: SSHInfo) {
    println!("use ssh method");
    let window_label = window.label().to_string();
    {
        if ssh_state.0.lock().unwrap().contains_key(&window_label) {
            return;
        }
    }
    
    let ssh_proc = conpty::spawn(format!("ssh {}@{} -p {}", username, ip, port)).unwrap();
    ssh_proc.resize(129, 33);
    
    
    let (action_send, action_get) = unbounded();
    let line_send = action_send.clone();
    let front_listen_id = window.listen("ssh-data-from-frontend", move |event| {
        // trx.send(event.payload().unwrap().to_string());
        line_send.send(ChannelAction::Message(event.payload().unwrap().to_string()));
        // println!("got window event-name with payload {:?}", event.payload());
    });
    // window.unlisten(front_listen_id);
    let resize_listen_id = window.listen("ssh-resize-from-front", move|event | {
        if let Some(size_data) = event.payload() {
            action_send.send(serde_json::from_str(size_data).unwrap());
        }
    });
    thread::spawn(move || {
        read_pty_output(ssh_proc, action_get, window);
    });
    // let my_uuid = Uuid::new_v4();
    // ssh_state.0.lock().unwrap().insert(window_label, SSHSessionManage { 
    //     sess,
    // });
}

/*
  创建新的窗口  以后多窗口使用
*/
#[command]
pub async  fn new_window(app_handle: AppHandle, window: Window, ssh_state: State<'_, SSHState>) -> Result<(), String> {
  println!("request create window from: {}", window.label());
//   {
//     println!("sshState is None{:?}", ssh_state.0.lock().unwrap());
//   }
  let my_uuid = Uuid::new_v4();
  app_handle
    .create_window("main2_".to_string() + &my_uuid.to_simple().to_string(), WindowUrl::default(), |win, webview| (win, webview))
    .expect("Error creating window");
  println!("- done");
  Ok(())
}

fn login_ssh2() {
  let tcp = TcpStream::connect("ip").unwrap();
  let mut sess = Session::new().unwrap();
  sess.set_tcp_stream(tcp);
  sess.handshake().unwrap();
  sess.userauth_password("user", "pass").unwrap();

  let mut channel = sess.channel_session().unwrap();

  channel.request_pty(
      "xterm",
       None,
       Some((80, 30, 0, 0)),
      ).unwrap();
  channel.shell().unwrap();

  sess.set_blocking(false);
  
  let (trx, rev) = unbounded();
  fn trim_newline(s: &mut String) {
      if s.ends_with('\n') {
          s.pop();
          if s.ends_with('\r') {
              s.pop();
          }
      }
  }
  thread::spawn(move || loop {
      let stdin = std::io::stdin();
      let mut line = String::new();
      stdin.read_line(&mut line).unwrap();
      trim_newline(&mut line);
      trx.send(line).unwrap();
  });

  let mut buf = [0; 4096];
  while !channel.eof() {
      // let bytes_available = channel.read_window().available;
      // if bytes_available > 0 {
      //     let mut buffer = vec![0; bytes_available as usize];
      //     channel.read_exact(&mut buffer).unwrap();
      //     println!("{}", String::from_utf8(buffer).unwrap());
      // }
      match channel.read(&mut buf) {
          Ok(size) => {
              let s = String::from_utf8(buf[0..size].to_vec()).unwrap();
              println!("size: {}, s len: {} {}", size, s.len(),s);
          }
          Err(e) => {
              if e.kind() != std::io::ErrorKind::WouldBlock {
                  println!("err {}", e);
              }
          }
      }
      if !rev.is_empty() {
          match rev.try_recv() {
              Ok(line) => {
                  
                  // let cmd_string = if line == "\n" {
                  //     "\n".to_string()
                  // } else {
                  //     line + "\n"
                  // };
                  println!("{:?}", line.bytes());
                  let cmd_string = line + "\n";
                  channel.write(cmd_string.as_bytes()).unwrap();
                  channel.flush().unwrap();
                  // channel.write_fmt(format_args!("{}\n", line)).unwrap();
              }

              Err(TryRecvError::Empty) => {
                  println!("{}", "empty");
              }

              Err(TryRecvError::Disconnected) => {
                  println!("{}", "disconnected");
              }
              // Err(e) => {
              //     println!("{:?}", e);
              // }
              // _=> ()
          }
      }
      std::thread::sleep(std::time::Duration::from_millis(50));
      println!("blocking: {}", sess.is_blocking());
  }
}
