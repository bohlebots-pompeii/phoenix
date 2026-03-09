use crate::data::robot_state::RobotState;

pub fn parse_line(line: &str) -> Option<RobotState> {
    let trimmed = line.trim();
    // Silently skip empty lines and lines that are clearly not JSON objects
    if !trimmed.starts_with('{') {
        return None;
    }
    match serde_json::from_str::<RobotState>(trimmed) {
        Ok(state) => Some(state),
        Err(e) => {
            eprintln!("Failed to parse robot state from JSON: {}", e);
            None
        }
    }
}