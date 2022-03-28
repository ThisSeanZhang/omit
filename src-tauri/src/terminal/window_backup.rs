// use std::{thread, io::{Read, Write}, time::Instant};

// use std::ffi::OsString;
// use winptyrs::{PTY, PTYArgs, MouseMode, AgentConfig, PTYBackend};
// use crossbeam::channel::{Receiver, TryRecvError};
// use serde::{Serialize, Deserialize};
// use tauri::Window;
// use crate::{error::{OmitError, OmitErrorType}, action::{Porter, ChannelAction}};

// pub struct Pty {
//   process: PTY,
//   size: (i32, i32),
// }

// impl Pty {
//   pub fn new(size: (i32, i32)) -> Result<Pty, OmitError> {
//     let cmd = OsString::from("powershell.exe");
//     println!("pty init size {:?}", size);
//     let pty_args = PTYArgs {
//         cols: size.0,
//         rows: size.1,
//         mouse_mode: MouseMode::WINPTY_MOUSE_MODE_NONE,
//         timeout: 10000,
//         agent_config: AgentConfig::WINPTY_FLAG_COLOR_ESCAPES
//     };
//     let mut process = PTY::new_with_backend(&pty_args, PTYBackend::WinPTY).map_err(OmitError::parse_pty_error)?;
//     process.spawn(cmd, None, None, None).unwrap();
//     Ok(Pty {
//       process,
//       size,
//     })
//   }

//   pub fn start_spin(&self, action_get: Receiver<ChannelAction>, window: Window) -> Window {
//       while self.process.is_alive().unwrap() {
//         let now = Instant::now();
//           match self.process.read(200, false) {
//               Ok(os_string) => {
                
//                   println!("read {}", now.elapsed().as_millis());
//                   if !os_string.is_empty() {
//                     println!("empty {}", now.elapsed().as_millis());
//                     println!("os_string: {}", os_string.len());
                    
//                     println!("len {}", now.elapsed().as_millis());
//                     // let s = String::from_utf8(buf[0..os_string].to_vec()).unwrap();
//                     // println!("os_string: {}", os_string);
//                     // println!("os_string: {}, s len: {} {}", os_string, s.len(),s);
//                     window.emit("ssh-data-from-backend", Porter { data: os_string.to_string_lossy().as_bytes().to_vec() }).unwrap();
                    
//                     println!("emit {}", now.elapsed().as_millis());
//                   }
                  
//               }
//               Err(e) => {
//                 println!("err {:?}", e);
//               }
//           }
//           println!("middle {}", now.elapsed().as_millis());
//           if !action_get.is_empty() {
//               match action_get.try_recv() {
//                   Ok(action) => {
//                     println!("action {}", now.elapsed().as_millis());
//                       match action {
//                           ChannelAction::Message(line) => {
//                             self.process.write(line.into()).unwrap();
//                           },
//                           ChannelAction::SizeChange{ width, height, width_px, height_px } => {
//                             self.process.set_size(width, height).unwrap();
//                           },
//                           _ => println!("")
//                       }
//                   }
//                   Err(TryRecvError::Empty) => {
//                       println!("{}", "empty");
//                   }
//                   Err(TryRecvError::Disconnected) => {
//                       println!("{}", "disconnected");
//                   }
//               }
//           }
//           println!("loop {}", now.elapsed().as_millis());
//           std::thread::sleep(std::time::Duration::from_millis(5));
//           println!("loop end {}", now.elapsed().as_millis());
//       }
//       println!("is_alive: {}", self.process.is_alive().unwrap());
//       window.emit("ssh-data-from-backend", Porter { data: "lose connection".into() }).unwrap();
//       window
//   }
// }

// impl OmitError {
//   fn parse_pty_error(err: OsString) -> OmitError {
//     OmitError {
//       t: OmitErrorType::Default,
//       message: err.to_string_lossy().to_string()
//     }
//   }
// }
