fn main() {
  // let test = "Hello, world!";
  // println!("{}", &test[0..0].len());
  // println!("{}", (1,2,3) == (1,2,3));
  login_ssh2();
}
use ssh2::Session;
use std::future::Ready;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::{ Mutex, Arc};
use std::thread;
use std::time::Duration;
fn login_ssh() {
  let tcp = TcpStream::connect("xxxx").unwrap();
  let mut sess = Session::new().unwrap();
  sess.set_tcp_stream(tcp);
  sess.handshake().unwrap();
  sess.userauth_password("user", "pass").unwrap();
  let mut channel = sess.channel_session().unwrap();
  channel.request_pty(
      "xterm",
       None,
       Some((80, 24, 0, 0)),
      ).unwrap();
  channel.shell().unwrap();
  let mutex_ch = Arc::new(Mutex::new(channel));
  let in_ch = mutex_ch.clone();
  let in_handle = thread::spawn(move || {
      let end = false;

      {
          thread::sleep(Duration::from_secs(2));
          let mut ch = in_ch.lock().unwrap();
          println!("eof _ {}", ch.eof());
          // ch.eof();
          println!("start send ls -la");
          // ch.exec( "ls -la").unwrap();
          ch.write_fmt(format_args!("{}\n", "ls -la")).unwrap();
          // ch.flush().unwrap();
          println!("end send ls -la");
      }
      thread::sleep(Duration::from_secs(2));
      {
          let mut ch = in_ch.lock().unwrap();
          println!("eof _ {}", ch.eof());
          // ch.eof();
          println!("start send exit");
          // ch.exec( "ls -la").unwrap();
          ch.write_fmt(format_args!("{}\n", "exit")).unwrap();
          // ch.flush().unwrap();
          println!("end send exit");
          println!("eof _ {}", ch.exit_status().unwrap());
      }
  });
  
  let out_ch = mutex_ch.clone();
  let out_handle = thread::spawn(move || {
      let mut buf= [0 as u8;1024];
      for _ in 0..5 {
          thread::sleep(Duration::from_secs(5));
          let mut ch = out_ch.lock().unwrap();
          println!("eof _ {}", ch.eof());
          // let mut s = String::new();
          // let size = ch.read(&mut buf).unwrap();
          // ch.read_window(&mut buf).unwrap();
          let bytes_available = ch.read_window().available;
          if bytes_available > 0 {
              let mut buffer = vec![0; bytes_available as usize];
              ch.read_exact(&mut buffer).unwrap();
              println!("out {}", String::from_utf8(buffer).unwrap());
          }
          // println!("size {}", size);
          // match std::str::from_utf8(&buf) {
          //     // Ok(v) => println!("out {}", &v[0..size]),
          //     Ok(v) => println!("out {}", v),
          //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
          // };
          // println!("out _ {}", std::str::from_utf8(&buf).unwrap());
      }
  });
  in_handle.join().unwrap();
  out_handle.join().unwrap();
  // channel.exec("top").unwrap();
  // let mut s = String::new();
  // channel.read_to_string(&mut s).unwrap();
  // println!("{}", s);
  println!("{}", mutex_ch.lock().unwrap().eof());
  mutex_ch.lock().unwrap().wait_close().ok();
  // channel.wait_close().ok();
  println!("{}", mutex_ch.lock().unwrap().exit_status().unwrap());
  println!("{}", mutex_ch.lock().unwrap().eof());
}

use crossbeam::channel::{unbounded, TryRecvError};
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
