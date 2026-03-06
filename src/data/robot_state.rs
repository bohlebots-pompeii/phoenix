#[derive(Clone, Debug)]
pub struct RobotState {
    pub vision: VisionData,
    pub sensors: SensorData,
    pub motion: MotionData,
    pub game: GameState,
    pub peer: PeerRobot,
    pub esp_now_bot_id: i8,
    pub console_print: Vec<f64>,
}

impl Default for RobotState {
    fn default() -> Self {
        Self {
            vision: VisionData::default(),
            sensors: SensorData::default(),
            motion: MotionData::default(),
            game: GameState::default(),
            peer: PeerRobot::default(),
            esp_now_bot_id: -1,
            console_print: vec![],
        }
    }
}

//
// ---------------- Vision (CM5) ----------------
//

#[derive(Clone, Debug)]
pub struct VisionData {
    pub heading: f32,
    pub global_x: f32,
    pub global_y: f32,

    pub ball_rot: f32,
    pub ball_dist: f32,
    pub ball_exists: bool,

    pub target_goal_rot: f32,
    pub target_goal_dist: f32,

    pub own_goal_rot: f32,
    pub own_goal_dist: f32,

    pub away_from_own_goal_angle: f32,

    pub target_goal_label: u8,
    pub own_goal_label: u8,

    pub num_detections: u8,

    pub object_label: [u8; 6],
    pub object_rot_deg: [f32; 6],
    pub object_dist_cm: [f32; 6],

    pub cm5_running: bool,
}

impl Default for VisionData {
    fn default() -> Self {
        Self {
            heading: 0.0,
            global_x: 0.0,
            global_y: 0.0,

            ball_rot: 0.0,
            ball_dist: 0.0,
            ball_exists: false,

            target_goal_rot: 0.0,
            target_goal_dist: 0.0,

            own_goal_rot: 0.0,
            own_goal_dist: 0.0,

            away_from_own_goal_angle: 0.0,

            target_goal_label: 0,
            own_goal_label: 0,

            num_detections: 0,

            object_label: [0; 6],
            object_rot_deg: [0.0; 6],
            object_dist_cm: [0.0; 6],

            cm5_running: false,
        }
    }
}

//
// ---------------- Sensors ----------------
//

#[derive(Clone, Debug)]
pub struct SensorData {
    pub line_rot: i16,
    pub progress: i16,
    pub line_seen: bool,

    pub has_ball: bool,
    pub ball_light_gate: i16,

    pub ena: bool,
}

impl Default for SensorData {
    fn default() -> Self {
        Self {
            line_rot: 0,
            progress: 0,
            line_seen: false,

            has_ball: false,
            ball_light_gate: 0,

            ena: false,
        }
    }
}

//
// ---------------- Motion / Positioning ----------------
//

#[derive(Clone, Debug)]
pub struct MotionData {
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub velocity_magnitude: f32,

    pub rotation_delta: f32,

    pub middle_point_vec_x: f32,
    pub middle_point_vec_y: f32,
    pub middle_point_vec_angle: f32,
    pub middle_point_vec_magnitude: f32,
}

impl Default for MotionData {
    fn default() -> Self {
        Self {
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_magnitude: 0.0,

            rotation_delta: 0.0,

            middle_point_vec_x: 0.0,
            middle_point_vec_y: 0.0,
            middle_point_vec_angle: 0.0,
            middle_point_vec_magnitude: 0.0,
        }
    }
}

//
// ---------------- Game State ----------------
//

#[derive(Clone, Debug)]
pub struct GameState {
    pub flags: u8,
}

impl Default for GameState {
    fn default() -> Self {
        Self { flags: 0 }
    }
}

impl GameState {
    pub fn is_running(&self) -> bool {
        self.flags & (1 << 0) != 0
    }

    pub fn is_goalie(&self) -> bool {
        self.flags & (1 << 1) != 0
    }

    pub fn target_is_yellow(&self) -> bool {
        self.flags & (1 << 2) != 0
    }

    pub fn switch_wanted(&self) -> bool {
        self.flags & (1 << 3) != 0
    }

    pub fn fsm_state(&self) -> u8 {
        (self.flags >> 4) & 0b11
    }
}

//
// ---------------- Peer Robot ----------------
//

#[derive(Clone, Debug)]
pub struct PeerRobot {
    pub global_x: f32,
    pub global_y: f32,
    pub heading: f32,

    pub ball_rot: f32,
    pub ball_dist: f32,

    pub flags: u8,
}

impl Default for PeerRobot {
    fn default() -> Self {
        Self {
            global_x: 0.0,
            global_y: 0.0,
            heading: 0.0,

            ball_rot: 0.0,
            ball_dist: 0.0,

            flags: 0,
        }
    }
}

impl PeerRobot {
    pub fn is_running(&self) -> bool {
        self.flags & (1 << 0) != 0
    }

    pub fn is_goalie(&self) -> bool {
        self.flags & (1 << 1) != 0
    }

    pub fn sees_line(&self) -> bool {
        self.flags & (1 << 2) != 0
    }

    pub fn ball_exists(&self) -> bool {
        self.flags & (1 << 3) != 0
    }

    pub fn switch_wanted(&self) -> bool {
        self.flags & (1 << 4) != 0
    }

    pub fn peer_alive(&self) -> bool {
        self.flags & (1 << 5) != 0
    }
}