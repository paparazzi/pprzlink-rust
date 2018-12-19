/* This file was automatically generated, do not edit */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AUTOPILOT_VERSION_DATA {
    /* version encoded as: MAJOR * 10000 + MINOR * 100 + PATCH*/
    pub version: u32,
    /* version description as string from paparazzi_version*/
    pub desc: Vec<u8>, /* unspecified */
}
/* alive/heartbeat message containing the MD5sum of the aircraft configuration */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ALIVE_DATA {
    pub md5sum: Vec<u8>, /* unspecified */
}
/* Answer to PING datalink message, to measure latencies */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct PONG_DATA {}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct TAKEOFF_DATA {
    pub cpu_time: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ARDRONE_NAVDATA_DATA {
    pub taille: u16,
    pub nu_trame: u16,
    pub ax: u16,
    pub ay: u16,
    pub az: u16,
    pub vx: i16,
    pub vy: i16,
    pub vz: i16,
    pub temperature_acc: u16,
    pub temperature_gyro: u16,
    pub ultrasound: u16,
    pub us_debut_echo: u16,
    pub us_fin_echo: u16,
    pub us_association_echo: u16,
    pub us_distance_echo: u16,
    pub us_curve_time: u16,
    pub us_curve_value: u16,
    pub us_curve_ref: u16,
    pub nb_echo: u16,
    pub sum_echo: u32,
    pub gradient: i16,
    pub flag_echo_ini: u16,
    pub pressure: i32,
    pub temperature_pressure: u16,
    pub mx: i16,
    pub my: i16,
    pub mz: i16,
    pub chksum: u16,
    pub checksum_errors: u32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ATTITUDE_DATA {
    pub phi: f32,
    pub psi: f32,
    pub theta: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct IR_SENSORS_DATA {
    pub ir1: i16,
    pub ir2: i16,
    pub longitudinal: i16,
    pub lateral: i16,
    pub vertical: i16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GPS_DATA {
    pub mode: u8,
    pub utm_east: i32,
    pub utm_north: i32,
    pub course: i16,
    /* Altitude above geoid (MSL)*/
    pub alt: i32,
    /* norm of 2d ground speed in cm/s*/
    pub speed: u16,
    pub climb: i16,
    pub week: u16,
    pub itow: u32,
    pub utm_zone: u8,
    pub gps_nb_err: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct NAVIGATION_REF_DATA {
    pub utm_east: i32,
    pub utm_north: i32,
    pub utm_zone: u8,
    pub ground_alt: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct NAVIGATION_DATA {
    pub cur_block: u8,
    pub cur_stage: u8,
    pub pos_x: f32,
    pub pos_y: f32,
    pub dist_wp: f32,
    pub dist_home: f32,
    pub circle_count: u8,
    pub oval_count: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct PPRZ_MODE_DATA {
    pub ap_mode: u8,
    pub ap_gaz: u8,
    pub ap_lateral: u8,
    pub ap_horizontal: u8,
    pub if_calib_mode: u8,
    pub mcu1_status: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct BAT_DATA {
    pub throttle: i16,
    pub voltage: u16,
    pub amps: i16,
    pub flight_time: u16,
    pub kill_auto_throttle: u8,
    pub block_time: u16,
    pub stage_time: u16,
    pub energy: i16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DEBUG_MCU_LINK_DATA {
    pub i2c_nb_err: u8,
    pub i2c_mcu1_nb_err: u8,
    pub ppm_rate: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct CALIBRATION_DATA {
    pub climb_sum_err: f32,
    pub climb_gaz_submode: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct SETTINGS_DATA {
    pub slider_1_val: f32,
    pub slider_2_val: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DESIRED_DATA {
    pub roll: f32,
    pub pitch: f32,
    pub course: f32,
    pub x: f32,
    pub y: f32,
    pub altitude: f32,
    pub climb: f32,
    pub airspeed: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GPS_SOL_DATA {
    pub Pacc: u32,
    pub Sacc: u32,
    pub PDOP: u16,
    pub numSV: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ADC_GENERIC_DATA {
    pub val1: u16,
    pub val2: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ECU_DATA {
    pub stg_in: u8,
    pub stb_in: u8,
    pub ain1: f32,
    pub ain2: f32,
    pub ain3: f32,
    pub ain4: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct CAM_DATA {
    pub pan: i16,
    pub tilt: i16,
    pub target_x: i16,
    pub target_y: i16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct CIRCLE_DATA {
    pub center_east: f32,
    pub center_north: f32,
    pub radius: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct SEGMENT_DATA {
    pub segment_east_1: f32,
    pub segment_north_1: f32,
    pub segment_east_2: f32,
    pub segment_north_2: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct VECTORNAV_INFO_DATA {
    pub timestamp: f32,
    pub chksm_error: u32,
    pub hdr_error: u32,
    pub rate: u16,
    pub ins_status: u8,
    pub ins_err: u8,
    pub YprU1: f32,
    pub YprU2: f32,
    pub YprU3: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct HYBRID_GUIDANCE_DATA {
    pub pos_x: i32,
    pub pos_y: i32,
    pub speed_x: i32,
    pub speed_y: i32,
    pub wind_x: i32,
    pub wind_y: i32,
    pub pos_err_x: i32,
    pub pos_err_y: i32,
    pub speed_sp_x: i32,
    pub speed_sp_y: i32,
    pub norm_ref_speed: i32,
    pub heading_diff: i32,
    pub phi: i32,
    pub theta: i32,
    pub psi: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct SVINFO_DATA {
    pub chn: u8,
    pub SVID: u8,
    pub Flags: u8,
    pub QI: u8,
    pub CNO: u8,
    pub Elev: i8,
    pub Azim: i16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DEBUG_DATA {
    pub msg: Vec<u8>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct SURVEY_DATA {
    pub east: f32,
    pub north: f32,
    pub west: f32,
    pub south: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct RSSI_DATA {
    pub rssi: u8,
    pub tx_power: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct RANGEFINDER_DATA {
    pub range: u16,
    pub z_dot: f32,
    pub z_dot_sum_err: f32,
    pub z_dot_setpoint: f32,
    pub z_sum_err: f32,
    pub z_setpoint: f32,
    pub flying: u8,
}
/* Datalink status reported by an aircraft for the ground */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DATALINK_REPORT_DATA {
    pub uplink_lost_time: u16,
    pub uplink_nb_msgs: u16,
    pub downlink_nb_msgs: u16,
    pub downlink_rate: u16,
    pub uplink_rate: u16,
    pub downlink_ovrn: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DL_VALUE_DATA {
    pub index: u8,
    pub value: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct MARK_DATA {
    pub ac_id: u8,
    pub lat: f32,
    pub long: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct SYS_MON_DATA {
    pub periodic_time: u16,
    pub periodic_time_min: u16,
    pub periodic_time_max: u16,
    pub periodic_cycle: u16,
    pub periodic_cycle_min: u16,
    pub periodic_cycle_max: u16,
    pub event_number: u16,
    pub cpu_load: u8,
    pub cpu_time: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct MOTOR_DATA {
    pub rpm: u16,
    pub current: i32,
}
/* Waypoint with id wp_id has been updated/moved to the specified UTM coordinates. */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct WP_MOVED_DATA {
    pub wp_id: u8,
    pub utm_east: f32,
    pub utm_north: f32,
    /* Height above Mean Sea Level (geoid)*/
    pub alt: f32,
    pub utm_zone: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct MKK_DATA {
    pub nr: u8,
    pub rpm: u8,
    pub current: u8,
    pub temp: i8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ENERGY_DATA {
    pub bat: f32,
    pub amp: f32,
    pub energy: u16,
    pub power: f32,
}
/* Velocities in body axes (assuming small pitch/roll angles) as measured by the dragspeed module and by the INS. */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DRAGSPEED_DATA {
    /* Estimated velocity along body +x axis*/
    pub u_est: f32,
    /* Estimated velocity along body +y axis*/
    pub v_est: f32,
    /* INS velocity along body +x axis*/
    pub u_ins: f32,
    /* INS velocity along body +y axis*/
    pub v_ins: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct RSSI_COMBINED_DATA {
    pub remote_rssi: u8,
    pub tx_power: u8,
    pub local_rssi: u8,
    pub local_noise: u8,
    pub remote_noise: u8,
}
/* Telemetry message for monitoring the status of the Distributed Circular Formation. */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DCF_DATA {
    /*  The size of the array is 4 x (maximum number of possible neighbors). The elements per each neighbor are: 1. ID of the neighbor, 2. Theta of the neighbor (degrees x 100), 3. Desired inter-vehicle angle (degrees x 100), 4. Last time in ms we received a msg from the neighbor*/
    pub table: Vec<i16>, /* unspecified */
    /*  The size of the array is the maximum number of possible neighbors. Errors w.r.t. desired inter-vehicle angles (degrees x 100)*/
    pub errors: Vec<i16>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ALT_KALMAN_DATA {
    pub p00: f32,
    pub p01: f32,
    pub p10: f32,
    pub p11: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ESTIMATOR_DATA {
    pub z: f32,
    pub z_dot: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct TUNE_ROLL_DATA {
    pub p: f32,
    pub phi: f32,
    pub phi_sp: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct BARO_MS5534A_DATA {
    pub pressure: u32,
    pub temp: u16,
    pub alt: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct BARO_WORDS_DATA {
    pub w1: u16,
    pub w2: u16,
    pub w3: u16,
    pub w4: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct WP_MOVED_LLA_DATA {
    pub wp_id: u8,
    pub lat: i32,
    pub lon: i32,
    /* Height above Mean Sea Level (geoid)*/
    pub alt: i32,
}
/* Relative localization data for other tracked MAVs in terms of x y and z in the body axis */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct RLFILTER_DATA {
    pub trackedID: i32,
    pub rangearray: f32,
    pub x_tracked: f32,
    pub y_tracked: f32,
    pub vx_own: f32,
    pub vy_own: f32,
    pub vx_tracked: f32,
    pub vy_tracked: f32,
    pub z_pos: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct WP_MOVED_ENU_DATA {
    pub wp_id: u8,
    pub east: i32,
    pub north: i32,
    pub up: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct WINDTURBINE_STATUS__DATA {
    pub ac_id: u8,
    pub tb_id: u8,
    pub sync_itow: u32,
    pub cycle_time: u32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct RC_3CH__DATA {
    pub throttle_mode: u8,
    pub roll: i8,
    pub pitch: i8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct MPPT_DATA {
    pub values: Vec<i16>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DEBUG_IR_I2C_DATA {
    pub ir1: i16,
    pub ir2: i16,
    pub top: i16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AIRSPEED_DATA {
    pub airspeed: f32,
    pub airspeed_sp: f32,
    pub airspeed_cnt: f32,
    pub groundspeed_sp: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct BARO_ETS_DATA {
    pub adc: u16,
    pub offset: u16,
    pub scaled: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AIRSPEED_ETS_DATA {
    pub adc: u16,
    pub offset: u16,
    pub scaled: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct VISION_OUTBACK_DATA {
    pub status: u8,
    pub het_moment: u8,
    pub timeoutcount: u8,
    pub vision_timeout: u8,
    pub height: f32,
    pub out_of_range: f32,
    pub marker_enu_x: f32,
    pub marker_enu_y: f32,
    pub flow_x: f32,
    pub flow_y: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GPS_LLA_DATA {
    pub lat: i32,
    pub lon: i32,
    /* altitude above WGS84 reference ellipsoid*/
    pub alt: i32,
    /* Height above Mean Sea Level (geoid)*/
    pub hmsl: i32,
    pub course: i16,
    pub speed: u16,
    pub climb: i16,
    pub week: u16,
    pub itow: u32,
    pub mode: u8,
    pub gps_nb_err: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct H_CTL_A_DATA {
    pub roll_sum_err: f32,
    pub roll_sp: f32,
    pub roll_ref: f32,
    pub phi: f32,
    pub aileron_sp: i16,
    pub pitch_sum_err: f32,
    pub pitch_sp: f32,
    pub pitch_ref: f32,
    pub theta: f32,
    pub elevator_sp: i16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct TURB_PRESSURE_VOLTAGE_DATA {
    pub ch_1_p: f32,
    pub ch_1_t: f32,
    pub ch_2_p: f32,
    pub ch_2_t: f32,
    pub ch_3_p: f32,
    pub ch_3_t: f32,
    pub ch_4_p: f32,
    pub ch_4_t: f32,
    pub ch_5_p: f32,
    pub ch_5_t: f32,
    pub ch_6_p: f32,
    pub ch_6_t: f32,
    pub ch_7_p: f32,
    pub ch_7_t: f32,
    pub gnd1: f32,
    pub gnd2: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct CAM_POINT_DATA {
    pub cam_point_distance_from_home: u16,
    pub cam_point_lat: f32,
    pub cam_point_lon: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DC_INFO_DATA {
    pub mode: i16,
    pub lat: i32,
    pub lon: i32,
    /* altitude above WGS84 reference ellipsoid*/
    pub alt: i32,
    pub course: f32,
    pub photo_nr: u16,
    pub dist: f32,
    pub next_dist: f32,
    pub start_x: f32,
    pub start_y: f32,
    pub start_angle: f32,
    pub angle: f32,
    pub last_block: f32,
    pub count: u16,
    pub shutter: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AMSYS_BARO_DATA {
    pub pBaroRaw: u16,
    pub pBaro: f32,
    pub pHomePressure: f32,
    pub AltOffset: f32,
    pub aktuell: f32,
    pub Over_Ground: f32,
    pub tempBaro: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AMSYS_AIRSPEED_DATA {
    pub asRaw: u16,
    pub asPresure: f32,
    pub asAirspeed: f32,
    pub asAirsFilt: f32,
    pub asTemp: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct FLIGHT_BENCHMARK_DATA {
    pub SE_As: f32,
    pub SE_Alt: f32,
    pub SE_Pos: f32,
    pub Err_As: f32,
    pub Err_Alt: f32,
    pub Err_Pos: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct MPL3115_BARO_DATA {
    pub pressure: u32,
    pub temp: i16,
    pub alt: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AOA_DATA {
    pub raw: u32,
    pub angle: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct XTEND_RSSI_DATA {
    pub datalink_time: u16,
    pub rssi_fade_margin: u8,
    pub duty: u8,
}
/* Information about the trajectory followed by the Guidance Vector Field algorithm. */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GVF_DATA {
    /* Error index e, i.e. 'distance' to the trajectory*/
    pub error: f32,
    /* Kind of trajectory*/
    pub traj: u8,
    /* Direction to be followed*/
    pub s: i8,
    /* Gain for the vector field convergence*/
    pub ke: f32,
    /* Parameters describing the trajectory*/
    pub p: Vec<f32>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct SUPERBITRF_DATA {
    pub status: u8,
    pub cyrf_status: u8,
    pub irq_count: u32,
    pub rx_packet_count: u32,
    pub tx_packet_count: u32,
    pub transfer_timeouts: u32,
    pub resync_count: u32,
    pub uplink_count: u32,
    pub rc_count: u32,
    pub timing1: u32,
    pub timing2: u32,
    pub bind_mfg_id: u32,
    pub mfg_id: Vec<u8>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GX3_INFO_DATA {
    pub GX3_freq: f32,
    pub chksm_error: u32,
    pub hdr_error: u32,
    pub GX3_chksm: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct UBLOX_INFO_DATA {
    pub baud: u32,
    pub ver_sw_h: u8,
    pub ver_sw_l: u8,
    pub ver_hw_h: u16,
    pub ver_hw_l: u16,
    pub sbas: u8,
    pub gnss: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct INV_FILTER_DATA {
    pub quat: f32,
    pub phi_inv: f32,
    pub theta_inv: f32,
    pub psi_inv: f32,
    pub Vx_inv: f32,
    pub Vy_inv: f32,
    pub Vz_inv: f32,
    pub Px_inv: f32,
    pub Py_inv: f32,
    pub Pz_inv: f32,
    pub bias_phi: f32,
    pub bias_theta: f32,
    pub bias_psi: f32,
    pub bias_as: f32,
    pub bias_hb: f32,
    pub meas_baro: f32,
    pub meas_gps: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct MISSION_STATUS_DATA {
    pub remaining_time: f32,
    pub index_list: Vec<u8>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GENERIC_COM_DATA {
    pub lat: i32,
    pub lon: i32,
    pub alt: i16,
    pub gspeed: u16,
    pub course: i16,
    pub airspeed: u16,
    pub vsupply: u8,
    pub energy: u8,
    pub throttle: u8,
    pub ap_mode: u8,
    pub nav_block: u8,
    pub flight_time: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct FORMATION_SLOT_TM_DATA {
    pub ac_id: u8,
    pub mode: u8,
    pub slot_east: f32,
    pub slot_north: f32,
    pub slot_alt: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct FORMATION_STATUS_TM_DATA {
    pub ac_id: u8,
    pub leader_id: u8,
    pub status: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct BMP_STATUS_DATA {
    pub UP: i32,
    pub UT: i32,
    pub press: i32,
    pub temp: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct MLX_STATUS_DATA {
    pub itemp_case: u16,
    pub temp_case: f32,
    pub itemp_obj: u16,
    pub temp_obj: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct TMP_STATUS_DATA {
    pub itemp: u16,
    pub temp: f32,
}
/* Wind information returned to the ground station. The wind is reported as a vector, it gives the direction the wind is blowing to. This can be used to acknowledge data comming from the ground wind estimator or from an embedded algorithm. Flags field definition: - bit 0: horizontal wind is valid (east and north fields) - bit 1: vertical wind is valid (up field) - bit 2: airspeed is valid */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct WIND_INFO_RET_DATA {
    /* bit 0: horizontal wind, bit 1: vertical wind: bit 2: airspeed*/
    pub flags: u8,
    /* east component of the wind*/
    pub east: f32,
    /* north component of the wind*/
    pub north: f32,
    /* vertical component of the wind*/
    pub up: f32,
    /* local airspeed norm*/
    pub airspeed: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct SCP_STATUS_DATA {
    pub press: u32,
    pub temp: i16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct SHT_STATUS_DATA {
    pub ihumid: u16,
    pub itemp: u16,
    pub humid: f32,
    pub temp: f32,
}
/* Generic message to send position measurement from computer vision */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct VISION_POSITION_ESTIMATE_DATA {
    /* Counter*/
    pub cnt: u16,
    /* X*/
    pub x: f32,
    /* Y*/
    pub y: f32,
    /* Z*/
    pub z: f32,
    /* Detection Quality or Uncertainty*/
    pub quality: f32,
    /* In case the default message does not suit you*/
    pub extra: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DPICCO_STATUS_DATA {
    pub humid: u16,
    pub temp: u16,
    pub fhumid: f32,
    pub ftemp: f32,
}
/* Logger status and error id (dependent of the logging system) */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct LOGGER_STATUS_DATA {
    /* General status of the logger*/
    pub status: u8,
    /* Error number, depend of the logging system, provides detailed information*/
    pub errno: u8,
    /* Accumulated number of bytes written by the logger*/
    pub used: u32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct MOTOR_BENCH_STATUS_DATA {
    pub time_ticks: u32,
    pub throttle: f32,
    pub rpm: f32,
    pub current: f32,
    pub thrust: f32,
    pub torque: f32,
    pub time_s: u16,
    pub mode: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct HIH_STATUS_DATA {
    pub humid: u16,
    pub fhumid: f32,
    pub ftemp: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct TEMT_STATUS_DATA {
    pub light: u16,
    pub f_light: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GP2Y_STATUS_DATA {
    pub idensity: u16,
    pub density: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct SHT_I2C_SERIAL_DATA {
    pub serial0: u32,
    pub serial1: u32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct PPM_DATA {
    pub ppm_rate: u8,
    pub values: Vec<u16>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct RC_DATA {
    pub values: Vec<i16>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct COMMANDS_DATA {
    pub values: Vec<i16>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct FBW_STATUS_DATA {
    pub rc_status: u8,
    pub frame_rate: u8,
    pub mode: u8,
    pub vsupply: u16,
    pub current: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ADC_DATA {
    pub mcu: u8,
    pub values: Vec<u16>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ACTUATORS_DATA {
    pub values: Vec<i16>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct BLUEGIGA_DATA {
    pub data_rate: u32,
    pub A2A_msg_rate: u32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct THROTTLE_CURVE_DATA {
    pub curve: u8,
    pub throttle: u16,
    pub collective: i16,
    pub rpm_sp: u16,
    pub rpm_meas: u16,
    pub rpm_err_sum: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct PIKSI_HEARTBEAT_DATA {
    pub heartbeat: u32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct MULTIGAZE_METERS_DATA {
    pub multigaze_meters: Vec<f32>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DC_SHOT_DATA {
    pub photo_nr: i16,
    /* Gedetic latitude*/
    pub lat: i32,
    /* Longitude*/
    pub lon: i32,
    /* altitude above WGS84 reference ellipsoid*/
    pub alt: i32,
    /* Height above Mean Sea Level (geoid)*/
    pub hmsl: i32,
    /* Euler angle around x-axis (roll)*/
    pub phi: i16,
    /* Euler angle around y-axis (pitch)*/
    pub theta: i16,
    /* Euler angle around z-axis (yaw)*/
    pub psi: i16,
    /* Course over ground (CW/north)*/
    pub course: i16,
    /* horizontal ground speed*/
    pub speed: u16,
    /* GPS time of week*/
    pub itow: u32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct CAMERA_PAYLOAD_DATA {
    /* Payload computer seconds since startup*/
    pub timestamp: f32,
    /* Percentage of used memory (RAM) of the payload computer rounded up to whole percent*/
    pub used_memory: u8,
    /* Percentage of used disk of the payload computer rounded up to whole percent*/
    pub used_disk: u8,
    /* Payload door open/close*/
    pub door_status: u8,
    /* Error codes of the payload*/
    pub error_code: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct MOTOR_MIXING_DATA {
    pub values: Vec<i16>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct MLX_SERIAL_DATA {
    pub serial0: u32,
    pub serial1: u32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct PAYLOAD_DATA {
    pub values: Vec<u8>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct HTM_STATUS_DATA {
    pub ihumid: u16,
    pub itemp: u16,
    pub humid: f32,
    pub temp: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct BARO_MS5611_DATA {
    pub d1: u32,
    pub d2: u32,
    pub pressure: f32,
    pub temp: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct MS5611_COEFF_DATA {
    pub c0: u16,
    pub c1: u16,
    pub c2: u16,
    pub c3: u16,
    pub c4: u16,
    pub c5: u16,
    pub c6: u16,
    pub c7: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ATMOSPHERE_CHARGE_DATA {
    pub t0: u16,
    pub t1: u16,
    pub t2: u16,
    pub t3: u16,
    pub t4: u16,
    pub t5: u16,
    pub t6: u16,
    pub t7: u16,
    pub t8: u16,
    pub t9: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct SOLAR_RADIATION_DATA {
    pub up_t0: u16,
    pub dn_t0: u16,
    pub up_t1: u16,
    pub dn_t1: u16,
    pub up_t2: u16,
    pub dn_t2: u16,
    pub up_t3: u16,
    pub dn_t3: u16,
    pub up_t4: u16,
    pub dn_t4: u16,
    pub up_t5: u16,
    pub dn_t5: u16,
    pub up_t6: u16,
    pub dn_t6: u16,
    pub up_t7: u16,
    pub dn_t7: u16,
    pub up_t8: u16,
    pub dn_t8: u16,
    pub up_t9: u16,
    pub dn_t9: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct TCAS_TA_DATA {
    pub ac_id: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct TCAS_RA_DATA {
    pub ac_id: u8,
    pub resolve: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct TCAS_RESOLVED_DATA {
    pub ac_id: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct TCAS_DEBUG_DATA {
    pub ac_id: u8,
    pub tau: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct POTENTIAL_DATA {
    pub east: f32,
    pub north: f32,
    pub alt: f32,
    pub speed: f32,
    pub climb: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct COPILOT_STATUS_DATA {
    /* Mission computer seconds since startup*/
    pub timestamp: f32,
    /* Percentage of used memory (RAM) of the mission computer rounded up to whole percent*/
    pub used_memory: u8,
    /* Percentage of used disk of the mission computer rounded up to whole percent*/
    pub used_disk: u8,
    /* Mission computer status*/
    pub status: u8,
    /* Error codes of the mission computer*/
    pub error_code: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct TEMP_TCOUPLE_DATA {
    pub fval0: f32,
    pub fval1: f32,
    pub fval2: f32,
    pub fval3: f32,
    pub fref0: f32,
    pub fref1: f32,
    pub fref2: f32,
    pub fref3: f32,
    pub val0: u16,
    pub val1: u16,
    pub val2: u16,
    pub val3: u16,
    pub ref0: u16,
    pub ref1: u16,
    pub ref2: u16,
    pub ref3: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct SHT_I2C_STATUS_DATA {
    pub ihumid: u16,
    pub itemp: u16,
    pub humid: f32,
    pub temp: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct CAMERA_SNAPSHOT_DATA {
    /* Unique camera ID - consists of make,model and camera index*/
    pub camera_id: u16,
    /* State of the given camera*/
    pub camera_state: u8,
    /* Snapshot number in sequence*/
    pub snapshot_image_number: u16,
    /* Flag checking whether the last snapshot was valid*/
    pub snapshot_valid: u8,
    /* Lens temperature, NaN if not measured*/
    pub lens_temp: f32,
    /* Imager sensor temperature, NaN if not measured*/
    pub array_temp: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct TIMESTAMP_DATA {
    pub timestamp: u32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct STAB_ATTITUDE_FLOAT_DATA {
    pub est_p: f32,
    pub est_q: f32,
    pub est_r: f32,
    pub est_phi: f32,
    pub est_theta: f32,
    pub est_psi: f32,
    pub ref_phi: f32,
    pub ref_theta: f32,
    pub ref_psi: f32,
    pub sum_err_phi: f32,
    pub sum_err_theta: f32,
    pub sum_err_psi: f32,
    pub delta_a_fb: f32,
    pub delta_e_fb: f32,
    pub delta_r_fb: f32,
    pub delta_a_ff: f32,
    pub delta_e_ff: f32,
    pub delta_r_ff: f32,
    pub delta_a: i32,
    pub delta_e: i32,
    pub delta_r: i32,
    pub est_p_d: f32,
    pub est_q_d: f32,
    pub est_r_d: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct IMU_GYRO_SCALED_DATA {
    pub gp: i32,
    pub gq: i32,
    pub gr: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct IMU_ACCEL_SCALED_DATA {
    pub ax: i32,
    pub ay: i32,
    pub az: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct IMU_MAG_SCALED_DATA {
    pub mx: i32,
    pub my: i32,
    pub mz: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct FILTER_DATA {
    pub phi: i32,
    pub theta: i32,
    pub psi: i32,
    pub measure_phi: i32,
    pub measure_theta: i32,
    pub measure_psi: i32,
    pub corrected_phi: i32,
    pub corrected_theta: i32,
    pub corrected_psi: i32,
    pub correction_phi: i32,
    pub correction_theta: i32,
    pub correction_psi: i32,
    pub bp: i32,
    pub bq: i32,
    pub br: i32,
    pub comp_id: u8,
}
/* Rotorcraft rate control loop. */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct RATE_LOOP_DATA {
    /* rate setpoint*/
    pub sp_p: f32,
    /* rate setpoint*/
    pub sp_q: f32,
    /* rate setpoint*/
    pub sp_r: f32,
    /* integrated quaternion error*/
    pub sumerr_p: f32,
    /* integrated quaternion error*/
    pub sumerr_q: f32,
    /* integrated quaternion error*/
    pub sumerr_r: f32,
    /* feedback command on pitch (pprz scale)*/
    pub fb_p: f32,
    /* feedback command on roll  (pprz scale)*/
    pub fb_q: f32,
    /* feedback command on yaw   (pprz scale)*/
    pub fb_r: f32,
    /* thrust command*/
    pub delta_t: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct FILTER_ALIGNER_DATA {
    pub lp_gp: i32,
    pub lp_gq: i32,
    pub lp_gr: i32,
    pub gp: i32,
    pub gq: i32,
    pub gr: i32,
    pub noise: i32,
    pub cnt: i32,
    pub status: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AIRSPEED_MS45XX_DATA {
    pub diffPress: f32,
    pub temperature: i16,
    pub airspeed: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct STAB_ATTITUDE_INT_DATA {
    pub est_p: i32,
    pub est_q: i32,
    pub est_r: i32,
    pub est_phi: i32,
    pub est_theta: i32,
    pub est_psi: i32,
    pub sp_phi: i32,
    pub sp_theta: i32,
    pub sp_psi: i32,
    pub sum_err_phi: i32,
    pub sum_err_theta: i32,
    pub sum_err_psi: i32,
    pub delta_a_fb: i32,
    pub delta_e_fb: i32,
    pub delta_r_fb: i32,
    pub delta_a_ff: i32,
    pub delta_e_ff: i32,
    pub delta_r_ff: i32,
    pub delta_a: i32,
    pub delta_e: i32,
    pub delta_r: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct STAB_ATTITUDE_REF_INT_DATA {
    pub sp_phi: i32,
    pub sp_theta: i32,
    pub sp_psi: i32,
    pub ref_phi: i32,
    pub ref_theta: i32,
    pub ref_psi: i32,
    pub ref_p: i32,
    pub ref_q: i32,
    pub ref_r: i32,
    pub ref_pd: i32,
    pub ref_qd: i32,
    pub ref_rd: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct STAB_ATTITUDE_REF_FLOAT_DATA {
    pub sp_phi: f32,
    pub sp_theta: f32,
    pub sp_psi: f32,
    pub ref_phi: f32,
    pub ref_theta: f32,
    pub ref_psi: f32,
    pub ref_p: f32,
    pub ref_q: f32,
    pub ref_r: f32,
    pub ref_pd: f32,
    pub ref_qd: f32,
    pub ref_rd: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ROTORCRAFT_CMD_DATA {
    pub cmd_roll: i32,
    pub cmd_pitch: i32,
    pub cmd_yaw: i32,
    pub cmd_thrust: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GUIDANCE_H_INT_DATA {
    pub sp_x: i32,
    pub sp_y: i32,
    pub ref_x: i32,
    pub ref_y: i32,
    pub est_x: i32,
    pub est_y: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct VERT_LOOP_DATA {
    pub z_sp: i32,
    pub zd_sp: i32,
    pub est_z: i32,
    pub est_zd: i32,
    pub est_zdd: i32,
    pub ref_z: i32,
    pub ref_zd: i32,
    pub ref_zdd: i32,
    pub adp_inv_m: i32,
    pub adp_cov: i32,
    pub adp_meas: i32,
    pub sum_err: i32,
    pub ff_cmd: i32,
    pub fb_cmd: i32,
    pub delta_t: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct HOVER_LOOP_DATA {
    pub sp_x: i32,
    pub sp_y: i32,
    pub est_x: i32,
    pub est_y: i32,
    pub est_xd: i32,
    pub est_yd: i32,
    pub est_xdd: i32,
    pub est_ydd: i32,
    pub err_x: i32,
    pub err_y: i32,
    pub err_xd: i32,
    pub err_yd: i32,
    pub err_sum_x: i32,
    pub err_sum_y: i32,
    pub cmd_x: i32,
    pub cmd_y: i32,
    pub cmd_heading: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ROTORCRAFT_FP_DATA {
    pub east: i32,
    pub north: i32,
    pub up: i32,
    pub veast: i32,
    pub vnorth: i32,
    pub vup: i32,
    pub phi: i32,
    pub theta: i32,
    pub psi: i32,
    pub carrot_east: i32,
    pub carrot_north: i32,
    pub carrot_up: i32,
    pub carrot_psi: i32,
    pub thrust: i32,
    pub flight_time: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct TEMP_ADC_DATA {
    pub temp1: f32,
    pub temp2: f32,
    pub temp3: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GUIDANCE_H_REF_INT_DATA {
    pub sp_x: i32,
    pub ref_x: i32,
    pub sp_xd: i32,
    pub ref_xd: i32,
    pub ref_xdd: i32,
    pub sp_y: i32,
    pub ref_y: i32,
    pub sp_yd: i32,
    pub ref_yd: i32,
    pub ref_ydd: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ROTORCRAFT_TUNE_HOVER_DATA {
    pub rc_roll: i16,
    pub rc_pitch: i16,
    pub rc_yaw: i16,
    pub cmd_roll: i32,
    pub cmd_pitch: i32,
    pub cmd_yaw: i32,
    pub cmd_thrust: i32,
    pub body_phi: i32,
    pub body_theta: i32,
    pub body_psi: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct INS_Z_DATA {
    pub baro_z: f32,
    pub ins_z: i32,
    pub ins_zd: i32,
    pub ins_zdd: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct PCAP01_STATUS_DATA {
    pub ihumid: u32,
    pub itemp: u32,
    pub humid: f32,
    pub temp: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GEIGER_COUNTER_DATA {
    pub tube1: u32,
    pub tube2: u32,
    pub vsupply: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct INS_REF_DATA {
    pub ecef_x0: i32,
    pub ecef_y0: i32,
    pub ecef_z0: i32,
    pub lat0: i32,
    pub lon0: i32,
    pub alt0: i32,
    pub hmsl0: i32,
    pub baro_qfe: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GPS_INT_DATA {
    pub ecef_x: i32,
    pub ecef_y: i32,
    pub ecef_z: i32,
    pub lat: i32,
    pub lon: i32,
    /* altitude above WGS84 reference ellipsoid*/
    pub alt: i32,
    /* height above mean sea level (geoid)*/
    pub hmsl: i32,
    pub ecef_xd: i32,
    pub ecef_yd: i32,
    pub ecef_zd: i32,
    pub pacc: u32,
    pub sacc: u32,
    pub tow: u32,
    pub pdop: u16,
    pub numsv: u8,
    pub fix: u8,
    pub comp_id: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AHRS_EULER_INT_DATA {
    pub imu_phi: i32,
    pub imu_theta: i32,
    pub imu_psi: i32,
    pub body_phi: i32,
    pub body_theta: i32,
    pub body_psi: i32,
    pub comp_id: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AHRS_QUAT_INT_DATA {
    pub weight: f32,
    pub imu_qi: i32,
    pub imu_qx: i32,
    pub imu_qy: i32,
    pub imu_qz: i32,
    pub body_qi: i32,
    pub body_qx: i32,
    pub body_qy: i32,
    pub body_qz: i32,
    pub comp_id: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ROTORCRAFT_NAV_STATUS_DATA {
    pub block_time: u16,
    pub stage_time: u16,
    pub dist_home: f32,
    pub dist_wp: f32,
    pub cur_block: u8,
    pub cur_stage: u8,
    pub horizontal_mode: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ROTORCRAFT_RADIO_CONTROL_DATA {
    pub roll: i16,
    pub pitch: i16,
    pub yaw: i16,
    pub throttle: i16,
    pub mode: i16,
    pub kill: i16,
    pub status: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct VFF_EXTENDED_DATA {
    pub meas_baro: f32,
    pub meas_alt: f32,
    pub z: f32,
    pub zd: f32,
    pub zdd: f32,
    pub bias: f32,
    pub offset: f32,
    pub obs_height: f32,
    pub Pzz: f32,
    pub Pzdzd: f32,
    pub Pbb: f32,
    pub Poffsetoffset: f32,
    pub Pobsobs: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct VFF_DATA {
    pub measure: f32,
    pub z: f32,
    pub zd: f32,
    pub bias: f32,
    pub Pzz: f32,
    pub Pzdzd: f32,
    pub Pbb: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GEO_MAG_DATA {
    pub Hx: f32,
    pub Hy: f32,
    pub Hz: f32,
    pub comp_id: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct HFF_DATA {
    pub x: f32,
    pub y: f32,
    pub xd: f32,
    pub yd: f32,
    pub xdd: f32,
    pub ydd: f32,
    pub xbias: f32,
    pub ybias: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct HFF_DBG_DATA {
    pub x_measure: f32,
    pub y_measure: f32,
    pub xd_measure: f32,
    pub yd_measure: f32,
    pub Pxx: f32,
    pub Pyy: f32,
    pub Pxdxd: f32,
    pub Pydyd: f32,
    pub Pxbiasxbias: f32,
    pub Pybiasybias: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct HFF_GPS_DATA {
    pub lag_cnt: u16,
    pub lag_cnt_err: i16,
    pub save_cnt: i16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ROTORCRAFT_CAM_DATA {
    pub tilt: i16,
    pub pan: i16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AHRS_REF_QUAT_DATA {
    pub ref_qi: i32,
    pub ref_qx: i32,
    pub ref_qy: i32,
    pub ref_qz: i32,
    pub body_qi: i32,
    pub body_qx: i32,
    pub body_qy: i32,
    pub body_qz: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AHRS_EULER_DATA {
    pub phi: f32,
    pub theta: f32,
    pub psi: f32,
    pub comp_id: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AHRS_MEASUREMENT_EULER_DATA {
    pub phi: f32,
    pub theta: f32,
    pub psi: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct WT_DATA {
    pub rpm: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct CSC_CAN_DEBUG_DATA {
    pub err_nb: u32,
    pub err_code: u32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct CSC_CAN_MSG_DATA {
    pub frame: u32,
    pub id: u32,
    pub data_a: u32,
    pub data_b: u32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AHRS_GYRO_BIAS_INT_DATA {
    pub bp: i32,
    pub bq: i32,
    pub br: i32,
    pub comp_id: u8,
}
/* Airflow data returned by OTF and uADC 3D probes from Aeroprobe. */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AEROPROBE_DATA {
    pub counter: u32,
    pub velocity: i16,
    pub a_attack: i16,
    pub a_sideslip: i16,
    pub altitude: i32,
    pub dynamic_p: i32,
    pub static_p: i32,
    pub checksum: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct FMS_TIME_DATA {
    pub tv_sec: u32,
    pub tv_nsec: u32,
    pub delay_ns: u32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AHRS_LKF_DATA {
    pub phi: f32,
    pub theta: f32,
    pub psi: f32,
    pub qi: f32,
    pub qx: f32,
    pub qy: f32,
    pub qz: f32,
    pub p: f32,
    pub q: f32,
    pub r: f32,
    pub ax: f32,
    pub ay: f32,
    pub az: f32,
    pub mx: f32,
    pub my: f32,
    pub mz: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct NPS_SENSORS_SCALED_DATA {
    pub acc_x: f32,
    pub acc_y: f32,
    pub acc_z: f32,
    pub mag_x: f32,
    pub mag_y: f32,
    pub mag_z: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct INS_DATA {
    pub ins_x: i32,
    pub ins_y: i32,
    pub ins_z: i32,
    pub ins_xd: i32,
    pub ins_yd: i32,
    pub ins_zd: i32,
    pub ins_xdd: i32,
    pub ins_ydd: i32,
    pub ins_zdd: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct IMU_GYRO_DATA {
    pub gp: f32,
    pub gq: f32,
    pub gr: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct IMU_MAG_DATA {
    pub mx: f32,
    pub my: f32,
    pub mz: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct IMU_ACCEL_DATA {
    pub ax: f32,
    pub ay: f32,
    pub az: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct IMU_GYRO_RAW_DATA {
    pub gp: i32,
    pub gq: i32,
    pub gr: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct IMU_ACCEL_RAW_DATA {
    pub ax: i32,
    pub ay: i32,
    pub az: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct IMU_MAG_RAW_DATA {
    pub mx: i32,
    pub my: i32,
    pub mz: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct IMU_MAG_SETTINGS_DATA {
    pub inclination: f32,
    pub declination: f32,
    pub hardiron_x: f32,
    pub hardiron_y: f32,
    pub hardiron_z: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct IMU_MAG_CURRENT_CALIBRATION_DATA {
    pub mx: i32,
    pub my: i32,
    pub mz: i32,
    pub electrical_current: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct UART_ERRORS_DATA {
    pub overrun_cnt: u16,
    pub noise_err_cnt: u16,
    pub framing_err_cnt: u16,
    pub bus_number: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct IMU_GYRO_LP_DATA {
    pub gp: f32,
    pub gq: f32,
    pub gr: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct IMU_PRESSURE_DATA {
    pub p: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct TUNE_VERT_DATA {
    pub z_sp: i32,
    pub est_z: i32,
    pub ref_z: i32,
    pub ref_zd: i32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct MF_DAQ_STATE_DATA {
    pub flight_time: u16,
    pub p: f32,
    pub q: f32,
    pub r: f32,
    pub phi: f32,
    pub theta: f32,
    pub psi: f32,
    pub ax: f32,
    pub ay: f32,
    pub az: f32,
    pub ve: f32,
    pub vn: f32,
    pub vu: f32,
    pub lat: f32,
    pub lon: f32,
    pub alt: f32,
    pub we: f32,
    pub wn: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct INFO_MSG_DATA {
    pub msg: Vec<u8>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct STAB_ATTITUDE_INDI_DATA {
    pub angular_accel_p: f32,
    pub angular_accel_q: f32,
    pub angular_accel_r: f32,
    pub angular_accel_ref_p: f32,
    pub angular_accel_ref_q: f32,
    pub angular_accel_ref_r: f32,
    pub g1_p: f32,
    pub g1_q: f32,
    pub g1_r: f32,
    pub g2_r: f32,
}
/* Minimalistic message to track Rotorcraft over very low bandwidth links */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ROTORCRAFT_FP_MIN_DATA {
    pub east: i32,
    pub north: i32,
    pub up: i32,
    pub gspeed: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct BEBOP_ACTUATORS_DATA {
    pub cmd_thrust: i32,
    pub cmd_roll: i32,
    pub cmd_pitch: i32,
    pub cmd_yaw: i32,
    pub rpm_ref_lf: u16,
    pub rpm_ref_rf: u16,
    pub rpm_ref_rb: u16,
    pub rpm_ref_lb: u16,
    pub rpm_obs_lf: u16,
    pub rpm_obs_rf: u16,
    pub rpm_obs_rb: u16,
    pub rpm_obs_lb: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct WEATHER_DATA {
    pub p_amb: f32,
    pub t_amb: f32,
    pub windspeed: f32,
    pub wind_from: f32,
    pub humidity: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct IMU_TURNTABLE_DATA {
    pub omega: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct BARO_RAW_DATA {
    pub abs: f32,
    pub diff: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AIR_DATA_DATA {
    /* static pressure*/
    pub pressure: f32,
    /* differential pressure*/
    pub diff_p: f32,
    /* air temperature*/
    pub temp: f32,
    /* barometric pressure adjusted to sea level*/
    pub qnh: f32,
    /* barometric altitude above mean sea level*/
    pub amsl_baro: f32,
    /* Equivalent Air Speed (or Calibrated Air Speed at low speed/altitude)*/
    pub airspeed: f32,
    /* True Air Speed (when P, T and P_diff are available)*/
    pub tas: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct AMSL_DATA {
    pub AMSL_BARO: f32,
    pub AMSL_GPS: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DIVERGENCE_DATA {
    /*  vertical velocity / height from optitrack (unit = 1/sec)*/
    pub divergence: f32,
    /*  vertical velocity / height from vision (unit = 1/sec)*/
    pub divergence_vision: f32,
    /*  thrust / max thrust paparazzi (-)*/
    pub normalized_thrust: f32,
    /*  covariance of divergence and thrust, or past divergence depending on the mode (-)*/
    pub cov_div: f32,
    /*  gain state in adaptive gain control: indicative of height (-) */
    pub pstate: f32,
    /*  gain used for control, includes the effect of the p-gain of adaptive control (-) */
    pub pused: f32,
    /*  measurement from the sonar (mm)*/
    pub sonar: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct VIDEO_SYNC_DATA {
    pub id: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct PERIODIC_TELEMETRY_ERR_DATA {
    pub process: u8,
    pub mode: u8,
    pub id: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct TIME_DATA {
    /* an integral value representing the number of seconds elapsed since 00:00 hours, Jan 1, 1970 UTC*/
    pub t: u32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct OPTIC_FLOW_EST_DATA {
    pub fps: f32,
    pub corner_cnt: u16,
    pub tracked_cnt: u16,
    pub flow_x: i16,
    pub flow_y: i16,
    pub flow_der_x: i16,
    pub flow_der_y: i16,
    pub vel_x: f32,
    pub vel_y: f32,
    pub vel_z: f32,
    pub div_size: f32,
    pub surface_roughness: f32,
    pub divergence: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct STEREO_IMG_DATA {
    pub fieldtype: u8,
    pub width: u8,
    pub height: u8,
    pub package_nb: u8,
    pub image_data: Vec<u8>, /* unspecified */
}
/* Rover status message */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ROVER_STATUS_DATA {
    pub rc_status: u8,
    pub frame_rate: u8,
    pub gps_status: u8,
    /* Rover AP modes are generated from XML file*/
    pub ap_mode: u8,
    pub ap_motors_on: u8,
    pub vsupply: u16,
    pub cpu_time: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ROTORCRAFT_STATUS_DATA {
    pub link_imu_nb_err: u32,
    pub motor_nb_err: u8,
    pub rc_status: u8,
    pub frame_rate: u8,
    pub gps_status: u8,
    pub ap_mode: u8,
    pub ap_in_flight: u8,
    pub ap_motors_on: u8,
    pub arming_status: u8,
    pub ap_h_mode: u8,
    pub ap_v_mode: u8,
    pub vsupply: u16,
    pub cpu_time: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct STATE_FILTER_STATUS_DATA {
    pub id: u8,
    pub state_filter_mode: u8,
    pub value: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct PX4FLOW_DATA {
    pub time_sec: f32,
    pub sensor_id: u8,
    pub flow_x: i16,
    pub flow_y: i16,
    pub flow_comp_m_x: f32,
    pub flow_comp_m_y: f32,
    pub quality: u8,
    pub ground_distance: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct LIDAR_DATA {
    pub distance: f32,
    pub status: u8,
    pub trans_status: u8,
}
/* Generic message with pixel coordinates of detected targets */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct VISUALTARGET_DATA {
    /* Counter*/
    pub cnt: u16,
    /* Center pixel X*/
    pub x: i16,
    /* Center pixel X*/
    pub y: i16,
    /* Width in pixels*/
    pub w: i16,
    /* height in pixels*/
    pub h: i16,
    /* In case many detectors run, which one did find this target*/
    pub source: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct SONAR_DATA {
    pub sonar_meas: u16,
    pub sonar_distance: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct PAYLOAD_FLOAT_DATA {
    pub values: Vec<f32>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct NPS_POS_LLH_DATA {
    pub pprz_lat: f32,
    pub lat_geod: f32,
    pub lat_geoc: f32,
    pub pprz_lon: f32,
    pub lon: f32,
    pub pprz_alt: f32,
    pub alt_geod: f32,
    pub agl: f32,
    pub asl: f32,
}
/* Message for key exchange during crypto initialization */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct KEY_EXCHANGE_UAV_DATA {
    pub msg_type: u8,
    pub msg_data: Vec<u8>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct NPS_SPEED_POS_DATA {
    pub ltpp_xdd: f32,
    pub ltpp_ydd: f32,
    pub ltpp_zdd: f32,
    pub ltpp_xd: f32,
    pub ltpp_yd: f32,
    pub ltpp_zd: f32,
    pub ltpp_x: f32,
    pub ltpp_y: f32,
    pub ltpp_z: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct NPS_RATE_ATTITUDE_DATA {
    pub p: f32,
    pub q: f32,
    pub r: f32,
    pub phi: f32,
    pub theta: f32,
    pub psi: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct NPS_GYRO_BIAS_DATA {
    pub bp: f32,
    pub bq: f32,
    pub br: f32,
}
/* This messages includes the messages send by the Optical Flow Hover module, providing data for all three axes. */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct OPTICAL_FLOW_HOVER_DATA {
    /*  Filtered Horizontal velocity in X / height used in Optical Flow Hover*/
    pub flowX: f32,
    /*  Filtered Horizontal velocity in X / height used in Optical Flow Hover*/
    pub flowY: f32,
    /*  Filtered vertical velocity / height used in Optical Flow Hover*/
    pub Divergence: f32,
    /*  covariance of Ventral flow in X and past ventral flow in X*/
    pub cov_flowX: f32,
    /*  covariance of Ventral flow in Y and past ventral flow in Y*/
    pub cov_flowY: f32,
    /*  covariance of divergence and past divergence or thrust history*/
    pub cov_divZ: f32,
    /*  Gain used in the X feedback*/
    pub pusedX: f32,
    /*  Gain used in the Y feedback*/
    pub pusedY: f32,
    /*  Gain used in the Z feedback*/
    pub pusedZ: f32,
    /*  Sum of errors in X*/
    pub sum_errX: f32,
    /*  Sum of errors in Y*/
    pub sum_errY: f32,
    /*  Sum of errors in Z*/
    pub sum_errZ: f32,
    /*  The thrust*/
    pub thrust: i32,
    /*  The desired phi angle in degrees*/
    pub phi_des: f32,
    /*  The desired theta angle in degrees*/
    pub theta_des: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct NPS_WIND_DATA {
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
}
/* Electronic Speed Controller data */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ESC_DATA {
    /* Current consumption*/
    pub amps: f32,
    /* Input battery voltage*/
    pub bat_volts: f32,
    /* Electrical power*/
    pub power: f32,
    /* Motor rotation speed*/
    pub rpm: f32,
    /* Motor voltage*/
    pub motor_volts: f32,
    /* Accumulated consumed energy*/
    pub energy: f32,
    /* Motor ID*/
    pub motor_id: u8,
}
/* RTOS monitoring */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct RTOS_MON_DATA {
    /* Number of running threads*/
    pub nb_threads: u8,
    /* Global CPU load*/
    pub cpu_load: u8,
    /* Core free memory*/
    pub core_free: u32,
    /* Total fragmented free memory in the heap*/
    pub heap_free: u32,
    /* Number of fragments in the heap*/
    pub heap_frags: u32,
    /* Largest free block in the heap*/
    pub heap_largest: u32,
    pub cpu_time: f32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct PPRZ_DEBUG_DATA {
    pub module: u8,
    pub errno: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct BATTERY_MONITOR_DATA {
    /* Battery monitor version*/
    pub version: u8,
    /* Bus monitor status*/
    pub bus_dev_stat: u8,
    /* Bus monitor i2c transaction status*/
    pub bus_trans_stat: u8,
    /* Raw bus current reading*/
    pub current_raw: u16,
    /* Bus current*/
    pub current: f32,
    /* Bus voltage*/
    pub bus_voltage: u16,
    /* Raw temperature reading*/
    pub bus_temp_raw: Vec<u16>, /* unspecified */
    /* Battery temperatures*/
    pub bus_temp: Vec<f32>, /* unspecified */
    /* Cell bank 1 monitor i2c transaction status*/
    pub bal1_trans_stat: u8,
    /* Cell bank 1 voltage*/
    pub bal1_cells: Vec<u16>, /* unspecified */
    /* Cell bank 2 monitor i2c transaction status*/
    pub bal2_trans_stat: u8,
    /* Cell bank 2 voltage*/
    pub bal2_cells: Vec<u16>, /* unspecified */
    /* Safety plug voltage*/
    pub safety_plug: u16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GPS_RXMRTCM_DATA {
    pub Msg1005: u32,
    pub Msg1077: u32,
    pub Msg1087: u32,
    pub Crc1005: u32,
    pub Crc1077: u32,
    pub Crc1087: u32,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct INDI_G_DATA {
    pub G1_roll: Vec<f32>, /* unspecified */
    pub G1_pitch: Vec<f32>, /* unspecified */
    pub G1_yaw: Vec<f32>, /* unspecified */
    pub G1_thrust: Vec<f32>, /* unspecified */
    pub G2: Vec<f32>, /* unspecified */
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GPS_RTK_DATA {
    pub iTOW: u32,
    pub refStationId: u16,
    pub relPosN: i32,
    pub relPosE: i32,
    pub relPosD: i32,
    pub relPosHPN: i8,
    pub relPosHPE: i8,
    pub relPosHPD: i8,
    pub accN: u32,
    pub accE: u32,
    pub accD: u32,
    pub carrSoln: u8,
    pub relPosValid: u8,
    pub diffSoln: u8,
    pub gnssFixOK: u8,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GPS_SMALL_DATA {
    /* bits 31-21 course in decideg : bits 20-10 ground speed in cm/s : bits 9-0 climb speed in cm/s*/
    pub multiplex_speed: u32,
    pub lat: i32,
    pub lon: i32,
    /* height above the ellipsoid*/
    pub alt: i16,
}
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct I2C_ERRORS_DATA {
    pub wd_reset_cnt: u16,
    pub queue_full_cnt: u16,
    pub acknowledge_failure_cnt: u16,
    pub misplaced_start_or_stop_cnt: u16,
    pub arbitration_lost_cnt: u16,
    pub overrun_or_underrun_cnt: u16,
    pub pec_error_in_reception_cnt: u16,
    pub timeout_or_tlow_error_cnt: u16,
    pub smbus_alert_cnt: u16,
    pub unexpected_event_cnt: u16,
    pub last_unexpected_event: u32,
    pub bus_number: u8,
}
/* Air-to-air message for the Distributed Circular Formation algorithm. It transmits the ac's theta to its neighbor */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct DCF_THETA_DATA {
    pub theta: f32,
}
/* Message for monitoring key exchange status */
#[derive(Clone, PartialEq, Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct SECURE_LINK_STATUS_DATA {
    pub sts_stage: u8,
    pub sts_error: u8,
    pub counter_err: u32,
    pub decrypt_err: u32,
    pub encrypt_err: u32,
}
