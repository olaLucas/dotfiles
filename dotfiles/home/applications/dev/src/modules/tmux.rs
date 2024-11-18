use std::path::PathBuf;
use std::process::exit;

use crate::proc;
use crate::cli;

use log::info;
use log::error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
  pub struct Session {
  pub session_name: String,
  pub workdir: PathBuf,
  pub applications: Vec<String>
}

impl Session {
  
  pub fn from(session_name: String, workdir: PathBuf, applications: Vec<String>) -> Session {
    Session {
      session_name,
      workdir,
      applications
    } 
  }
  
  // its throwing:
  // thread 'main' has overflowed its stack
  // fatal runtime error: stack overflow
  // Aborted (core dumped)

  pub fn parse(args: cli::Args) -> Session {
    let working_directory = match args.fzf {
      true => cli::fzf(),
      false => args.workdir
    };

    let session_name = args.session_name;
    let applications = args.applicatons;

    Session::from(session_name, working_directory, applications)
  }
}

// create fn create_session
// create fn create_window

pub fn create_session(session: &Session) {
  let args = ["new-session", "-d", "-c", &session.workdir.to_str().unwrap(), "-s", &session.session_name].to_vec();
  match proc::execute("tmux", args) {
    Ok(_) => info!("session {} succefully created." , &session.session_name),
    Err(e) => {
      error!("failed to create tmux session {}: {}. exiting", &session.session_name, e);
      exit(-1);
    }
  }
}

pub fn create_windows(session: &Session) {
  for app in session.applications.iter() {
    match proc::execute("tmux", ["new-window",  app].to_vec()) {
      Ok(_) => info!("window {} succefully created.", app),
      Err(e) => { 
        error!("failed to create window {}, stderr: {}. exiting.", app, e);
        kill_session(session);
        exit(-1);
      }
    }
  }
}

pub fn kill_session(session: &Session) {
  match proc::execute("tmux", ["kill-session"].to_vec()) {
    Ok(_) => info!("killed session {}", &session.session_name), 
    Err(e) => error!("failed to kill {}: {}", &session.session_name, e) 
  }
}