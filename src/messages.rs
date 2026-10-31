use tonic::Status;

pub const READING_CONFIG_STDIN: &str = "Reading config from stdin...";

pub const LIST_CONFIGS_ERROR: &str = "Error listing configs";
pub const GET_CONFIG_ERROR: &str = "Error getting config";
pub const READ_CONFIG_FILE_ERROR: &str = "Error reading config file";
pub const READ_STDIN_ERROR: &str = "Error reading from stdin";
pub const EDIT_CONFIG_ERROR: &str = "Error editing config";
pub const DELETE_CONFIG_ERROR: &str = "Error deleting config";
pub const START_SERVICE_ERROR: &str = "Error starting service";
pub const RESTART_SERVICE_ERROR: &str = "Error restarting service";
pub const STOP_SERVICE_ERROR: &str = "Error stopping service";
pub const RECEIVE_LOG_STREAM_ERROR: &str = "Error receiving log stream";

pub const CONFIG_EDITED: &str = "Config updated.";
pub const CONFIG_DELETED: &str = "Config deleted.";
pub const SERVICE_STARTED: &str = "Service started.";
pub const SERVICE_RESTARTED: &str = "Service restarted.";
pub const SERVICE_STOPPED: &str = "Service stopped.";

pub fn format_status(status: &Status) -> String {
    format!("{:?}: {}", status.code(), status.message())
}
