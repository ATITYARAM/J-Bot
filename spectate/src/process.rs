use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
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
                ManagedProcess::new(def.clone())
            );

        }

        Self {
            inner: Arc::new(Mutex::new(map))
        }

    }

    pub fn definitions() -> &'static [ProcessDefinition] {
        PROCESS_LIST
    }

    pub fn start(&self, id: &str) -> Result<(), String> {

        let mut processes = self.inner.lock().unwrap();

        let proc = processes
            .get_mut(id)
            .ok_or("Unknown process")?;

        if proc.child.is_some() {
            return Err("Already running".into());
        }

        proc.output.lock().unwrap().clear();

        let mut child = Command::new("bash")
            .arg("-c")
            .arg(proc.definition.command)
            .current_dir("/home/atitya/Documents/J-Bot/ros_ws")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

        proc.stdin = child.stdin.take();

        let output = proc.output.clone();

        if let Some(stdout) = child.stdout.take() {

            thread::spawn(move || {

                let reader = BufReader::new(stdout);

                for line in reader.lines().flatten() {

                    output.lock().unwrap().push(line);

                }

            });

        }

        let output = proc.output.clone();

        if let Some(stderr) = child.stderr.take() {

            thread::spawn(move || {

                let reader = BufReader::new(stderr);

                for line in reader.lines().flatten() {

                    output.lock().unwrap().push(line);

                }

            });

        }

        proc.child = Some(child);

        Ok(())
    }

    pub fn stop(&self, id: &str) -> Result<(), String> {

        let mut processes = self.inner.lock().unwrap();

        let proc = processes
            .get_mut(id)
            .ok_or("Unknown process")?;

        if let Some(child) = proc.child.as_mut() {

            child.kill().map_err(|e| e.to_string())?;

        }

        proc.child = None;
        proc.stdin = None;

        Ok(())
    }

    pub fn running(&self, id: &str) -> bool {

        let processes = self.inner.lock().unwrap();

        processes
            .get(id)
            .map(|p| p.child.is_some())
            .unwrap_or(false)

    }

    pub fn output(&self, id: &str) -> Vec<String> {

        let processes = self.inner.lock().unwrap();

        processes
            .get(id)
            .map(|p| p.output.lock().unwrap().clone())
            .unwrap_or_default()

    }

    pub fn clear(&self, id: &str) {

        let mut processes = self.inner.lock().unwrap();

        if let Some(proc) = processes.get_mut(id) {

            proc.output.lock().unwrap().clear();

        }

    }

    pub fn send_input(
        &self,
        id: &str,
        input: &str,
    ) -> Result<(), String> {

        let mut processes = self.inner.lock().unwrap();

        let proc = processes
            .get_mut(id)
            .ok_or("Unknown process")?;

        let stdin = proc
            .stdin
            .as_mut()
            .ok_or("Process has no stdin")?;

        stdin
            .write_all(input.as_bytes())
            .map_err(|e| e.to_string())?;

        stdin
            .write_all(b"\n")
            .map_err(|e| e.to_string())?;

        stdin
            .flush()
            .map_err(|e| e.to_string())?;

        Ok(())
    }

}

impl ManagedProcess {

    fn new(definition: ProcessDefinition) -> Self {

        Self {

            definition,

            child: None,

            stdin: None,

            output: Arc::new(Mutex::new(Vec::new())),

        }

    }

}
