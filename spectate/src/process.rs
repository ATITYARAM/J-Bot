use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
    os::unix::process::CommandExt,
};

use nix::{
    sys::signal::{killpg, Signal},
    unistd::{setpgid, Pid},
};

#[derive(Clone)]
pub struct ProcessManager {
    inner: Arc<Mutex<HashMap<String, ManagedProcess>>>,
}

#[derive(Clone)]
pub struct ProcessDefinition {
    pub id: &'static str,
    pub title: &'static str,
    pub command: &'static str,
    pub interactive: bool,
}

struct ManagedProcess {

    definition: ProcessDefinition,

    child: Option<Child>,

    stdin: Option<ChildStdin>,

    output: Arc<Mutex<Vec<String>>>,

}

const PROCESS_LIST: &[ProcessDefinition] = &[

    ProcessDefinition {

        id: "teleop",

        title: "Teleop",

        command: "ros2 run teleop viaduct",

        interactive: false,

    },

    ProcessDefinition {

        id: "build",

        title: "Build Workspace",

        command: "colcon build",

        interactive: false,

    },

    ProcessDefinition {

        id: "s3",

        title: "S3 Node",

        command: "source install/setup.bash && ros2 run s3 viaduct",

        interactive: false,

    },

    ProcessDefinition {

        id: "keyboard",

        title: "Keyboard Teleop",

        command: "ros2 run teleop teleop",

        interactive: true,

    },

];

impl ProcessManager {

    pub fn new() -> Self {

        let mut map = HashMap::new();

        for def in PROCESS_LIST {

            map.insert(
                def.id.to_string(),
                ManagedProcess::new(def.clone()),
            );

        }

        Self {

            inner: Arc::new(Mutex::new(map)),

        }

    }

    pub fn definitions() -> &'static [ProcessDefinition] {

        PROCESS_LIST

    }
    /* ==========================================================
       START PROCESS
    ========================================================== */

    pub fn start(
        &self,
        id: &str,
    ) -> Result<(), String> {

        let mut processes = self.inner.lock().unwrap();

        let proc = processes
            .get_mut(id)
            .ok_or("Unknown process")?;

        if proc.child.is_some() {
            return Err("Process already running".into());
        }

        proc.output.lock().unwrap().clear();

        let mut command = Command::new("bash");

        command
            .arg("-c")
            .arg(proc.definition.command)
            .current_dir("/home/atitya/Documents/J-Bot/ros_ws")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        unsafe {

            command.pre_exec(|| {

                setpgid(
                    Pid::from_raw(0),
                    Pid::from_raw(0),
                )
                .map_err(std::io::Error::other)?;

                Ok(())

            });

        }

        let mut child = command
            .spawn()
            .map_err(|e| e.to_string())?;

        proc.stdin = child.stdin.take();

        if let Some(stdout) = child.stdout.take() {

            let output = proc.output.clone();

            thread::spawn(move || {

                let reader = BufReader::new(stdout);

                for line in reader.lines().flatten() {

                    output
                        .lock()
                        .unwrap()
                        .push(line);

                }

            });

        }

        if let Some(stderr) = child.stderr.take() {

            let output = proc.output.clone();

            thread::spawn(move || {

                let reader = BufReader::new(stderr);

                for line in reader.lines().flatten() {

                    output
                        .lock()
                        .unwrap()
                        .push(line);

                }

            });

        }

        proc.child = Some(child);

        Ok(())

    }

    /* ==========================================================
       STOP SINGLE PROCESS
    ========================================================== */

    pub fn stop(
        &self,
        id: &str,
    ) -> Result<(), String> {

        let mut processes = self.inner.lock().unwrap();

        let proc = processes
            .get_mut(id)
            .ok_or("Unknown process")?;

        Self::kill_process_group(proc)?;

        Ok(())

    }

    /* ==========================================================
       STOP ALL PROCESSES
    ========================================================== */

    pub fn stop_all(&self) {

        let mut processes = self.inner.lock().unwrap();

        for proc in processes.values_mut() {

            let _ = Self::kill_process_group(proc);

        }

    }

    /* ==========================================================
       DISCONNECT ONLY
    ========================================================== */

    pub fn disconnect_all(&self) {

        let mut processes = self.inner.lock().unwrap();

        for proc in processes.values_mut() {

            proc.stdin.take();

            proc.child.take();

            proc.output
                .lock()
                .unwrap()
                .push(String::from(
                    "[SPECTATE DISCONNECTED]"
                ));

        }

    }

    /* ==========================================================
       KILL PROCESS GROUP
    ========================================================== */

    fn kill_process_group(
        proc: &mut ManagedProcess,
    ) -> Result<(), String> {

        if let Some(child) = proc.child.as_mut() {

            let pgid =
                Pid::from_raw(child.id() as i32);

            let _ = killpg(
                pgid,
                Signal::SIGTERM,
            );

            std::thread::sleep(
                std::time::Duration::from_millis(500),
            );

            match child.try_wait() {

                Ok(Some(_)) => {}

                _ => {

                    let _ = killpg(
                        pgid,
                        Signal::SIGKILL,
                    );

                }

            }

            let _ = child.wait();

            proc.output
                .lock()
                .unwrap()
                .push(String::from(
                    "[PROCESS STOPPED]"
                ));

        }

        proc.child = None;

        proc.stdin = None;

        Ok(())

    }

    /* ==========================================================
       RUNNING
    ========================================================== */

    pub fn running(
        &self,
        id: &str,
    ) -> bool {

        let mut processes =
            self.inner.lock().unwrap();

        let proc =
            match processes.get_mut(id) {

                Some(proc) => proc,

                None => return false,

            };

        if let Some(child) = proc.child.as_mut() {

            match child.try_wait() {

                Ok(Some(status)) => {

                    proc.output
                        .lock()
                        .unwrap()
                        .push(format!(
                            "[PROCESS EXITED] {}",
                            status
                        ));

                    proc.child = None;
                    proc.stdin = None;

                    false

                }

                Ok(None) => true,

                Err(err) => {

                    proc.output
                        .lock()
                        .unwrap()
                        .push(format!(
                            "[WAIT ERROR] {}",
                            err
                        ));

                    proc.child = None;
                    proc.stdin = None;

                    false

                }

            }

        } else {

            false

        }

    }

    /* ==========================================================
       OUTPUT
    ========================================================== */

    pub fn output(
        &self,
        id: &str,
    ) -> Vec<String> {

        let processes =
            self.inner.lock().unwrap();

        processes

            .get(id)

            .map(|proc| {

                proc.output
                    .lock()
                    .unwrap()
                    .clone()

            })

            .unwrap_or_default()

    }

    /* ==========================================================
       CLEAR
    ========================================================== */

    pub fn clear(
        &self,
        id: &str,
    ) {

        let mut processes =
            self.inner.lock().unwrap();

        if let Some(proc) =
            processes.get_mut(id)
        {

            proc.output
                .lock()
                .unwrap()
                .clear();

        }

    }

    /* ==========================================================
       SEND INPUT
    ========================================================== */

    pub fn send_input(
        &self,
        id: &str,
        input: &str,
    ) -> Result<(), String> {

        let mut processes =
            self.inner.lock().unwrap();

        let proc =
            processes
                .get_mut(id)
                .ok_or("Unknown process")?;

        let stdin =
            proc.stdin
                .as_mut()
                .ok_or("Process not running")?;

        stdin
            .write_all(input.as_bytes())
            .map_err(|e| e.to_string())?;

        stdin
            .flush()
            .map_err(|e| e.to_string())?;

        Ok(())

    }

}

/* ==========================================================
   MANAGED PROCESS
========================================================== */

impl ManagedProcess {

    fn new(
        definition: ProcessDefinition,
    ) -> Self {

        Self {

            definition,

            child: None,

            stdin: None,

            output: Arc::new(
                Mutex::new(Vec::new())
            ),

        }

    }

}
