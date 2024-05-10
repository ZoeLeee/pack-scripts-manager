use std::collections::HashMap;
use std::process::Child;
use tokio::process::Command;

pub struct CommandManager {
    running_commands: HashMap<String, Child>,
}

impl CommandManager {
    fn new() -> Self {
        CommandManager {
            running_commands: HashMap::new(),
        }
    }

    async fn run_script(
        &mut self,
        identifier: String,
        path: &str,
        script: &str,
    ) -> Result<String, String> {
        let output = Command::new("npm")
            .args(["run", script])
            .current_dir(path)
            .output()
            .await
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            // 如果命令执行成功，将输出转换为字符串并返回
            let output_str = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(output_str)
        } else {
            // 如果命令执行失败，返回错误信息
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            Err(error)
        }
    }

    fn add_running_command(&mut self, identifier: String, child: Child) {
        self.running_commands.insert(identifier, child);
    }

    fn kill_command(&mut self, identifier: &str) -> Result<(), String> {
        if let Some(mut child) = self.running_commands.remove(identifier) {
            match child.kill() {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to terminate child process: {}", e)),
            }
        } else {
            Err(format!(
                "Command with identifier '{}' not found",
                identifier
            ))
        }
    }
}
