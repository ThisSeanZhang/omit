use std::{io::Write, fmt::format};
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use anyhow::{Result as AyResult, Ok};
use crossbeam::channel::unbounded;
use futures::select;
// use log::info;
use russh::*;
use russh_keys::*;
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};

use std::thread;
use tauri::Window;
use crate::action::{ChannelAction, FrontEndPorter, FrontEndAction};

#[derive(Serialize, Deserialize, Clone)]
struct SSHMessage {
  data: Vec<u8>,
}
#[derive(Deserialize)]
pub struct SSHInfo<'a> {
    pub uid: &'a str,
    pub ip: &'a str,
    pub port: u32,
    pub username: &'a str,
    pub passwd: &'a str,
}

#[tauri::command(async)]
pub async fn create_pty(window: Window, SSHInfo{ uid, username, ip, port, passwd}: SSHInfo<'_>) -> Result<(), String>{
    // let (action_send, mut action_get) = unbounded();
    // let ssh = Session::connect("aaaaa", "aaaa", SocketAddr::from_str("10.1.1.130:2333").unwrap()).await;
    // let mut ssh = ssh.unwrap();
    // let r = ssh.call("ls -la\n").await;
    // let r = r.unwrap();
    // assert!(r.success());
    // println!("Result: {}", r.output());
    // ssh.close().await.unwrap();
    println!("{} {} {} {}", username, ip, port, passwd);
    let username = username.to_string();
    let passwd = passwd.to_string();
    let address = SocketAddr::from_str(format!("{}:{}", ip, port).as_str()).unwrap();
    let (action_send, mut action_get) = mpsc::channel(32);
    let line_send = action_send.clone();
    let front_listen_id = window.listen(format!("data-from-front_{}", uid), move |event| {
        // trx.send(event.payload().unwrap().to_string());
        let message = event.payload().unwrap().to_string();
        // println!("message {}", message);
        let clone_again = line_send.clone();
        tokio::spawn(async move {
            clone_again.send(ChannelAction::Message(message)).await;
        });
        // line_send.send(ChannelAction::Message(message));
        // line_send.blocking_send(ChannelAction::Message(message)).unwrap();
        // println!("got window event-name with payload {:?}", event.payload());
    });
    let resize_listen_id = window.listen(format!("resize-from-front_{}", uid), move|event | {
        if let Some(size_data) = event.payload().clone() {
            let message = serde_json::from_str(size_data).unwrap();
            let clone_again = action_send.clone();
            tokio::spawn(async move {
                clone_again.send(message).await;
            });
        }
    });
    let config = client::Config {
        connection_timeout: Some(Duration::from_secs(5)),
        ..<_>::default()
      };
      let config = Arc::new(config);
      let mut session = client::connect(config, address,  Client {}).await.map_err(|e| e.to_string())?;
      let _auth_res = session
    //   .authenticate_none(username)
      .authenticate_password(username, passwd)
        // .authenticate_publickey(user, Arc::new(key_pair))
      .await.map_err(|e| e.to_string())?;
      let mut channel = session.channel_open_session().await.map_err(|e| e.to_string())?;
      println!("open channel");
      channel.request_pty(false, "xterm-256color", 129, 33, 0,0, &[
        (Pty::ECHO, 1),
        (Pty::ECHOCTL, 0),
        (Pty::TTY_OP_ISPEED, 14400),
        (Pty::TTY_OP_OSPEED, 14400),
        (Pty::TTY_OP_END, 1)]).await.unwrap();
      channel.request_shell(true).await.unwrap();
      let send_emit_action_name = format!("action-from-backend_{}", uid);
    tokio::spawn(async move {
          while !session.is_closed() {
            tokio::select!{
                Some(msg) = channel.wait() => {
                    println!("wait: {:?}", msg);
                    match msg {
                        russh::ChannelMsg::Data { ref data } => {
                            let a = std::str::from_utf8(&data);
                            // println!("output {}", a.unwrap_or("error"));
                            window.emit(&send_emit_action_name, FrontEndPorter::shell_data(data.to_vec())).unwrap();
                        }
                        russh::ChannelMsg::Eof => {
                            window.emit(&send_emit_action_name, FrontEndPorter::action(FrontEndAction::Eof, None)).unwrap();
                            break;
                        }
                        _ => {println!("nothing happend")}
                    }
                }
                Some(action) = action_get.recv()  => {
                    println!("{:?}", action);
                    match action {
                        ChannelAction::Message(line) => {
                            println!("revc {}", line);
                            channel.data(line.as_bytes()).await.unwrap();
                        },
                        ChannelAction::SizeChange{ width, height, width_px, height_px } => {
                            channel.window_change(width, height, 0, 0).await.unwrap();
                            // self.process.resize(width as i16, height as i16);
                            // todo!();
                        },
                        // ChannelAction::EXIT => {

                        // },
                        _ => println!("")
                    }
            }
          }
        }
        //   channel.window_change(30, 30, 640,480).await.unwrap();
          // channel.exec(true, "a").await.unwrap();
        //   let mut output = Vec::new();
        //   let mut code = None;
        //   while let Some(msg) = channel.wait().await {
        //       println!("wait: {:?}", msg);
        //       match msg {
        //           russh::ChannelMsg::Data { ref data } => {
        //               output.write_all(data).unwrap();
        //               let a = std::str::from_utf8(&data);
        //               println!("output {}", a.unwrap_or("error"));
        //           }
        //           russh::ChannelMsg::ExitStatus { exit_status } => {
        //               code = Some(exit_status);
        //           }
        //           _ => {println!("nothing happend")}
        //       }
        //   }
        window.unlisten(front_listen_id);
        window.unlisten(resize_listen_id);
    });
    Result::Ok(())
}

// #[tokio::main]
// async fn main() -> Result<()> {
//     // env_logger::builder()
//     //     .filter_level(log::LevelFilter::Debug)
//     //     .init();

//     // let args: Vec<String> = std::env::args().collect();
//     // let (host, key) = match args.get(1..3) {
//     //     Some(args) => (&args[0], &args[1]),
//     //     None => {
//     //         eprintln!("Usage: {} <host:port> <private-key-path>", args[0]);
//     //         std::process::exit(1);
//     //     }
//     // };

//     // info!("Connecting to {host}");
//     // info!("Key path: {key}");

//     Ok(())
// }

struct Client {}

impl client::Handler for Client {
    type Error = russh::Error;
    type FutureUnit = futures::future::Ready<Result<(Self, client::Session), Self::Error>>;
    type FutureBool = futures::future::Ready<Result<(Self, bool), Self::Error>>;

    fn finished_bool(self, b: bool) -> Self::FutureBool {
      println!("finished_bool: {}", b);
        futures::future::ready(Result::Ok((self, b)))
    }
    fn finished(self, session: client::Session) -> Self::FutureUnit {
        futures::future::ready(Result::Ok((self, session)))
    }
    fn check_server_key(self, _server_public_key: &key::PublicKey) -> Self::FutureBool {
        self.finished_bool(true)
    }
}

pub struct Session {
    session: client::Handle<Client>,
}

impl Session {
    async fn connect<P: AsRef<Path>>(
        key_path: P,
        user: impl Into<String>,
        addr: SocketAddr,
    ) -> AyResult<Self> {
        // let key_pair = load_secret_key(key_path, None)?;
        let config = client::Config {
            connection_timeout: Some(Duration::from_secs(5)),
            ..<_>::default()
        };
        let config = Arc::new(config);
        let sh = Client {};
        let mut session = client::connect(config, addr, sh).await?;
        let _auth_res = session
        .authenticate_password("test", "test")
            // .authenticate_publickey(user, Arc::new(key_pair))
            .await?;
        println!("is_closed {}", session.is_closed());
        Ok(Self { session })
    }

    async fn call(&mut self, command: &str) -> AyResult<CommandResult> {
        let mut channel = self.session.channel_open_session().await?;
        println!("open channel");
        channel.request_pty(false, "xterm", 129, 33, 8,8, &[
          (Pty::ECHO, 1),
          (Pty::ECHOCTL, 1),
          (Pty::TTY_OP_ISPEED, 14400),
          (Pty::TTY_OP_OSPEED, 14400)]).await?;
        channel.data(&b"whoami"[..]);
        channel.exec(true, command).await?;
        println!("command: {}", command);
        let mut output = Vec::new();
        let mut code = None;
        while let Some(msg) = channel.wait().await {
          println!("wait: {:?}", msg);
            match msg {
                russh::ChannelMsg::Data { ref data } => {
                    output.write_all(data).unwrap();
                    let a = std::str::from_utf8(&output);
                    println!("output {}", a.unwrap_or("error"));
                }
                russh::ChannelMsg::ExitStatus { exit_status } => {
                    code = Some(exit_status);
                }
                _ => {println!("nothing happend")}
            }
        }
        Ok(CommandResult { output, code })
    }

    async fn close(&mut self) -> AyResult<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}

struct CommandResult {
    output: Vec<u8>,
    code: Option<u32>,
}

impl CommandResult {
    fn output(&self) -> String {
        String::from_utf8_lossy(&self.output).into()
    }

    fn success(&self) -> bool {
        self.code == Some(0)
    }
}
