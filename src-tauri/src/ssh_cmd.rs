use ssh2::{Channel, Session};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::{io::prelude::*};
use std::net::TcpStream;
use std::{env, thread};
use crossbeam::channel::{Receiver, Sender, TryRecvError, unbounded};
use uuid::Uuid;
use serde::Deserialize;

use tauri::{AppHandle, Window, WindowUrl, command, State, Manager};

pub struct SSHState(pub Arc<Mutex<HashMap<String, SSHSessionManage>>>);

pub struct SSHSessionManage {
    sess: Session,
}

#[derive(serde::Serialize)]
struct SSHMessage {
  data: Vec<u8>,
}

fn read_channel(channel: &mut Channel, receiver: Receiver<String>, window: Window) {
    println!("stare read channel");
    let mut buf = [0; 4096];
    while !channel.eof() {
        match channel.read(&mut buf) {
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
        if !receiver.is_empty() {
            match receiver.try_recv() {
                Ok(line) => {
                    
                    // let cmd_string = if line == "\n" {
                    //     "\n".to_string()
                    // } else {
                    //     line + "\n"
                    // };
                    // println!("{:?}", line.bytes());
                    // let cmd_string = line + "\n";
                    channel.write(line.as_bytes()).unwrap();
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
    let tcp = TcpStream::connect(format!("{}:{}", ip, port)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(username, passwd).unwrap();
    let mut channel = sess.channel_session().unwrap();
    channel.request_pty(
        "xterm",
         None,
         Some((129, 33, 0, 0)),
        ).unwrap();
    channel.shell().unwrap();
    
    sess.set_blocking(false);
    let (trx, rev):(Sender<String>, Receiver<String>) = unbounded();
    let front_listen_id = window.listen("ssh-data-from-frontend", move |event| {
        trx.send(event.payload().unwrap().to_string());
        // println!("got window event-name with payload {:?}", event.payload());
    });
    
    thread::spawn(move || {
        read_channel(&mut channel, rev, window);
    });
    // let my_uuid = Uuid::new_v4();
    ssh_state.0.lock().unwrap().insert(window_label, SSHSessionManage { 
        sess,
    });
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
