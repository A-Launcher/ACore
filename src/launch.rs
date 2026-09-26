/// A request handed to the runtime launcher.
///
/// ACore defines the data contract only. Downloading files, resolving
/// libraries, selecting a JVM, and starting the process belong to higher-level
/// components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchRequest {
    pub game_dir: String,
    pub java_executable: String,
    pub main_class: String,
    pub jvm_args: Vec<String>,
    pub classpath: Vec<String>,
    pub program_args: Vec<String>,
}

impl LaunchRequest {
    pub fn new(
        game_dir: impl Into<String>,
        java_executable: impl Into<String>,
        main_class: impl Into<String>,
    ) -> Self {
        Self {
            game_dir: game_dir.into(),
            java_executable: java_executable.into(),
            main_class: main_class.into(),
            jvm_args: Vec::new(),
            classpath: Vec::new(),
            program_args: Vec::new(),
        }
    }

    pub fn with_jvm_arg(mut self, arg: impl Into<String>) -> Self {
        self.jvm_args.push(arg.into());
        self
    }

    pub fn with_classpath_entry(mut self, path: impl Into<String>) -> Self {
        self.classpath.push(path.into());
        self
    }

    pub fn with_program_arg(mut self, arg: impl Into<String>) -> Self {
        self.program_args.push(arg.into());
        self
    }
}

/// Result of handing a launch request to a runtime implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchResult {
    pub process_id: Option<u32>,
    pub exit_code: Option<i32>,
}

impl LaunchResult {
    pub const fn started(process_id: u32) -> Self {
        Self {
            process_id: Some(process_id),
            exit_code: None,
        }
    }

    pub const fn exited(exit_code: i32) -> Self {
        Self {
            process_id: None,
            exit_code: Some(exit_code),
        }
    }
}
