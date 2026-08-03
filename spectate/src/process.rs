use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};

#[derive(Clone)]
pub struct ProcessManager {
    inner: Arc<Mutex<HashMap<String, ManagedProcess>>>,
}

struct ManagedProcess {
    child: Option<Child>,
    output: Arc<Mutex<Vec<String>>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        let mut map = HashMap::new();

        map.insert("teleop".into(), ManagedProcess::new());
        map.insert("build".into(), ManagedProcess::new());
        map.insert("s3".into(), ManagedProcess::new());

        Self {
            inner: Arc::new(Mutex::new(map)),
        }
    }

    pub fn start(&self, name: &str) -> Result<(), String> {
        let mut processes = self.inner.lock().unwrap();

        let proc = processes
            .get_mut(name)
            .ok_or("Unknown process")?;

        if proc.child.is_some() {
            return Err("Already running".into());
        }

        let command = command_for(name)?;

        let mut child = Command::new("bash")
            .arg("-c")
            .arg(command)
            .current_dir(workspace())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

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

    pub fn stop(&self, name: &str) -> Result<(), String> {
        let mut processes = self.inner.lock().unwrap();

        let proc = processes
            .get_mut(name)
            .ok_or("Unknown process")?;

        if let Some(child) = proc.child.as_mut() {
            child.kill().map_err(|e| e.to_string())?;
        }

        proc.child = None;

        Ok(())
    }

    pub fn running(&self, name: &str) -> bool {
        let processes = self.inner.lock().unwrap();

        processes
            .get(name)
            .map(|p| p.child.is_some())
            .unwrap_or(false)
    }

    pub fn output(&self, name: &str) -> Vec<String> {
        let processes = self.inner.lock().unwrap();

        processes
            .get(name)
            .map(|p| p.output.lock().unwrap().clone())
            .unwrap_or_default()
    }

    pub fn clear(&self, name: &str) {
        if let Some(proc) = self.inner.lock().unwrap().get_mut(name) {
            proc.output.lock().unwrap().clear();
        }
    }
}

impl ManagedProcess {
    fn new() -> Self {
        Self {
            child: None,
            output: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

fn workspace() -> &'static str {
    "/home/atitya/Documents/J-Bot/ros_ws"
}

fn command_for(name: &str) -> Result<&'static str, String> {
    match name {
        "teleop" => Ok("ros2 run teleop viaduct"),

        "build" => Ok("colcon build"),

        "s3" => Ok(
            "source install/setup.bash && ros2 run s3 viaduct"
        ),

        _ => Err("Unknown process".into()),
    }
}
