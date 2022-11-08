// use std::{thread, io::{Read, Write}};

// use conpty::{self, Process};
// use crossbeam::channel::{Receiver, TryRecvError};
// use serde::{Serialize, Deserialize};
// use tauri::Window;
// use crate::{error::{OmitError, OmitErrorType}, action::{Porter, ChannelAction}};

// pub struct Pty {
//   process: Process,
//   size: (i16, i16),
// }

// impl Pty {
//   pub fn new(size: (i16, i16)) -> Result<Pty, OmitError> {
//     let process = conpty::spawn("powershell.exe").map_err(OmitError::parse_pty_error)?;
//     process.resize(size.0, size.1);
//     Ok(Pty {
//       process,
//       size,
//     })
//   }

//   pub fn start_spin(&self, action_get: Receiver<ChannelAction>, window: Window) -> Window {
//       let mut output = self.process.output().unwrap();
//       let mut input = self.process.input().unwrap();
//       output.set_non_blocking_mode().unwrap();
//       let mut buf = [0; 4096];
//       while self.process.is_alive() {
//           match output.read(&mut buf) {
//               Ok(size) => {
//                   println!("size: {}", size);
//                   // let s = String::from_utf8(buf[0..size].to_vec()).unwrap();
//                   // println!("size: {}", size);
//                   // println!("size: {}, s len: {} {}", size, s.len(),s);
//                   window.emit("ssh-data-from-backend", Porter { data: buf[0..size].to_vec() }).unwrap();
//               }
//               Err(e) => {
//                   if e.kind() != std::io::ErrorKind::WouldBlock {
//                       println!("err {}", e);
//                       break;
//                   }
//               }
//           }
//           if !action_get.is_empty() {
//               match action_get.try_recv() {
//                   Ok(action) => {
//                       match action {
//                           ChannelAction::Message(line) => {
//                               input.write(line.as_bytes()).unwrap();
//                               input.flush().unwrap();
//                           },
//                           ChannelAction::SizeChange{ width, height, width_px, height_px } => {
//                             self.process.resize(width as i16, height as i16);
//                           },
//                           // ChannelAction::EXIT => {

//                           // },
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
//           std::thread::sleep(std::time::Duration::from_millis(5));
//       }
//       println!("is_alive: {}", self.process.is_alive());
//       window.emit("ssh-data-from-backend", Porter { data: "lose connection".into() }).unwrap();
//       window
//   }
// }

// impl OmitError {
//   fn parse_pty_error(err: conpty::error::Error) -> OmitError {
//     OmitError {
//       t: OmitErrorType::Default,
//       message: err.to_string()
//     }
//   }
// }
