use crate::data::robot_state::RobotState;

pub fn parse_line(line: &str) -> Option<RobotState> {
    match serde_json::from_str::<RobotState>(line) {
        Ok(state) => Some(state),
        Err(e) => {
            eprintln!("Failed to parse robot state from JSON: {}", e);
            None
        }
    }
}