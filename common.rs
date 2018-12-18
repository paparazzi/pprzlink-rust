/* This file was automatically generated, do not edit */
pub mod datalink {
    pub enum PprzMessageDatalink {
        ACINFO(ACINFO_DATA),
        MOVE_WP(MOVE_WP_DATA),
        WIND_INFO(WIND_INFO_DATA),
        SETTING(SETTING_DATA),
        BLOCK(BLOCK_DATA),
        HITL_UBX(HITL_UBX_DATA),
        HITL_INFRARED(HITL_INFRARED_DATA),
        PING(PING_DATA),
        FORMATION_SLOT(FORMATION_SLOT_DATA),
        FORMATION_STATUS(FORMATION_STATUS_DATA),
        JOYSTICK_RAW(JOYSTICK_RAW_DATA),
        COMMANDS_RAW(COMMANDS_RAW_DATA),
        DGPS_RAW(DGPS_RAW_DATA),
        ACINFO_LLA(ACINFO_LLA_DATA),
        DESIRED_SETPOINT(DESIRED_SETPOINT_DATA),
        GET_SETTING(GET_SETTING_DATA),
        TCAS_RESOLVE(TCAS_RESOLVE_DATA),
        MISSION_GOTO_WP(MISSION_GOTO_WP_DATA),
        MISSION_GOTO_WP_LLA(MISSION_GOTO_WP_LLA_DATA),
        MISSION_CIRCLE(MISSION_CIRCLE_DATA),
        MISSION_CIRCLE_LLA(MISSION_CIRCLE_LLA_DATA),
        MISSION_SEGMENT(MISSION_SEGMENT_DATA),
        MISSION_SEGMENT_LLA(MISSION_SEGMENT_LLA_DATA),
        MISSION_PATH(MISSION_PATH_DATA),
        MISSION_PATH_LLA(MISSION_PATH_LLA_DATA),
        MISSION_CUSTOM(MISSION_CUSTOM_DATA),
        GOTO_MISSION(GOTO_MISSION_DATA),
        NEXT_MISSION(NEXT_MISSION_DATA),
        END_MISSION(END_MISSION_DATA),
        COPILOT_STATUS_DL(COPILOT_STATUS_DL_DATA),
        CAMERA_PAYLOAD_DL(CAMERA_PAYLOAD_DL_DATA),
        CAMERA_SNAPSHOT_DL(CAMERA_SNAPSHOT_DL_DATA),
        GUIDED_SETPOINT_NED(GUIDED_SETPOINT_NED_DATA),
        WINDTURBINE_STATUS(WINDTURBINE_STATUS_DATA),
        RC_3CH(RC_3CH_DATA),
        RC_4CH(RC_4CH_DATA),
        RC_5CH(RC_5CH_DATA),
        REMOTE_GPS_SMALL(REMOTE_GPS_SMALL_DATA),
        REMOTE_GPS(REMOTE_GPS_DATA),
        REMOTE_GPS_LOCAL(REMOTE_GPS_LOCAL_DATA),
        KITE_COMMAND(KITE_COMMAND_DATA),
        PAYLOAD_COMMAND(PAYLOAD_COMMAND_DATA),
        SET_ACTUATOR(SET_ACTUATOR_DATA),
        CSC_SERVO_CMD(CSC_SERVO_CMD_DATA),
        BOOZ2_FMS_COMMAND(BOOZ2_FMS_COMMAND_DATA),
        BOOZ_NAV_STICK(BOOZ_NAV_STICK_DATA),
        EXTERNAL_FILTER_SOLUTION(EXTERNAL_FILTER_SOLUTION_DATA),
        ROTORCRAFT_CAM_STICK(ROTORCRAFT_CAM_STICK_DATA),
        GPS_INJECT(GPS_INJECT_DATA),
        EXTERNAL_MAG_RAW(EXTERNAL_MAG_RAW_DATA),
        VIDEO_ROI(VIDEO_ROI_DATA),
        EMERGENCY_CMD(EMERGENCY_CMD_DATA),
        RTCM_INJECT(RTCM_INJECT_DATA),
        DCF_REG_TABLE(DCF_REG_TABLE_DATA),
        KEY_EXCHANGE_GCS(KEY_EXCHANGE_GCS_DATA),
    }
    impl PprzMessageDatalink {
        pub fn message_id(&self) -> u8 {
            use self::PprzMessageDatalink::*;
            match self {
                ACINFO(..) => 1,
                MOVE_WP(..) => 2,
                WIND_INFO(..) => 3,
                SETTING(..) => 4,
                BLOCK(..) => 5,
                HITL_UBX(..) => 6,
                HITL_INFRARED(..) => 7,
                PING(..) => 8,
                FORMATION_SLOT(..) => 9,
                FORMATION_STATUS(..) => 10,
                JOYSTICK_RAW(..) => 11,
                COMMANDS_RAW(..) => 12,
                DGPS_RAW(..) => 13,
                ACINFO_LLA(..) => 14,
                DESIRED_SETPOINT(..) => 15,
                GET_SETTING(..) => 16,
                TCAS_RESOLVE(..) => 17,
                MISSION_GOTO_WP(..) => 20,
                MISSION_GOTO_WP_LLA(..) => 21,
                MISSION_CIRCLE(..) => 22,
                MISSION_CIRCLE_LLA(..) => 23,
                MISSION_SEGMENT(..) => 24,
                MISSION_SEGMENT_LLA(..) => 25,
                MISSION_PATH(..) => 26,
                MISSION_PATH_LLA(..) => 27,
                MISSION_CUSTOM(..) => 28,
                GOTO_MISSION(..) => 30,
                NEXT_MISSION(..) => 31,
                END_MISSION(..) => 32,
                COPILOT_STATUS_DL(..) => 33,
                CAMERA_PAYLOAD_DL(..) => 34,
                CAMERA_SNAPSHOT_DL(..) => 35,
                GUIDED_SETPOINT_NED(..) => 40,
                WINDTURBINE_STATUS(..) => 50,
                RC_3CH(..) => 51,
                RC_4CH(..) => 52,
                RC_5CH(..) => 53,
                REMOTE_GPS_SMALL(..) => 54,
                REMOTE_GPS(..) => 55,
                REMOTE_GPS_LOCAL(..) => 56,
                KITE_COMMAND(..) => 96,
                PAYLOAD_COMMAND(..) => 97,
                SET_ACTUATOR(..) => 100,
                CSC_SERVO_CMD(..) => 101,
                BOOZ2_FMS_COMMAND(..) => 149,
                BOOZ_NAV_STICK(..) => 150,
                EXTERNAL_FILTER_SOLUTION(..) => 151,
                ROTORCRAFT_CAM_STICK(..) => 152,
                GPS_INJECT(..) => 153,
                EXTERNAL_MAG_RAW(..) => 154,
                VIDEO_ROI(..) => 155,
                EMERGENCY_CMD(..) => 156,
                RTCM_INJECT(..) => 157,
                DCF_REG_TABLE(..) => 158,
                KEY_EXCHANGE_GCS(..) => 159,
            }
        }
    } /* This file was automatically generated, do not edit */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ACINFO_DATA {
        course: i16,
        utm_east: i32,
        utm_north: i32,
        utm_zone: u8,
        /* Height above Mean Sea Level (geoid)*/
        alt: i32,
        itow: u32,
        /* ground speed*/
        speed: u16,
        climb: i16,
        ac_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MOVE_WP_DATA {
        wp_id: u8,
        ac_id: u8,
        lat: i32,
        lon: i32,
        /* Height above Mean Sea Level (geoid)*/
        alt: i32,
    }
    /* Wind information. The wind is reported as a vector, it gives the direction the wind is blowing to. This can be comming from the ground wind estimator or from an embedded algorithm. Flags field definition: - bit 0: horizontal wind is valid (east and north fields) - bit 1: vertical wind is valid (up field) - bit 2: airspeed is valid */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct WIND_INFO_DATA {
        ac_id: u8,
        /* bit 0: horizontal wind, bit 1: vertical wind: bit 2: airspeed*/
        flags: u8,
        /* east component of the wind*/
        east: f32,
        /* north component of the wind*/
        north: f32,
        /* vertical component of the wind*/
        up: f32,
        /* local airspeed norm*/
        airspeed: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SETTING_DATA {
        index: u8,
        ac_id: u8,
        value: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct BLOCK_DATA {
        block_id: u8,
        ac_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct HITL_UBX_DATA {
        class: u8,
        id: u8,
        ac_id: u8,
        ubx_payload: Vec<u8>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct HITL_INFRARED_DATA {
        roll: i16,
        pitch: i16,
        top: i16,
        ac_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct PING_DATA {}
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct FORMATION_SLOT_DATA {
        ac_id: u8,
        mode: u8,
        slot_east: f32,
        slot_north: f32,
        slot_alt: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct FORMATION_STATUS_DATA {
        ac_id: u8,
        leader_id: u8,
        status: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct JOYSTICK_RAW_DATA {
        ac_id: u8,
        roll: i8,
        pitch: i8,
        throttle: i8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct COMMANDS_RAW_DATA {
        ac_id: u8,
        commands: Vec<i8>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DGPS_RAW_DATA {
        ac_id: u8,
        length: u8,
        rtcm: Vec<u8>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ACINFO_LLA_DATA {
        course: i16,
        lat: i32,
        lon: i32,
        /* Height above ellipsoid*/
        alt: i32,
        itow: u32,
        /* ground speed*/
        speed: u16,
        climb: i16,
        ac_id: u8,
    }
    /* This message is used to set 3D desired vehicle's states such as accelerations or velocities. The 'flag' field can be used at the user convenience to provide indication about the type of value to track (like position, speed, acceleration, ...) or the reference frame (ENU / NED, LTP / body frame, ...) */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DESIRED_SETPOINT_DATA {
        /* AC_ID of the vehicle*/
        ac_id: u8,
        /* Up to the user, for example to distinguish between ENU or NED*/
        flag: u8,
        /* Quantity to be tracked in the X axis*/
        ux: f32,
        /* Quantity to be tracked in the Y axis*/
        uy: f32,
        /* Quantity to be tracked in the Z axis*/
        uz: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GET_SETTING_DATA {
        index: u8,
        ac_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TCAS_RESOLVE_DATA {
        ac_id: u8,
        ac_id_conflict: u8,
        resolve: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MISSION_GOTO_WP_DATA {
        ac_id: u8,
        insert: u8,
        wp_east: f32,
        wp_north: f32,
        /* altitude above geoid (MSL)*/
        wp_alt: f32,
        duration: f32,
        index: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MISSION_GOTO_WP_LLA_DATA {
        ac_id: u8,
        insert: u8,
        wp_lat: i32,
        wp_lon: i32,
        /* altitude above geoid (MSL)*/
        wp_alt: i32,
        duration: f32,
        index: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MISSION_CIRCLE_DATA {
        ac_id: u8,
        insert: u8,
        center_east: f32,
        center_north: f32,
        /* altitude above geoid (MSL)*/
        center_alt: f32,
        radius: f32,
        duration: f32,
        index: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MISSION_CIRCLE_LLA_DATA {
        ac_id: u8,
        insert: u8,
        center_lat: i32,
        center_lon: i32,
        /* altitude above geoid (MSL)*/
        center_alt: i32,
        radius: f32,
        duration: f32,
        index: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MISSION_SEGMENT_DATA {
        ac_id: u8,
        insert: u8,
        segment_east_1: f32,
        segment_north_1: f32,
        segment_east_2: f32,
        segment_north_2: f32,
        /* altitude above geoid (MSL)*/
        segment_alt: f32,
        duration: f32,
        index: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MISSION_SEGMENT_LLA_DATA {
        ac_id: u8,
        insert: u8,
        segment_lat_1: i32,
        segment_lon_1: i32,
        segment_lat_2: i32,
        segment_lon_2: i32,
        /* altitude above geoid (MSL)*/
        segment_alt: i32,
        duration: f32,
        index: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MISSION_PATH_DATA {
        ac_id: u8,
        insert: u8,
        point_east_1: f32,
        point_north_1: f32,
        point_east_2: f32,
        point_north_2: f32,
        point_east_3: f32,
        point_north_3: f32,
        point_east_4: f32,
        point_north_4: f32,
        point_east_5: f32,
        point_north_5: f32,
        /* altitude above geoid (MSL)*/
        path_alt: f32,
        duration: f32,
        nb: u8,
        index: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MISSION_PATH_LLA_DATA {
        ac_id: u8,
        insert: u8,
        point_lat_1: i32,
        point_lon_1: i32,
        point_lat_2: i32,
        point_lon_2: i32,
        point_lat_3: i32,
        point_lon_3: i32,
        point_lat_4: i32,
        point_lon_4: i32,
        point_lat_5: i32,
        point_lon_5: i32,
        /* altitude above geoid (MSL)*/
        path_alt: i32,
        duration: f32,
        nb: u8,
        index: u8,
    }
    /* Custom navigation pattern or action for mission controller. This will add the mission element correspond to the string identifier 'type' if it has been registered. */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MISSION_CUSTOM_DATA {
        /* Aircraft ID*/
        ac_id: u8,
        /* Insertion mode*/
        insert: u8,
        /* Mission element index, should be unique*/
        index: u8,
        /* String identifier of the custom pattern*/
        fieldtype: Vec<u8>, /* 5 */
        /* Duration of the element or -1 for unlimited*/
        duration: f32,
        /* List of parameters, 12 max*/
        params: Vec<f32>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GOTO_MISSION_DATA {
        ac_id: u8,
        mission_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct NEXT_MISSION_DATA {
        ac_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct END_MISSION_DATA {
        ac_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct COPILOT_STATUS_DL_DATA {
        ac_id: u8,
        /* Percentage of used memory (RAM) of the mission computer rounded up to whole percent*/
        used_memory: u8,
        /* Mission computer seconds since startup*/
        timestamp: f32,
        /* Percentage of used disk of the mission computer rounded up to whole percent*/
        used_disk: u8,
        /* Mission computer status*/
        status: u8,
        /* Error codes of the mission computer*/
        error_code: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct CAMERA_PAYLOAD_DL_DATA {
        ac_id: u8,
        /* Error codes of the payload*/
        error_code: u8,
        /* Payload computer seconds sice startup*/
        timestamp: f32,
        /* Percentage of used memory (RAM) of the payload computer rounded up to whole percent*/
        used_memory: u8,
        /* Percentage of used disk of the payload computer rounded up to whole percent*/
        used_disk: u8,
        /* Payload door open/close*/
        door_status: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct CAMERA_SNAPSHOT_DL_DATA {
        /* Unique camera ID - consists of make,model and camera index*/
        camera_id: u16,
        /* Snapshot number in sequence*/
        snapshot_image_number: u16,
        /* State of the given camera*/
        camera_state: u8,
        /* Flag checking whether the last snapshot was valid*/
        snapshot_valid: u8,
        /* Imager sensor temperature, NaN if not measured*/
        array_temp: f32,
        /* Lens temperature, NaN if not measured*/
        lens_temp: f32,
        ac_id: u8,
    }
    /* Set vehicle position or velocity in NED. Frame can be specified with the bits 0-3 Velocity of position setpoint can be specified with the bits 5-7 Flags field definition: - bit 0: x,y as offset coordinates - bit 1: x,y in body coordinates - bit 2: z as offset coordinates - bit 3: yaw as offset coordinates - bit 4: free - bit 5: x,y as vel - bit 6: z as vel - bit 7: yaw as rate */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GUIDED_SETPOINT_NED_DATA {
        ac_id: u8,
        /* bits 0-3: frame, bits 5-7: use as velocity*/
        flags: u8,
        /* X position/velocity in NED*/
        x: f32,
        /* Y position/velocity in NED*/
        y: f32,
        /* Z position/velocity in NED (negative altitude)*/
        z: f32,
        /* yaw/rate setpoint*/
        yaw: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct WINDTURBINE_STATUS_DATA {
        ac_id: u8,
        tb_id: u8,
        sync_itow: u32,
        cycle_time: u32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct RC_3CH_DATA {
        throttle_mode: u8,
        roll: i8,
        pitch: i8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct RC_4CH_DATA {
        ac_id: u8,
        mode: u8,
        throttle: u8,
        roll: i8,
        pitch: i8,
        yaw: i8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct RC_5CH_DATA {
        ac_id: u8,
        throttle: u8,
        roll: i8,
        pitch: i8,
        yaw: i8,
        mode: i8,
        kill: i8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct REMOTE_GPS_SMALL_DATA {
        heading: i16,
        /* bits 31-21 east position [cm] : bits 20-10 north position [cm] : bits 9-0 up position [cm]*/
        pos_xyz: u32,
        /* bits 31-21 east speed [cm/s] : bits 20-10 north speed [cm/s] : bits 9-0 climb speed [cm/s]*/
        speed_xyz: u32,
        tow: u32,
        ac_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct REMOTE_GPS_DATA {
        ac_id: u8,
        numsv: u8,
        ecef_x: i32,
        ecef_y: i32,
        ecef_z: i32,
        lat: i32,
        lon: i32,
        /* Height above WGS84 reference ellipsoid*/
        alt: i32,
        /* Height above Mean Sea Level (geoid)*/
        hmsl: i32,
        ecef_xd: i32,
        ecef_yd: i32,
        ecef_zd: i32,
        tow: u32,
        course: i32,
    }
    /* Position and speed in local frame from a remote GPS or motion capture system Global position transformations are handled onboard if needed */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct REMOTE_GPS_LOCAL_DATA {
        ac_id: u8,
        pad: u8,
        enu_x: f32,
        enu_y: f32,
        enu_z: f32,
        enu_xd: f32,
        enu_yd: f32,
        enu_zd: f32,
        tow: u32,
        course: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct KITE_COMMAND_DATA {
        POWER: u16,
        TURN: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct PAYLOAD_COMMAND_DATA {
        ac_id: u8,
        command: Vec<u8>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SET_ACTUATOR_DATA {
        value: u16,
        no: u8,
        ac_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct CSC_SERVO_CMD_DATA {
        servo_1: u16,
        servo_2: u16,
        servo_3: u16,
        servo_4: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct BOOZ2_FMS_COMMAND_DATA {
        h_mode: u8,
        v_mode: u8,
        v_sp: i32,
        h_sp_1: i32,
        h_sp_2: i32,
        h_sp_3: i32,
        ac_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct BOOZ_NAV_STICK_DATA {
        ac_id: u8,
        vx_sp: i8,
        vy_sp: i8,
        vz_sp: i8,
        r_sp: i8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct EXTERNAL_FILTER_SOLUTION_DATA {
        ac_id: u8,
        status: u8,
        x: f32,
        xd: f32,
        y: f32,
        yd: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ROTORCRAFT_CAM_STICK_DATA {
        ac_id: u8,
        tilt: i8,
        pan: i8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GPS_INJECT_DATA {
        ac_id: u8,
        packet_id: u8,
        data: Vec<u8>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct EXTERNAL_MAG_RAW_DATA {
        x: i16,
        y: i16,
        z: i16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct VIDEO_ROI_DATA {
        /* The aircraft in which video stream you clicked */
        ac_id: u8,
        /* The x of the upper left corner of the selected area*/
        startx: i32,
        /* The y of the upper left corner of the selected area */
        starty: i32,
        /* The width of the selected area */
        width: i32,
        /* The height of the selected area */
        height: i32,
        /* The width of the image you received. Added because a module the receives this message does not know how big the image was that was broadcasted */
        downsized_width: i32,
    }
    /* Overcome setting ID and block ID problems in the case of multiboard autopilots like AP/FBW. With this message a KILL command can be sent to AP and FBW at the same time. */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct EMERGENCY_CMD_DATA {
        ac_id: u8,
        cmd: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct RTCM_INJECT_DATA {
        packet_id: u8,
        data: Vec<u8>, /* unspecified */
    }
    /* Init the table of an aircraft for the Distributed Circular Formation algorithm. If the nei_id is equal to zero, then you wipe out (clean) the whole table of the aircraft. */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DCF_REG_TABLE_DATA {
        /* ID of the table's aircraft to be updated*/
        ac_id: u8,
        /* ID of the neighbor for the table*/
        nei_id: u8,
        /* Desired angle to have w.r.t. the neighbor*/
        desired_sigma: i16,
    }
    /* Message for key exchange during crypto initialization */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct KEY_EXCHANGE_GCS_DATA {
        /* ID of the table's aircraft to be updated*/
        ac_id: u8,
        msg_type: u8,
        msg_data: Vec<u8>, /* unspecified */
    }
}
pub mod ground {
    pub enum PprzMessageGround {
        NEW_AIRCRAFT(NEW_AIRCRAFT_DATA),
        AIRCRAFT_DIE(AIRCRAFT_DIE_DATA),
        AIRCRAFTS(AIRCRAFTS_DATA),
        AIRCRAFTS_REQ(AIRCRAFTS_REQ_DATA),
        SELECTED(SELECTED_DATA),
        SELECTED_REQ(SELECTED_REQ_DATA),
        WIND_CLEAR(WIND_CLEAR_DATA),
        WIND(WIND_DATA),
        CONFIG_REQ(CONFIG_REQ_DATA),
        CONFIG(CONFIG_DATA),
        FLIGHT_PARAM(FLIGHT_PARAM_DATA),
        AP_STATUS(AP_STATUS_DATA),
        NAV_STATUS(NAV_STATUS_DATA),
        CAM_STATUS(CAM_STATUS_DATA),
        ENGINE_STATUS(ENGINE_STATUS_DATA),
        SVSINFO(SVSINFO_DATA),
        FLY_BY_WIRE(FLY_BY_WIRE_DATA),
        INFLIGH_CALIB(INFLIGH_CALIB_DATA),
        WORLD_ENV(WORLD_ENV_DATA),
        WORLD_ENV_REQ(WORLD_ENV_REQ_DATA),
        CIRCLE_STATUS(CIRCLE_STATUS_DATA),
        SEGMENT_STATUS(SEGMENT_STATUS_DATA),
        MOVE_WAYPOINT(MOVE_WAYPOINT_DATA),
        GET_DL_SETTING(GET_DL_SETTING_DATA),
        DL_SETTING(DL_SETTING_DATA),
        JUMP_TO_BLOCK(JUMP_TO_BLOCK_DATA),
        DL_VALUES(DL_VALUES_DATA),
        RAW_DATALINK(RAW_DATALINK_DATA),
        WAYPOINT_MOVED(WAYPOINT_MOVED_DATA),
        SURVEY_STATUS(SURVEY_STATUS_DATA),
        TELEMETRY_STATUS(TELEMETRY_STATUS_DATA),
        TELEMETRY_ERROR(TELEMETRY_ERROR_DATA),
        TELEMETRY_MESSAGE(TELEMETRY_MESSAGE_DATA),
        DATALINK_MESSAGE(DATALINK_MESSAGE_DATA),
        LINK_REPORT(LINK_REPORT_DATA),
        INTRUDER(INTRUDER_DATA),
        SHAPE(SHAPE_DATA),
        DL_EMERGENCY_CMD(DL_EMERGENCY_CMD_DATA),
        GROUND_REF(GROUND_REF_DATA),
        JOYSTICK(JOYSTICK_DATA),
        PLUMES(PLUMES_DATA),
    }
    impl PprzMessageGround {
        pub fn message_id(&self) -> u8 {
            use self::PprzMessageGround::*;
            match self {
                NEW_AIRCRAFT(..) => 1,
                AIRCRAFT_DIE(..) => 2,
                AIRCRAFTS(..) => 3,
                AIRCRAFTS_REQ(..) => 4,
                SELECTED(..) => 5,
                SELECTED_REQ(..) => 6,
                WIND_CLEAR(..) => 7,
                WIND(..) => 8,
                CONFIG_REQ(..) => 9,
                CONFIG(..) => 10,
                FLIGHT_PARAM(..) => 11,
                AP_STATUS(..) => 12,
                NAV_STATUS(..) => 13,
                CAM_STATUS(..) => 14,
                ENGINE_STATUS(..) => 15,
                SVSINFO(..) => 16,
                FLY_BY_WIRE(..) => 17,
                INFLIGH_CALIB(..) => 19,
                WORLD_ENV(..) => 20,
                WORLD_ENV_REQ(..) => 21,
                CIRCLE_STATUS(..) => 22,
                SEGMENT_STATUS(..) => 23,
                MOVE_WAYPOINT(..) => 24,
                GET_DL_SETTING(..) => 25,
                DL_SETTING(..) => 26,
                JUMP_TO_BLOCK(..) => 27,
                DL_VALUES(..) => 28,
                RAW_DATALINK(..) => 29,
                WAYPOINT_MOVED(..) => 30,
                SURVEY_STATUS(..) => 31,
                TELEMETRY_STATUS(..) => 32,
                TELEMETRY_ERROR(..) => 33,
                TELEMETRY_MESSAGE(..) => 34,
                DATALINK_MESSAGE(..) => 35,
                LINK_REPORT(..) => 36,
                INTRUDER(..) => 37,
                SHAPE(..) => 38,
                DL_EMERGENCY_CMD(..) => 39,
                GROUND_REF(..) => 40,
                JOYSTICK(..) => 41,
                PLUMES(..) => 100,
            }
        }
    } /* This file was automatically generated, do not edit */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct NEW_AIRCRAFT_DATA {
        ac_id: String,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AIRCRAFT_DIE_DATA {
        ac_id: String,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AIRCRAFTS_DATA {
        ac_list: String,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AIRCRAFTS_REQ_DATA {}
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SELECTED_DATA {
        aircraft_id: String,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SELECTED_REQ_DATA {}
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct WIND_CLEAR_DATA {
        ac_id: String,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct WIND_DATA {
        ac_id: String,
        dir: f32,
        wspeed: f32,
        mean_aspeed: f32,
        stddev: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct CONFIG_REQ_DATA {
        ac_id: String,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct CONFIG_DATA {
        ac_id: String,
        flight_plan: String,
        airframe: String,
        radio: String,
        settings: String,
        default_gui_color: String,
        ac_name: String,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct FLIGHT_PARAM_DATA {
        ac_id: String,
        roll: f32,
        pitch: f32,
        heading: f32,
        lat: f32,
        long: f32,
        speed: f32,
        course: f32,
        alt: f32,
        climb: f32,
        agl: f32,
        unix_time: f32,
        itow: u32,
        airspeed: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AP_STATUS_DATA {
        ac_id: String,
        ap_mode: String,
        lat_mode: String,
        horiz_mode: String,
        gaz_mode: String,
        gps_mode: String,
        kill_mode: String,
        flight_time: u32,
        state_filter_mode: String,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct NAV_STATUS_DATA {
        ac_id: String,
        cur_block: u8,
        cur_stage: u8,
        block_time: u32,
        stage_time: u32,
        target_lat: f32,
        target_long: f32,
        target_climb: f32,
        target_alt: f32,
        target_course: f32,
        dist_to_wp: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct CAM_STATUS_DATA {
        ac_id: String,
        lats: String,
        longs: String,
        cam_target_lat: f32,
        cam_target_long: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ENGINE_STATUS_DATA {
        ac_id: String,
        throttle: f32,
        throttle_accu: f32,
        rpm: f32,
        temp: f32,
        bat: f32,
        amp: f32,
        energy: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SVSINFO_DATA {
        ac_id: String,
        pacc: u16,
        svid: String,
        flags: String,
        qi: String,
        cno: String,
        elev: String,
        azim: String,
        msg_age: String,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct FLY_BY_WIRE_DATA {
        ac_id: String,
        rc_status: String,
        rc_mode: String,
        rc_rate: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct INFLIGH_CALIB_DATA {
        ac_id: String,
        if_mode: String,
        if_value1: f32,
        if_value2: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct WORLD_ENV_DATA {
        wind_east: f32,
        wind_north: f32,
        wind_up: f32,
        ir_contrast: f32,
        time_scale: f32,
        gps_availability: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct WORLD_ENV_REQ_DATA {
        lat: f32,
        long: f32,
        alt: f32,
        east: f32,
        north: f32,
        up: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct CIRCLE_STATUS_DATA {
        ac_id: String,
        circle_lat: f32,
        circle_long: f32,
        radius: i16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SEGMENT_STATUS_DATA {
        ac_id: String,
        segment1_lat: f32,
        segment1_long: f32,
        segment2_lat: f32,
        segment2_long: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MOVE_WAYPOINT_DATA {
        ac_id: String,
        wp_id: u8,
        lat: f32,
        long: f32,
        alt: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GET_DL_SETTING_DATA {
        ac_id: String,
        index: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DL_SETTING_DATA {
        ac_id: String,
        index: u8,
        value: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct JUMP_TO_BLOCK_DATA {
        ac_id: String,
        block_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DL_VALUES_DATA {
        ac_id: String,
        values: String,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct RAW_DATALINK_DATA {
        ac_id: String,
        message: String,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct WAYPOINT_MOVED_DATA {
        ac_id: String,
        wp_id: u8,
        lat: f32,
        long: f32,
        alt: f32,
        ground_alt: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SURVEY_STATUS_DATA {
        ac_id: String,
        east_long: f32,
        north_lat: f32,
        west_long: f32,
        south_lat: f32,
    }
    /* Datalink status reported by Server for the GCS Combines DATLINK_REPORT (telemetry class) and LINK_REPORT (ground class) */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TELEMETRY_STATUS_DATA {
        ac_id: String,
        link_id: String,
        time_since_last_msg: f32,
        rx_bytes: u32,
        rx_msgs: u32,
        rx_bytes_rate: f32,
        tx_msgs: u32,
        uplink_lost_time: u32,
        uplink_msgs: u16,
        downlink_msgs: u16,
        downlink_rate: u16,
        ping_time: f32,
    }
    /* Report a telemetry error */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TELEMETRY_ERROR_DATA {
        ac_id: String,
        message: String,
    }
    /* Encapsulated a telemetry class message (when using redundant link) */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TELEMETRY_MESSAGE_DATA {
        ac_id: String,
        link_id: String,
        message: String,
    }
    /* Encapsulated a datalink class message (when using redundant link) */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DATALINK_MESSAGE_DATA {
        ac_id: String,
        link_id: String,
        message: String,
    }
    /* Datalink status reported by Link for the Server */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct LINK_REPORT_DATA {
        ac_id: String,
        link_id: String,
        run_time: u32,
        rx_lost_time: u32,
        rx_bytes: u32,
        rx_msgs: u32,
        rx_err: u32,
        rx_bytes_rate: f32,
        rx_msgs_rate: f32,
        tx_msgs: u32,
        ping_time: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct INTRUDER_DATA {
        id: String,
        name: String,
        lat: i32,
        lon: i32,
        /* altitude above WGS84 reference ellipsoid*/
        alt: i32,
        course: f32,
        speed: f32,
        climb: f32,
        itow: u32,
    }
    /* The SHAPE message used to draw shapes onto the Paparazzi GCS. Field name shape is used to define the type of shape i.e. Circle, Polygon, Line, or Text. This is indexed from 0-3 respectively. Each shape drawn must have an id number associated with it. This id number in conjuction with the shapetype will be needed to update or delete the shape. A circle can be defined with the same id as a polygon but since they have different shape types they are considered unique. linecolor and fillcolor take in a color string ie: "red", "blue" opacity will change the level of transparency of the fill. 0 - Transparent 1 - Light Fill 2 - Medium Fill 3 - Opaque Passing a status of 0 will create or update the shape specified by id and type. Passing a status of 1 will delete the shape specified by id and type. latarr is an array of coordinates that contain the latitude coordinate for each point in the shape. The array is comma separated. lonarr is similar to latarr but contain the longitude coordinate for each point in the shape. Circle and Text type will take the first coordinates given to place the shape. Polygon will take all the coordinates given. Line will take the first two coordinates given. Radius is only used for the circle. Text will always be populated with each message using the first set of coordinates. The text field can not be blank or have spaces. If text is not desired for a shape then pass "NULL" into the text field. */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SHAPE_DATA {
        id: u8,
        linecolor: String,
        fillcolor: String,
        opacity: u8,
        shape: u8,
        status: u8,
        latarr: Vec<i32>, /* unspecified */
        lonarr: Vec<i32>, /* unspecified */
        radius: f32,
        text: String,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DL_EMERGENCY_CMD_DATA {
        ac_id: String,
        cmd: u8,
    }
    /* Ground reference provided by an external positioning system for instance */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GROUND_REF_DATA {
        /* Aircraft ID*/
        ac_id: String,
        /* position reference frame*/
        frame: String,
        /* position in selected frame*/
        pos: Vec<f32>, /* 3 */
        /* speed in selected frame (LTP_NED in case of LLA frame)*/
        speed: Vec<f32>, /* 3 */
        /* Unitary quaternion representing LTP to BODY orientation (qi, qx, qy, qz)*/
        quat: Vec<f32>, /* 4 */
        /* Rotation speeds in body frame*/
        rate: Vec<f32>, /* 3 */
        /* Timestamp*/
        timestamp: f32,
    }
    /* Generic 4 axis and 4 buttons joystick or joypad. This message can be provided by the 'input2ivy' tool for other ground agents. Standard joystick axis values are on 16 bits signed integers, but tools like 'input2ivy' may scale them on int8 type. */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct JOYSTICK_DATA {
        /* Joystick ID*/
        id: u8,
        /* axis 1 position*/
        axis1: i16,
        /* axis 2 position*/
        axis2: i16,
        /* axis 3 position*/
        axis3: i16,
        /* axis 4 position*/
        axis4: i16,
        /* button 1 status*/
        button1: u8,
        /* button 2 status*/
        button2: u8,
        /* button 3 status*/
        button3: u8,
        /* button 4 status*/
        button4: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct PLUMES_DATA {
        ids: String,
        lats: String,
        longs: String,
        values: String,
    }
}
pub mod alert {
    pub enum PprzMessageAlert {
        BAT_LOW(BAT_LOW_DATA),
        AIR_PROX(AIR_PROX_DATA),
    }
    impl PprzMessageAlert {
        pub fn message_id(&self) -> u8 {
            use self::PprzMessageAlert::*;
            match self {
                BAT_LOW(..) => 1,
                AIR_PROX(..) => 2,
            }
        }
    } /* This file was automatically generated, do not edit */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct BAT_LOW_DATA {
        ac_id: String,
        level: String,
        value: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AIR_PROX_DATA {
        ac_id: String,
        level: String,
    }
}
pub mod intermcu {
    pub enum PprzMessageIntermcu {
        IMCU_COMMANDS(IMCU_COMMANDS_DATA),
        IMCU_RADIO_COMMANDS(IMCU_RADIO_COMMANDS_DATA),
        IMCU_SPEKTRUM_SOFT_BIND(IMCU_SPEKTRUM_SOFT_BIND_DATA),
        IMCU_FBW_STATUS(IMCU_FBW_STATUS_DATA),
        IMCU_REMOTE_MAG(IMCU_REMOTE_MAG_DATA),
        IMCU_REMOTE_BARO(IMCU_REMOTE_BARO_DATA),
        IMCU_REMOTE_AIRSPEED(IMCU_REMOTE_AIRSPEED_DATA),
        IMCU_REMOTE_GROUND(IMCU_REMOTE_GROUND_DATA),
        IMCU_REMOTE_GPS(IMCU_REMOTE_GPS_DATA),
        STEREOCAM_ARRAY(STEREOCAM_ARRAY_DATA),
        STEREOCAM_VELOCITY(STEREOCAM_VELOCITY_DATA),
        STEREOCAM_STATE(STEREOCAM_STATE_DATA),
        STEREOCAM_FOLLOW_ME(STEREOCAM_FOLLOW_ME_DATA),
        IMCU_DATALINK(IMCU_DATALINK_DATA),
        IMCU_TELEMETRY(IMCU_TELEMETRY_DATA),
        IMCU_DEBUG(IMCU_DEBUG_DATA),
        IMCU_PAYLOAD(IMCU_PAYLOAD_DATA),
    }
    impl PprzMessageIntermcu {
        pub fn message_id(&self) -> u8 {
            use self::PprzMessageIntermcu::*;
            match self {
                IMCU_COMMANDS(..) => 1,
                IMCU_RADIO_COMMANDS(..) => 2,
                IMCU_SPEKTRUM_SOFT_BIND(..) => 3,
                IMCU_FBW_STATUS(..) => 4,
                IMCU_REMOTE_MAG(..) => 10,
                IMCU_REMOTE_BARO(..) => 11,
                IMCU_REMOTE_AIRSPEED(..) => 12,
                IMCU_REMOTE_GROUND(..) => 15,
                IMCU_REMOTE_GPS(..) => 62,
                STEREOCAM_ARRAY(..) => 80,
                STEREOCAM_VELOCITY(..) => 81,
                STEREOCAM_STATE(..) => 82,
                STEREOCAM_FOLLOW_ME(..) => 83,
                IMCU_DATALINK(..) => 113,
                IMCU_TELEMETRY(..) => 114,
                IMCU_DEBUG(..) => 115,
                IMCU_PAYLOAD(..) => 116,
            }
        }
    } /* This file was automatically generated, do not edit */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMCU_COMMANDS_DATA {
        status: u8,
        values: Vec<i16>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMCU_RADIO_COMMANDS_DATA {
        status: u8,
        values: Vec<i16>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMCU_SPEKTRUM_SOFT_BIND_DATA {}
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMCU_FBW_STATUS_DATA {
        mode: u8,
        rc_status: u8,
        frame_rate: u8,
        vsupply: u16,
        current: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMCU_REMOTE_MAG_DATA {
        /* RAW Magnetometer Data*/
        mag_x: i16,
        mag_y: i16,
        mag_z: i16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMCU_REMOTE_BARO_DATA {
        /* Static Barometric Pressure in Pascal*/
        pitot_stat: f32,
        /* Pressure Sensor Temperature*/
        pitot_temp: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMCU_REMOTE_AIRSPEED_DATA {
        /* Indicated Airspeed*/
        pitot_IAS: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMCU_REMOTE_GROUND_DATA {
        mode: u8,
        /* Sensor ID*/
        id: u8,
        range: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMCU_REMOTE_GPS_DATA {
        ecef_x: i32,
        ecef_y: i32,
        ecef_z: i32,
        /* altitude above WGS84 reference ellipsoid*/
        alt: i32,
        /* height above mean sea level (geoid)*/
        hmsl: i32,
        ecef_xd: i32,
        ecef_yd: i32,
        ecef_zd: i32,
        course: i32,
        gspeed: u16,
        pacc: u32,
        sacc: u32,
        numsv: u8,
        fix: u8,
    }
    /* Raw data fromt the stereocamera. Type defines what kind of data it is. This can be raw image, disparity map, obstacle histogram, ect. */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct STEREOCAM_ARRAY_DATA {
        fieldtype: u8,
        /* array size parameters*/
        width: u8,
        height: u8,
        /* If the data being sent is too large for one message (e.g when sending a full image) this will indicate which package of the total data is contained in this message*/
        package_nb: u8,
        image_data: Vec<u8>, /* unspecified */
    }
    /* Velocity measured using optical flow and stereovision. All parameters are in the camera frame */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct STEREOCAM_VELOCITY_DATA {
        /* Resolution of the vel and pos messages*/
        resolution: u8,
        /* Time difference to previous frame*/
        dt_frame: u8,
        /* Time difference to previous message, not strictly required*/
        dt: u8,
        /* Velocity estimaed using stereo edgeflow*/
        velx: i16,
        vely: i16,
        velz: i16,
        /* Distance traveled since the last message*/
        dposx: i16,
        dposy: i16,
        dposz: i16,
        /* RMS of the velocity estimate*/
        vRMS: u8,
        /* RMS of the position*/
        posRMS: u8,
        /* Average distance to scene*/
        avg_dist: u16,
    }
    /* Estimated state of the camera. As the stereocamera has no inertial sensors, this data should be sent to the stereocamera to enable onboard derotation of the optical flow */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct STEREOCAM_STATE_DATA {
        /* Pitch, roll and yaw angles of the camera*/
        phi: f32,
        theta: f32,
        psi: f32,
        /* Altitude above the ground. If not looking down, set to 0*/
        agl: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct STEREOCAM_FOLLOW_ME_DATA {
        headingToFollow: u8,
        heightObject: u8,
        distanceToObject: u8,
    }
    /* Forward FBW datalink to AP */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMCU_DATALINK_DATA {
        msg: Vec<u8>, /* unspecified */
    }
    /* Forward AP telemetry to FBW */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMCU_TELEMETRY_DATA {
        msg: Vec<u8>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMCU_DEBUG_DATA {
        msg: Vec<u8>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMCU_PAYLOAD_DATA {
        data: Vec<u8>, /* unspecified */
    }
}
pub mod telemetry {
    pub enum PprzMessageTelemetry {
        AUTOPILOT_VERSION(AUTOPILOT_VERSION_DATA),
        ALIVE(ALIVE_DATA),
        PONG(PONG_DATA),
        TAKEOFF(TAKEOFF_DATA),
        ARDRONE_NAVDATA(ARDRONE_NAVDATA_DATA),
        ATTITUDE(ATTITUDE_DATA),
        IR_SENSORS(IR_SENSORS_DATA),
        GPS(GPS_DATA),
        NAVIGATION_REF(NAVIGATION_REF_DATA),
        NAVIGATION(NAVIGATION_DATA),
        PPRZ_MODE(PPRZ_MODE_DATA),
        BAT(BAT_DATA),
        DEBUG_MCU_LINK(DEBUG_MCU_LINK_DATA),
        CALIBRATION(CALIBRATION_DATA),
        SETTINGS(SETTINGS_DATA),
        DESIRED(DESIRED_DATA),
        GPS_SOL(GPS_SOL_DATA),
        ADC_GENERIC(ADC_GENERIC_DATA),
        ECU(ECU_DATA),
        CAM(CAM_DATA),
        CIRCLE(CIRCLE_DATA),
        SEGMENT(SEGMENT_DATA),
        VECTORNAV_INFO(VECTORNAV_INFO_DATA),
        HYBRID_GUIDANCE(HYBRID_GUIDANCE_DATA),
        SVINFO(SVINFO_DATA),
        DEBUG(DEBUG_DATA),
        SURVEY(SURVEY_DATA),
        RSSI(RSSI_DATA),
        RANGEFINDER(RANGEFINDER_DATA),
        DATALINK_REPORT(DATALINK_REPORT_DATA),
        DL_VALUE(DL_VALUE_DATA),
        MARK(MARK_DATA),
        SYS_MON(SYS_MON_DATA),
        MOTOR(MOTOR_DATA),
        WP_MOVED(WP_MOVED_DATA),
        MKK(MKK_DATA),
        ENERGY(ENERGY_DATA),
        DRAGSPEED(DRAGSPEED_DATA),
        RSSI_COMBINED(RSSI_COMBINED_DATA),
        DCF(DCF_DATA),
        ALT_KALMAN(ALT_KALMAN_DATA),
        ESTIMATOR(ESTIMATOR_DATA),
        TUNE_ROLL(TUNE_ROLL_DATA),
        BARO_MS5534A(BARO_MS5534A_DATA),
        BARO_WORDS(BARO_WORDS_DATA),
        WP_MOVED_LLA(WP_MOVED_LLA_DATA),
        RLFILTER(RLFILTER_DATA),
        WP_MOVED_ENU(WP_MOVED_ENU_DATA),
        WINDTURBINE_STATUS_(WINDTURBINE_STATUS__DATA),
        RC_3CH_(RC_3CH__DATA),
        MPPT(MPPT_DATA),
        DEBUG_IR_I2C(DEBUG_IR_I2C_DATA),
        AIRSPEED(AIRSPEED_DATA),
        BARO_ETS(BARO_ETS_DATA),
        AIRSPEED_ETS(AIRSPEED_ETS_DATA),
        VISION_OUTBACK(VISION_OUTBACK_DATA),
        GPS_LLA(GPS_LLA_DATA),
        H_CTL_A(H_CTL_A_DATA),
        TURB_PRESSURE_VOLTAGE(TURB_PRESSURE_VOLTAGE_DATA),
        CAM_POINT(CAM_POINT_DATA),
        DC_INFO(DC_INFO_DATA),
        AMSYS_BARO(AMSYS_BARO_DATA),
        AMSYS_AIRSPEED(AMSYS_AIRSPEED_DATA),
        FLIGHT_BENCHMARK(FLIGHT_BENCHMARK_DATA),
        MPL3115_BARO(MPL3115_BARO_DATA),
        AOA(AOA_DATA),
        XTEND_RSSI(XTEND_RSSI_DATA),
        GVF(GVF_DATA),
        SUPERBITRF(SUPERBITRF_DATA),
        GX3_INFO(GX3_INFO_DATA),
        UBLOX_INFO(UBLOX_INFO_DATA),
        INV_FILTER(INV_FILTER_DATA),
        MISSION_STATUS(MISSION_STATUS_DATA),
        GENERIC_COM(GENERIC_COM_DATA),
        FORMATION_SLOT_TM(FORMATION_SLOT_TM_DATA),
        FORMATION_STATUS_TM(FORMATION_STATUS_TM_DATA),
        BMP_STATUS(BMP_STATUS_DATA),
        MLX_STATUS(MLX_STATUS_DATA),
        TMP_STATUS(TMP_STATUS_DATA),
        WIND_INFO_RET(WIND_INFO_RET_DATA),
        SCP_STATUS(SCP_STATUS_DATA),
        SHT_STATUS(SHT_STATUS_DATA),
        VISION_POSITION_ESTIMATE(VISION_POSITION_ESTIMATE_DATA),
        DPICCO_STATUS(DPICCO_STATUS_DATA),
        LOGGER_STATUS(LOGGER_STATUS_DATA),
        MOTOR_BENCH_STATUS(MOTOR_BENCH_STATUS_DATA),
        HIH_STATUS(HIH_STATUS_DATA),
        TEMT_STATUS(TEMT_STATUS_DATA),
        GP2Y_STATUS(GP2Y_STATUS_DATA),
        SHT_I2C_SERIAL(SHT_I2C_SERIAL_DATA),
        PPM(PPM_DATA),
        RC(RC_DATA),
        COMMANDS(COMMANDS_DATA),
        FBW_STATUS(FBW_STATUS_DATA),
        ADC(ADC_DATA),
        ACTUATORS(ACTUATORS_DATA),
        BLUEGIGA(BLUEGIGA_DATA),
        THROTTLE_CURVE(THROTTLE_CURVE_DATA),
        PIKSI_HEARTBEAT(PIKSI_HEARTBEAT_DATA),
        MULTIGAZE_METERS(MULTIGAZE_METERS_DATA),
        DC_SHOT(DC_SHOT_DATA),
        CAMERA_PAYLOAD(CAMERA_PAYLOAD_DATA),
        MOTOR_MIXING(MOTOR_MIXING_DATA),
        MLX_SERIAL(MLX_SERIAL_DATA),
        PAYLOAD(PAYLOAD_DATA),
        HTM_STATUS(HTM_STATUS_DATA),
        BARO_MS5611(BARO_MS5611_DATA),
        MS5611_COEFF(MS5611_COEFF_DATA),
        ATMOSPHERE_CHARGE(ATMOSPHERE_CHARGE_DATA),
        SOLAR_RADIATION(SOLAR_RADIATION_DATA),
        TCAS_TA(TCAS_TA_DATA),
        TCAS_RA(TCAS_RA_DATA),
        TCAS_RESOLVED(TCAS_RESOLVED_DATA),
        TCAS_DEBUG(TCAS_DEBUG_DATA),
        POTENTIAL(POTENTIAL_DATA),
        COPILOT_STATUS(COPILOT_STATUS_DATA),
        TEMP_TCOUPLE(TEMP_TCOUPLE_DATA),
        SHT_I2C_STATUS(SHT_I2C_STATUS_DATA),
        CAMERA_SNAPSHOT(CAMERA_SNAPSHOT_DATA),
        TIMESTAMP(TIMESTAMP_DATA),
        STAB_ATTITUDE_FLOAT(STAB_ATTITUDE_FLOAT_DATA),
        IMU_GYRO_SCALED(IMU_GYRO_SCALED_DATA),
        IMU_ACCEL_SCALED(IMU_ACCEL_SCALED_DATA),
        IMU_MAG_SCALED(IMU_MAG_SCALED_DATA),
        FILTER(FILTER_DATA),
        RATE_LOOP(RATE_LOOP_DATA),
        FILTER_ALIGNER(FILTER_ALIGNER_DATA),
        AIRSPEED_MS45XX(AIRSPEED_MS45XX_DATA),
        STAB_ATTITUDE_INT(STAB_ATTITUDE_INT_DATA),
        STAB_ATTITUDE_REF_INT(STAB_ATTITUDE_REF_INT_DATA),
        STAB_ATTITUDE_REF_FLOAT(STAB_ATTITUDE_REF_FLOAT_DATA),
        ROTORCRAFT_CMD(ROTORCRAFT_CMD_DATA),
        GUIDANCE_H_INT(GUIDANCE_H_INT_DATA),
        VERT_LOOP(VERT_LOOP_DATA),
        HOVER_LOOP(HOVER_LOOP_DATA),
        ROTORCRAFT_FP(ROTORCRAFT_FP_DATA),
        TEMP_ADC(TEMP_ADC_DATA),
        GUIDANCE_H_REF_INT(GUIDANCE_H_REF_INT_DATA),
        ROTORCRAFT_TUNE_HOVER(ROTORCRAFT_TUNE_HOVER_DATA),
        INS_Z(INS_Z_DATA),
        PCAP01_STATUS(PCAP01_STATUS_DATA),
        GEIGER_COUNTER(GEIGER_COUNTER_DATA),
        INS_REF(INS_REF_DATA),
        GPS_INT(GPS_INT_DATA),
        AHRS_EULER_INT(AHRS_EULER_INT_DATA),
        AHRS_QUAT_INT(AHRS_QUAT_INT_DATA),
        ROTORCRAFT_NAV_STATUS(ROTORCRAFT_NAV_STATUS_DATA),
        ROTORCRAFT_RADIO_CONTROL(ROTORCRAFT_RADIO_CONTROL_DATA),
        VFF_EXTENDED(VFF_EXTENDED_DATA),
        VFF(VFF_DATA),
        GEO_MAG(GEO_MAG_DATA),
        HFF(HFF_DATA),
        HFF_DBG(HFF_DBG_DATA),
        HFF_GPS(HFF_GPS_DATA),
        ROTORCRAFT_CAM(ROTORCRAFT_CAM_DATA),
        AHRS_REF_QUAT(AHRS_REF_QUAT_DATA),
        AHRS_EULER(AHRS_EULER_DATA),
        AHRS_MEASUREMENT_EULER(AHRS_MEASUREMENT_EULER_DATA),
        WT(WT_DATA),
        CSC_CAN_DEBUG(CSC_CAN_DEBUG_DATA),
        CSC_CAN_MSG(CSC_CAN_MSG_DATA),
        AHRS_GYRO_BIAS_INT(AHRS_GYRO_BIAS_INT_DATA),
        AEROPROBE(AEROPROBE_DATA),
        FMS_TIME(FMS_TIME_DATA),
        AHRS_LKF(AHRS_LKF_DATA),
        NPS_SENSORS_SCALED(NPS_SENSORS_SCALED_DATA),
        INS(INS_DATA),
        IMU_GYRO(IMU_GYRO_DATA),
        IMU_MAG(IMU_MAG_DATA),
        IMU_ACCEL(IMU_ACCEL_DATA),
        IMU_GYRO_RAW(IMU_GYRO_RAW_DATA),
        IMU_ACCEL_RAW(IMU_ACCEL_RAW_DATA),
        IMU_MAG_RAW(IMU_MAG_RAW_DATA),
        IMU_MAG_SETTINGS(IMU_MAG_SETTINGS_DATA),
        IMU_MAG_CURRENT_CALIBRATION(IMU_MAG_CURRENT_CALIBRATION_DATA),
        UART_ERRORS(UART_ERRORS_DATA),
        IMU_GYRO_LP(IMU_GYRO_LP_DATA),
        IMU_PRESSURE(IMU_PRESSURE_DATA),
        TUNE_VERT(TUNE_VERT_DATA),
        MF_DAQ_STATE(MF_DAQ_STATE_DATA),
        INFO_MSG(INFO_MSG_DATA),
        STAB_ATTITUDE_INDI(STAB_ATTITUDE_INDI_DATA),
        ROTORCRAFT_FP_MIN(ROTORCRAFT_FP_MIN_DATA),
        BEBOP_ACTUATORS(BEBOP_ACTUATORS_DATA),
        WEATHER(WEATHER_DATA),
        IMU_TURNTABLE(IMU_TURNTABLE_DATA),
        BARO_RAW(BARO_RAW_DATA),
        AIR_DATA(AIR_DATA_DATA),
        AMSL(AMSL_DATA),
        DIVERGENCE(DIVERGENCE_DATA),
        VIDEO_SYNC(VIDEO_SYNC_DATA),
        PERIODIC_TELEMETRY_ERR(PERIODIC_TELEMETRY_ERR_DATA),
        TIME(TIME_DATA),
        OPTIC_FLOW_EST(OPTIC_FLOW_EST_DATA),
        STEREO_IMG(STEREO_IMG_DATA),
        ROVER_STATUS(ROVER_STATUS_DATA),
        ROTORCRAFT_STATUS(ROTORCRAFT_STATUS_DATA),
        STATE_FILTER_STATUS(STATE_FILTER_STATUS_DATA),
        PX4FLOW(PX4FLOW_DATA),
        LIDAR(LIDAR_DATA),
        VISUALTARGET(VISUALTARGET_DATA),
        SONAR(SONAR_DATA),
        PAYLOAD_FLOAT(PAYLOAD_FLOAT_DATA),
        NPS_POS_LLH(NPS_POS_LLH_DATA),
        KEY_EXCHANGE_UAV(KEY_EXCHANGE_UAV_DATA),
        NPS_SPEED_POS(NPS_SPEED_POS_DATA),
        NPS_RATE_ATTITUDE(NPS_RATE_ATTITUDE_DATA),
        NPS_GYRO_BIAS(NPS_GYRO_BIAS_DATA),
        OPTICAL_FLOW_HOVER(OPTICAL_FLOW_HOVER_DATA),
        NPS_WIND(NPS_WIND_DATA),
        ESC(ESC_DATA),
        RTOS_MON(RTOS_MON_DATA),
        PPRZ_DEBUG(PPRZ_DEBUG_DATA),
        BATTERY_MONITOR(BATTERY_MONITOR_DATA),
        GPS_RXMRTCM(GPS_RXMRTCM_DATA),
        INDI_G(INDI_G_DATA),
        GPS_RTK(GPS_RTK_DATA),
        GPS_SMALL(GPS_SMALL_DATA),
        I2C_ERRORS(I2C_ERRORS_DATA),
        DCF_THETA(DCF_THETA_DATA),
        SECURE_LINK_STATUS(SECURE_LINK_STATUS_DATA),
    }
    impl PprzMessageTelemetry {
        pub fn message_id(&self) -> u8 {
            use self::PprzMessageTelemetry::*;
            match self {
                AUTOPILOT_VERSION(..) => 1,
                ALIVE(..) => 2,
                PONG(..) => 3,
                TAKEOFF(..) => 4,
                ARDRONE_NAVDATA(..) => 5,
                ATTITUDE(..) => 6,
                IR_SENSORS(..) => 7,
                GPS(..) => 8,
                NAVIGATION_REF(..) => 9,
                NAVIGATION(..) => 10,
                PPRZ_MODE(..) => 11,
                BAT(..) => 12,
                DEBUG_MCU_LINK(..) => 13,
                CALIBRATION(..) => 14,
                SETTINGS(..) => 15,
                DESIRED(..) => 16,
                GPS_SOL(..) => 17,
                ADC_GENERIC(..) => 18,
                ECU(..) => 19,
                CAM(..) => 20,
                CIRCLE(..) => 21,
                SEGMENT(..) => 22,
                VECTORNAV_INFO(..) => 23,
                HYBRID_GUIDANCE(..) => 24,
                SVINFO(..) => 25,
                DEBUG(..) => 26,
                SURVEY(..) => 27,
                RSSI(..) => 28,
                RANGEFINDER(..) => 29,
                DATALINK_REPORT(..) => 30,
                DL_VALUE(..) => 31,
                MARK(..) => 32,
                SYS_MON(..) => 33,
                MOTOR(..) => 34,
                WP_MOVED(..) => 35,
                MKK(..) => 36,
                ENERGY(..) => 37,
                DRAGSPEED(..) => 38,
                RSSI_COMBINED(..) => 39,
                DCF(..) => 40,
                ALT_KALMAN(..) => 41,
                ESTIMATOR(..) => 42,
                TUNE_ROLL(..) => 43,
                BARO_MS5534A(..) => 44,
                BARO_WORDS(..) => 46,
                WP_MOVED_LLA(..) => 47,
                RLFILTER(..) => 48,
                WP_MOVED_ENU(..) => 49,
                WINDTURBINE_STATUS_(..) => 50,
                RC_3CH_(..) => 51,
                MPPT(..) => 52,
                DEBUG_IR_I2C(..) => 53,
                AIRSPEED(..) => 54,
                BARO_ETS(..) => 56,
                AIRSPEED_ETS(..) => 57,
                VISION_OUTBACK(..) => 58,
                GPS_LLA(..) => 59,
                H_CTL_A(..) => 60,
                TURB_PRESSURE_VOLTAGE(..) => 62,
                CAM_POINT(..) => 63,
                DC_INFO(..) => 64,
                AMSYS_BARO(..) => 65,
                AMSYS_AIRSPEED(..) => 66,
                FLIGHT_BENCHMARK(..) => 67,
                MPL3115_BARO(..) => 68,
                AOA(..) => 69,
                XTEND_RSSI(..) => 70,
                GVF(..) => 71,
                SUPERBITRF(..) => 72,
                GX3_INFO(..) => 73,
                UBLOX_INFO(..) => 74,
                INV_FILTER(..) => 78,
                MISSION_STATUS(..) => 79,
                GENERIC_COM(..) => 81,
                FORMATION_SLOT_TM(..) => 82,
                FORMATION_STATUS_TM(..) => 83,
                BMP_STATUS(..) => 84,
                MLX_STATUS(..) => 85,
                TMP_STATUS(..) => 86,
                WIND_INFO_RET(..) => 87,
                SCP_STATUS(..) => 88,
                SHT_STATUS(..) => 89,
                VISION_POSITION_ESTIMATE(..) => 90,
                DPICCO_STATUS(..) => 91,
                LOGGER_STATUS(..) => 92,
                MOTOR_BENCH_STATUS(..) => 94,
                HIH_STATUS(..) => 96,
                TEMT_STATUS(..) => 97,
                GP2Y_STATUS(..) => 98,
                SHT_I2C_SERIAL(..) => 99,
                PPM(..) => 100,
                RC(..) => 101,
                COMMANDS(..) => 102,
                FBW_STATUS(..) => 103,
                ADC(..) => 104,
                ACTUATORS(..) => 105,
                BLUEGIGA(..) => 106,
                THROTTLE_CURVE(..) => 107,
                PIKSI_HEARTBEAT(..) => 108,
                MULTIGAZE_METERS(..) => 109,
                DC_SHOT(..) => 110,
                CAMERA_PAYLOAD(..) => 111,
                MOTOR_MIXING(..) => 112,
                MLX_SERIAL(..) => 113,
                PAYLOAD(..) => 114,
                HTM_STATUS(..) => 115,
                BARO_MS5611(..) => 116,
                MS5611_COEFF(..) => 117,
                ATMOSPHERE_CHARGE(..) => 118,
                SOLAR_RADIATION(..) => 119,
                TCAS_TA(..) => 120,
                TCAS_RA(..) => 121,
                TCAS_RESOLVED(..) => 122,
                TCAS_DEBUG(..) => 123,
                POTENTIAL(..) => 124,
                COPILOT_STATUS(..) => 125,
                TEMP_TCOUPLE(..) => 126,
                SHT_I2C_STATUS(..) => 127,
                CAMERA_SNAPSHOT(..) => 128,
                TIMESTAMP(..) => 129,
                STAB_ATTITUDE_FLOAT(..) => 130,
                IMU_GYRO_SCALED(..) => 131,
                IMU_ACCEL_SCALED(..) => 132,
                IMU_MAG_SCALED(..) => 133,
                FILTER(..) => 134,
                RATE_LOOP(..) => 136,
                FILTER_ALIGNER(..) => 137,
                AIRSPEED_MS45XX(..) => 138,
                STAB_ATTITUDE_INT(..) => 140,
                STAB_ATTITUDE_REF_INT(..) => 141,
                STAB_ATTITUDE_REF_FLOAT(..) => 142,
                ROTORCRAFT_CMD(..) => 143,
                GUIDANCE_H_INT(..) => 144,
                VERT_LOOP(..) => 145,
                HOVER_LOOP(..) => 146,
                ROTORCRAFT_FP(..) => 147,
                TEMP_ADC(..) => 148,
                GUIDANCE_H_REF_INT(..) => 149,
                ROTORCRAFT_TUNE_HOVER(..) => 150,
                INS_Z(..) => 151,
                PCAP01_STATUS(..) => 152,
                GEIGER_COUNTER(..) => 153,
                INS_REF(..) => 154,
                GPS_INT(..) => 155,
                AHRS_EULER_INT(..) => 156,
                AHRS_QUAT_INT(..) => 157,
                ROTORCRAFT_NAV_STATUS(..) => 159,
                ROTORCRAFT_RADIO_CONTROL(..) => 160,
                VFF_EXTENDED(..) => 161,
                VFF(..) => 162,
                GEO_MAG(..) => 163,
                HFF(..) => 164,
                HFF_DBG(..) => 165,
                HFF_GPS(..) => 166,
                ROTORCRAFT_CAM(..) => 168,
                AHRS_REF_QUAT(..) => 169,
                AHRS_EULER(..) => 173,
                AHRS_MEASUREMENT_EULER(..) => 174,
                WT(..) => 175,
                CSC_CAN_DEBUG(..) => 176,
                CSC_CAN_MSG(..) => 177,
                AHRS_GYRO_BIAS_INT(..) => 178,
                AEROPROBE(..) => 179,
                FMS_TIME(..) => 180,
                AHRS_LKF(..) => 193,
                NPS_SENSORS_SCALED(..) => 197,
                INS(..) => 198,
                IMU_GYRO(..) => 200,
                IMU_MAG(..) => 201,
                IMU_ACCEL(..) => 202,
                IMU_GYRO_RAW(..) => 203,
                IMU_ACCEL_RAW(..) => 204,
                IMU_MAG_RAW(..) => 205,
                IMU_MAG_SETTINGS(..) => 206,
                IMU_MAG_CURRENT_CALIBRATION(..) => 207,
                UART_ERRORS(..) => 208,
                IMU_GYRO_LP(..) => 209,
                IMU_PRESSURE(..) => 210,
                TUNE_VERT(..) => 213,
                MF_DAQ_STATE(..) => 214,
                INFO_MSG(..) => 215,
                STAB_ATTITUDE_INDI(..) => 216,
                ROTORCRAFT_FP_MIN(..) => 217,
                BEBOP_ACTUATORS(..) => 218,
                WEATHER(..) => 219,
                IMU_TURNTABLE(..) => 220,
                BARO_RAW(..) => 221,
                AIR_DATA(..) => 222,
                AMSL(..) => 223,
                DIVERGENCE(..) => 224,
                VIDEO_SYNC(..) => 225,
                PERIODIC_TELEMETRY_ERR(..) => 226,
                TIME(..) => 227,
                OPTIC_FLOW_EST(..) => 228,
                STEREO_IMG(..) => 229,
                ROVER_STATUS(..) => 230,
                ROTORCRAFT_STATUS(..) => 231,
                STATE_FILTER_STATUS(..) => 232,
                PX4FLOW(..) => 233,
                LIDAR(..) => 234,
                VISUALTARGET(..) => 235,
                SONAR(..) => 236,
                PAYLOAD_FLOAT(..) => 237,
                NPS_POS_LLH(..) => 238,
                KEY_EXCHANGE_UAV(..) => 239,
                NPS_SPEED_POS(..) => 240,
                NPS_RATE_ATTITUDE(..) => 241,
                NPS_GYRO_BIAS(..) => 242,
                OPTICAL_FLOW_HOVER(..) => 243,
                NPS_WIND(..) => 244,
                ESC(..) => 245,
                RTOS_MON(..) => 246,
                PPRZ_DEBUG(..) => 247,
                BATTERY_MONITOR(..) => 248,
                GPS_RXMRTCM(..) => 249,
                INDI_G(..) => 250,
                GPS_RTK(..) => 251,
                GPS_SMALL(..) => 252,
                I2C_ERRORS(..) => 253,
                DCF_THETA(..) => 254,
                SECURE_LINK_STATUS(..) => 255,
            }
        }
    } /* This file was automatically generated, do not edit */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AUTOPILOT_VERSION_DATA {
        /* version encoded as: MAJOR * 10000 + MINOR * 100 + PATCH*/
        version: u32,
        /* version description as string from paparazzi_version*/
        desc: Vec<u8>, /* unspecified */
    }
    /* alive/heartbeat message containing the MD5sum of the aircraft configuration */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ALIVE_DATA {
        md5sum: Vec<u8>, /* unspecified */
    }
    /* Answer to PING datalink message, to measure latencies */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct PONG_DATA {}
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TAKEOFF_DATA {
        cpu_time: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ARDRONE_NAVDATA_DATA {
        taille: u16,
        nu_trame: u16,
        ax: u16,
        ay: u16,
        az: u16,
        vx: i16,
        vy: i16,
        vz: i16,
        temperature_acc: u16,
        temperature_gyro: u16,
        ultrasound: u16,
        us_debut_echo: u16,
        us_fin_echo: u16,
        us_association_echo: u16,
        us_distance_echo: u16,
        us_curve_time: u16,
        us_curve_value: u16,
        us_curve_ref: u16,
        nb_echo: u16,
        sum_echo: u32,
        gradient: i16,
        flag_echo_ini: u16,
        pressure: i32,
        temperature_pressure: u16,
        mx: i16,
        my: i16,
        mz: i16,
        chksum: u16,
        checksum_errors: u32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ATTITUDE_DATA {
        phi: f32,
        psi: f32,
        theta: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IR_SENSORS_DATA {
        ir1: i16,
        ir2: i16,
        longitudinal: i16,
        lateral: i16,
        vertical: i16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GPS_DATA {
        mode: u8,
        utm_east: i32,
        utm_north: i32,
        course: i16,
        /* Altitude above geoid (MSL)*/
        alt: i32,
        /* norm of 2d ground speed in cm/s*/
        speed: u16,
        climb: i16,
        week: u16,
        itow: u32,
        utm_zone: u8,
        gps_nb_err: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct NAVIGATION_REF_DATA {
        utm_east: i32,
        utm_north: i32,
        utm_zone: u8,
        ground_alt: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct NAVIGATION_DATA {
        cur_block: u8,
        cur_stage: u8,
        pos_x: f32,
        pos_y: f32,
        dist_wp: f32,
        dist_home: f32,
        circle_count: u8,
        oval_count: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct PPRZ_MODE_DATA {
        ap_mode: u8,
        ap_gaz: u8,
        ap_lateral: u8,
        ap_horizontal: u8,
        if_calib_mode: u8,
        mcu1_status: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct BAT_DATA {
        throttle: i16,
        voltage: u16,
        amps: i16,
        flight_time: u16,
        kill_auto_throttle: u8,
        block_time: u16,
        stage_time: u16,
        energy: i16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DEBUG_MCU_LINK_DATA {
        i2c_nb_err: u8,
        i2c_mcu1_nb_err: u8,
        ppm_rate: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct CALIBRATION_DATA {
        climb_sum_err: f32,
        climb_gaz_submode: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SETTINGS_DATA {
        slider_1_val: f32,
        slider_2_val: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DESIRED_DATA {
        roll: f32,
        pitch: f32,
        course: f32,
        x: f32,
        y: f32,
        altitude: f32,
        climb: f32,
        airspeed: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GPS_SOL_DATA {
        Pacc: u32,
        Sacc: u32,
        PDOP: u16,
        numSV: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ADC_GENERIC_DATA {
        val1: u16,
        val2: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ECU_DATA {
        stg_in: u8,
        stb_in: u8,
        ain1: f32,
        ain2: f32,
        ain3: f32,
        ain4: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct CAM_DATA {
        pan: i16,
        tilt: i16,
        target_x: i16,
        target_y: i16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct CIRCLE_DATA {
        center_east: f32,
        center_north: f32,
        radius: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SEGMENT_DATA {
        segment_east_1: f32,
        segment_north_1: f32,
        segment_east_2: f32,
        segment_north_2: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct VECTORNAV_INFO_DATA {
        timestamp: f32,
        chksm_error: u32,
        hdr_error: u32,
        rate: u16,
        ins_status: u8,
        ins_err: u8,
        YprU1: f32,
        YprU2: f32,
        YprU3: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct HYBRID_GUIDANCE_DATA {
        pos_x: i32,
        pos_y: i32,
        speed_x: i32,
        speed_y: i32,
        wind_x: i32,
        wind_y: i32,
        pos_err_x: i32,
        pos_err_y: i32,
        speed_sp_x: i32,
        speed_sp_y: i32,
        norm_ref_speed: i32,
        heading_diff: i32,
        phi: i32,
        theta: i32,
        psi: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SVINFO_DATA {
        chn: u8,
        SVID: u8,
        Flags: u8,
        QI: u8,
        CNO: u8,
        Elev: i8,
        Azim: i16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DEBUG_DATA {
        msg: Vec<u8>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SURVEY_DATA {
        east: f32,
        north: f32,
        west: f32,
        south: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct RSSI_DATA {
        rssi: u8,
        tx_power: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct RANGEFINDER_DATA {
        range: u16,
        z_dot: f32,
        z_dot_sum_err: f32,
        z_dot_setpoint: f32,
        z_sum_err: f32,
        z_setpoint: f32,
        flying: u8,
    }
    /* Datalink status reported by an aircraft for the ground */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DATALINK_REPORT_DATA {
        uplink_lost_time: u16,
        uplink_nb_msgs: u16,
        downlink_nb_msgs: u16,
        downlink_rate: u16,
        uplink_rate: u16,
        downlink_ovrn: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DL_VALUE_DATA {
        index: u8,
        value: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MARK_DATA {
        ac_id: u8,
        lat: f32,
        long: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SYS_MON_DATA {
        periodic_time: u16,
        periodic_time_min: u16,
        periodic_time_max: u16,
        periodic_cycle: u16,
        periodic_cycle_min: u16,
        periodic_cycle_max: u16,
        event_number: u16,
        cpu_load: u8,
        cpu_time: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MOTOR_DATA {
        rpm: u16,
        current: i32,
    }
    /* Waypoint with id wp_id has been updated/moved to the specified UTM coordinates. */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct WP_MOVED_DATA {
        wp_id: u8,
        utm_east: f32,
        utm_north: f32,
        /* Height above Mean Sea Level (geoid)*/
        alt: f32,
        utm_zone: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MKK_DATA {
        nr: u8,
        rpm: u8,
        current: u8,
        temp: i8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ENERGY_DATA {
        bat: f32,
        amp: f32,
        energy: u16,
        power: f32,
    }
    /* Velocities in body axes (assuming small pitch/roll angles) as measured by the dragspeed module and by the INS. */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DRAGSPEED_DATA {
        /* Estimated velocity along body +x axis*/
        u_est: f32,
        /* Estimated velocity along body +y axis*/
        v_est: f32,
        /* INS velocity along body +x axis*/
        u_ins: f32,
        /* INS velocity along body +y axis*/
        v_ins: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct RSSI_COMBINED_DATA {
        remote_rssi: u8,
        tx_power: u8,
        local_rssi: u8,
        local_noise: u8,
        remote_noise: u8,
    }
    /* Telemetry message for monitoring the status of the Distributed Circular Formation. */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DCF_DATA {
        /*  The size of the array is 4 x (maximum number of possible neighbors). The elements per each neighbor are: 1. ID of the neighbor, 2. Theta of the neighbor (degrees x 100), 3. Desired inter-vehicle angle (degrees x 100), 4. Last time in ms we received a msg from the neighbor*/
        table: Vec<i16>, /* unspecified */
        /*  The size of the array is the maximum number of possible neighbors. Errors w.r.t. desired inter-vehicle angles (degrees x 100)*/
        errors: Vec<i16>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ALT_KALMAN_DATA {
        p00: f32,
        p01: f32,
        p10: f32,
        p11: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ESTIMATOR_DATA {
        z: f32,
        z_dot: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TUNE_ROLL_DATA {
        p: f32,
        phi: f32,
        phi_sp: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct BARO_MS5534A_DATA {
        pressure: u32,
        temp: u16,
        alt: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct BARO_WORDS_DATA {
        w1: u16,
        w2: u16,
        w3: u16,
        w4: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct WP_MOVED_LLA_DATA {
        wp_id: u8,
        lat: i32,
        lon: i32,
        /* Height above Mean Sea Level (geoid)*/
        alt: i32,
    }
    /* Relative localization data for other tracked MAVs in terms of x y and z in the body axis */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct RLFILTER_DATA {
        trackedID: i32,
        rangearray: f32,
        x_tracked: f32,
        y_tracked: f32,
        vx_own: f32,
        vy_own: f32,
        vx_tracked: f32,
        vy_tracked: f32,
        z_pos: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct WP_MOVED_ENU_DATA {
        wp_id: u8,
        east: i32,
        north: i32,
        up: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct WINDTURBINE_STATUS__DATA {
        ac_id: u8,
        tb_id: u8,
        sync_itow: u32,
        cycle_time: u32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct RC_3CH__DATA {
        throttle_mode: u8,
        roll: i8,
        pitch: i8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MPPT_DATA {
        values: Vec<i16>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DEBUG_IR_I2C_DATA {
        ir1: i16,
        ir2: i16,
        top: i16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AIRSPEED_DATA {
        airspeed: f32,
        airspeed_sp: f32,
        airspeed_cnt: f32,
        groundspeed_sp: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct BARO_ETS_DATA {
        adc: u16,
        offset: u16,
        scaled: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AIRSPEED_ETS_DATA {
        adc: u16,
        offset: u16,
        scaled: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct VISION_OUTBACK_DATA {
        status: u8,
        het_moment: u8,
        timeoutcount: u8,
        vision_timeout: u8,
        height: f32,
        out_of_range: f32,
        marker_enu_x: f32,
        marker_enu_y: f32,
        flow_x: f32,
        flow_y: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GPS_LLA_DATA {
        lat: i32,
        lon: i32,
        /* altitude above WGS84 reference ellipsoid*/
        alt: i32,
        /* Height above Mean Sea Level (geoid)*/
        hmsl: i32,
        course: i16,
        speed: u16,
        climb: i16,
        week: u16,
        itow: u32,
        mode: u8,
        gps_nb_err: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct H_CTL_A_DATA {
        roll_sum_err: f32,
        roll_sp: f32,
        roll_ref: f32,
        phi: f32,
        aileron_sp: i16,
        pitch_sum_err: f32,
        pitch_sp: f32,
        pitch_ref: f32,
        theta: f32,
        elevator_sp: i16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TURB_PRESSURE_VOLTAGE_DATA {
        ch_1_p: f32,
        ch_1_t: f32,
        ch_2_p: f32,
        ch_2_t: f32,
        ch_3_p: f32,
        ch_3_t: f32,
        ch_4_p: f32,
        ch_4_t: f32,
        ch_5_p: f32,
        ch_5_t: f32,
        ch_6_p: f32,
        ch_6_t: f32,
        ch_7_p: f32,
        ch_7_t: f32,
        gnd1: f32,
        gnd2: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct CAM_POINT_DATA {
        cam_point_distance_from_home: u16,
        cam_point_lat: f32,
        cam_point_lon: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DC_INFO_DATA {
        mode: i16,
        lat: i32,
        lon: i32,
        /* altitude above WGS84 reference ellipsoid*/
        alt: i32,
        course: f32,
        photo_nr: u16,
        dist: f32,
        next_dist: f32,
        start_x: f32,
        start_y: f32,
        start_angle: f32,
        angle: f32,
        last_block: f32,
        count: u16,
        shutter: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AMSYS_BARO_DATA {
        pBaroRaw: u16,
        pBaro: f32,
        pHomePressure: f32,
        AltOffset: f32,
        aktuell: f32,
        Over_Ground: f32,
        tempBaro: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AMSYS_AIRSPEED_DATA {
        asRaw: u16,
        asPresure: f32,
        asAirspeed: f32,
        asAirsFilt: f32,
        asTemp: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct FLIGHT_BENCHMARK_DATA {
        SE_As: f32,
        SE_Alt: f32,
        SE_Pos: f32,
        Err_As: f32,
        Err_Alt: f32,
        Err_Pos: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MPL3115_BARO_DATA {
        pressure: u32,
        temp: i16,
        alt: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AOA_DATA {
        raw: u32,
        angle: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct XTEND_RSSI_DATA {
        datalink_time: u16,
        rssi_fade_margin: u8,
        duty: u8,
    }
    /* Information about the trajectory followed by the Guidance Vector Field algorithm. */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GVF_DATA {
        /* Error index e, i.e. 'distance' to the trajectory*/
        error: f32,
        /* Kind of trajectory*/
        traj: u8,
        /* Direction to be followed*/
        s: i8,
        /* Gain for the vector field convergence*/
        ke: f32,
        /* Parameters describing the trajectory*/
        p: Vec<f32>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SUPERBITRF_DATA {
        status: u8,
        cyrf_status: u8,
        irq_count: u32,
        rx_packet_count: u32,
        tx_packet_count: u32,
        transfer_timeouts: u32,
        resync_count: u32,
        uplink_count: u32,
        rc_count: u32,
        timing1: u32,
        timing2: u32,
        bind_mfg_id: u32,
        mfg_id: Vec<u8>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GX3_INFO_DATA {
        GX3_freq: f32,
        chksm_error: u32,
        hdr_error: u32,
        GX3_chksm: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct UBLOX_INFO_DATA {
        baud: u32,
        ver_sw_h: u8,
        ver_sw_l: u8,
        ver_hw_h: u16,
        ver_hw_l: u16,
        sbas: u8,
        gnss: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct INV_FILTER_DATA {
        quat: f32,
        phi_inv: f32,
        theta_inv: f32,
        psi_inv: f32,
        Vx_inv: f32,
        Vy_inv: f32,
        Vz_inv: f32,
        Px_inv: f32,
        Py_inv: f32,
        Pz_inv: f32,
        bias_phi: f32,
        bias_theta: f32,
        bias_psi: f32,
        bias_as: f32,
        bias_hb: f32,
        meas_baro: f32,
        meas_gps: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MISSION_STATUS_DATA {
        remaining_time: f32,
        index_list: Vec<u8>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GENERIC_COM_DATA {
        lat: i32,
        lon: i32,
        alt: i16,
        gspeed: u16,
        course: i16,
        airspeed: u16,
        vsupply: u8,
        energy: u8,
        throttle: u8,
        ap_mode: u8,
        nav_block: u8,
        flight_time: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct FORMATION_SLOT_TM_DATA {
        ac_id: u8,
        mode: u8,
        slot_east: f32,
        slot_north: f32,
        slot_alt: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct FORMATION_STATUS_TM_DATA {
        ac_id: u8,
        leader_id: u8,
        status: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct BMP_STATUS_DATA {
        UP: i32,
        UT: i32,
        press: i32,
        temp: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MLX_STATUS_DATA {
        itemp_case: u16,
        temp_case: f32,
        itemp_obj: u16,
        temp_obj: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TMP_STATUS_DATA {
        itemp: u16,
        temp: f32,
    }
    /* Wind information returned to the ground station. The wind is reported as a vector, it gives the direction the wind is blowing to. This can be used to acknowledge data comming from the ground wind estimator or from an embedded algorithm. Flags field definition: - bit 0: horizontal wind is valid (east and north fields) - bit 1: vertical wind is valid (up field) - bit 2: airspeed is valid */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct WIND_INFO_RET_DATA {
        /* bit 0: horizontal wind, bit 1: vertical wind: bit 2: airspeed*/
        flags: u8,
        /* east component of the wind*/
        east: f32,
        /* north component of the wind*/
        north: f32,
        /* vertical component of the wind*/
        up: f32,
        /* local airspeed norm*/
        airspeed: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SCP_STATUS_DATA {
        press: u32,
        temp: i16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SHT_STATUS_DATA {
        ihumid: u16,
        itemp: u16,
        humid: f32,
        temp: f32,
    }
    /* Generic message to send position measurement from computer vision */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct VISION_POSITION_ESTIMATE_DATA {
        /* Counter*/
        cnt: u16,
        /* X*/
        x: f32,
        /* Y*/
        y: f32,
        /* Z*/
        z: f32,
        /* Detection Quality or Uncertainty*/
        quality: f32,
        /* In case the default message does not suit you*/
        extra: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DPICCO_STATUS_DATA {
        humid: u16,
        temp: u16,
        fhumid: f32,
        ftemp: f32,
    }
    /* Logger status and error id (dependent of the logging system) */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct LOGGER_STATUS_DATA {
        /* General status of the logger*/
        status: u8,
        /* Error number, depend of the logging system, provides detailed information*/
        errno: u8,
        /* Accumulated number of bytes written by the logger*/
        used: u32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MOTOR_BENCH_STATUS_DATA {
        time_ticks: u32,
        throttle: f32,
        rpm: f32,
        current: f32,
        thrust: f32,
        torque: f32,
        time_s: u16,
        mode: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct HIH_STATUS_DATA {
        humid: u16,
        fhumid: f32,
        ftemp: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TEMT_STATUS_DATA {
        light: u16,
        f_light: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GP2Y_STATUS_DATA {
        idensity: u16,
        density: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SHT_I2C_SERIAL_DATA {
        serial0: u32,
        serial1: u32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct PPM_DATA {
        ppm_rate: u8,
        values: Vec<u16>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct RC_DATA {
        values: Vec<i16>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct COMMANDS_DATA {
        values: Vec<i16>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct FBW_STATUS_DATA {
        rc_status: u8,
        frame_rate: u8,
        mode: u8,
        vsupply: u16,
        current: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ADC_DATA {
        mcu: u8,
        values: Vec<u16>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ACTUATORS_DATA {
        values: Vec<i16>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct BLUEGIGA_DATA {
        data_rate: u32,
        A2A_msg_rate: u32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct THROTTLE_CURVE_DATA {
        curve: u8,
        throttle: u16,
        collective: i16,
        rpm_sp: u16,
        rpm_meas: u16,
        rpm_err_sum: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct PIKSI_HEARTBEAT_DATA {
        heartbeat: u32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MULTIGAZE_METERS_DATA {
        multigaze_meters: Vec<f32>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DC_SHOT_DATA {
        photo_nr: i16,
        /* Gedetic latitude*/
        lat: i32,
        /* Longitude*/
        lon: i32,
        /* altitude above WGS84 reference ellipsoid*/
        alt: i32,
        /* Height above Mean Sea Level (geoid)*/
        hmsl: i32,
        /* Euler angle around x-axis (roll)*/
        phi: i16,
        /* Euler angle around y-axis (pitch)*/
        theta: i16,
        /* Euler angle around z-axis (yaw)*/
        psi: i16,
        /* Course over ground (CW/north)*/
        course: i16,
        /* horizontal ground speed*/
        speed: u16,
        /* GPS time of week*/
        itow: u32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct CAMERA_PAYLOAD_DATA {
        /* Payload computer seconds since startup*/
        timestamp: f32,
        /* Percentage of used memory (RAM) of the payload computer rounded up to whole percent*/
        used_memory: u8,
        /* Percentage of used disk of the payload computer rounded up to whole percent*/
        used_disk: u8,
        /* Payload door open/close*/
        door_status: u8,
        /* Error codes of the payload*/
        error_code: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MOTOR_MIXING_DATA {
        values: Vec<i16>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MLX_SERIAL_DATA {
        serial0: u32,
        serial1: u32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct PAYLOAD_DATA {
        values: Vec<u8>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct HTM_STATUS_DATA {
        ihumid: u16,
        itemp: u16,
        humid: f32,
        temp: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct BARO_MS5611_DATA {
        d1: u32,
        d2: u32,
        pressure: f32,
        temp: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MS5611_COEFF_DATA {
        c0: u16,
        c1: u16,
        c2: u16,
        c3: u16,
        c4: u16,
        c5: u16,
        c6: u16,
        c7: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ATMOSPHERE_CHARGE_DATA {
        t0: u16,
        t1: u16,
        t2: u16,
        t3: u16,
        t4: u16,
        t5: u16,
        t6: u16,
        t7: u16,
        t8: u16,
        t9: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SOLAR_RADIATION_DATA {
        up_t0: u16,
        dn_t0: u16,
        up_t1: u16,
        dn_t1: u16,
        up_t2: u16,
        dn_t2: u16,
        up_t3: u16,
        dn_t3: u16,
        up_t4: u16,
        dn_t4: u16,
        up_t5: u16,
        dn_t5: u16,
        up_t6: u16,
        dn_t6: u16,
        up_t7: u16,
        dn_t7: u16,
        up_t8: u16,
        dn_t8: u16,
        up_t9: u16,
        dn_t9: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TCAS_TA_DATA {
        ac_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TCAS_RA_DATA {
        ac_id: u8,
        resolve: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TCAS_RESOLVED_DATA {
        ac_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TCAS_DEBUG_DATA {
        ac_id: u8,
        tau: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct POTENTIAL_DATA {
        east: f32,
        north: f32,
        alt: f32,
        speed: f32,
        climb: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct COPILOT_STATUS_DATA {
        /* Mission computer seconds since startup*/
        timestamp: f32,
        /* Percentage of used memory (RAM) of the mission computer rounded up to whole percent*/
        used_memory: u8,
        /* Percentage of used disk of the mission computer rounded up to whole percent*/
        used_disk: u8,
        /* Mission computer status*/
        status: u8,
        /* Error codes of the mission computer*/
        error_code: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TEMP_TCOUPLE_DATA {
        fval0: f32,
        fval1: f32,
        fval2: f32,
        fval3: f32,
        fref0: f32,
        fref1: f32,
        fref2: f32,
        fref3: f32,
        val0: u16,
        val1: u16,
        val2: u16,
        val3: u16,
        ref0: u16,
        ref1: u16,
        ref2: u16,
        ref3: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SHT_I2C_STATUS_DATA {
        ihumid: u16,
        itemp: u16,
        humid: f32,
        temp: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct CAMERA_SNAPSHOT_DATA {
        /* Unique camera ID - consists of make,model and camera index*/
        camera_id: u16,
        /* State of the given camera*/
        camera_state: u8,
        /* Snapshot number in sequence*/
        snapshot_image_number: u16,
        /* Flag checking whether the last snapshot was valid*/
        snapshot_valid: u8,
        /* Lens temperature, NaN if not measured*/
        lens_temp: f32,
        /* Imager sensor temperature, NaN if not measured*/
        array_temp: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TIMESTAMP_DATA {
        timestamp: u32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct STAB_ATTITUDE_FLOAT_DATA {
        est_p: f32,
        est_q: f32,
        est_r: f32,
        est_phi: f32,
        est_theta: f32,
        est_psi: f32,
        ref_phi: f32,
        ref_theta: f32,
        ref_psi: f32,
        sum_err_phi: f32,
        sum_err_theta: f32,
        sum_err_psi: f32,
        delta_a_fb: f32,
        delta_e_fb: f32,
        delta_r_fb: f32,
        delta_a_ff: f32,
        delta_e_ff: f32,
        delta_r_ff: f32,
        delta_a: i32,
        delta_e: i32,
        delta_r: i32,
        est_p_d: f32,
        est_q_d: f32,
        est_r_d: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMU_GYRO_SCALED_DATA {
        gp: i32,
        gq: i32,
        gr: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMU_ACCEL_SCALED_DATA {
        ax: i32,
        ay: i32,
        az: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMU_MAG_SCALED_DATA {
        mx: i32,
        my: i32,
        mz: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct FILTER_DATA {
        phi: i32,
        theta: i32,
        psi: i32,
        measure_phi: i32,
        measure_theta: i32,
        measure_psi: i32,
        corrected_phi: i32,
        corrected_theta: i32,
        corrected_psi: i32,
        correction_phi: i32,
        correction_theta: i32,
        correction_psi: i32,
        bp: i32,
        bq: i32,
        br: i32,
        comp_id: u8,
    }
    /* Rotorcraft rate control loop. */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct RATE_LOOP_DATA {
        /* rate setpoint*/
        sp_p: f32,
        /* rate setpoint*/
        sp_q: f32,
        /* rate setpoint*/
        sp_r: f32,
        /* integrated quaternion error*/
        sumerr_p: f32,
        /* integrated quaternion error*/
        sumerr_q: f32,
        /* integrated quaternion error*/
        sumerr_r: f32,
        /* feedback command on pitch (pprz scale)*/
        fb_p: f32,
        /* feedback command on roll  (pprz scale)*/
        fb_q: f32,
        /* feedback command on yaw   (pprz scale)*/
        fb_r: f32,
        /* thrust command*/
        delta_t: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct FILTER_ALIGNER_DATA {
        lp_gp: i32,
        lp_gq: i32,
        lp_gr: i32,
        gp: i32,
        gq: i32,
        gr: i32,
        noise: i32,
        cnt: i32,
        status: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AIRSPEED_MS45XX_DATA {
        diffPress: f32,
        temperature: i16,
        airspeed: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct STAB_ATTITUDE_INT_DATA {
        est_p: i32,
        est_q: i32,
        est_r: i32,
        est_phi: i32,
        est_theta: i32,
        est_psi: i32,
        sp_phi: i32,
        sp_theta: i32,
        sp_psi: i32,
        sum_err_phi: i32,
        sum_err_theta: i32,
        sum_err_psi: i32,
        delta_a_fb: i32,
        delta_e_fb: i32,
        delta_r_fb: i32,
        delta_a_ff: i32,
        delta_e_ff: i32,
        delta_r_ff: i32,
        delta_a: i32,
        delta_e: i32,
        delta_r: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct STAB_ATTITUDE_REF_INT_DATA {
        sp_phi: i32,
        sp_theta: i32,
        sp_psi: i32,
        ref_phi: i32,
        ref_theta: i32,
        ref_psi: i32,
        ref_p: i32,
        ref_q: i32,
        ref_r: i32,
        ref_pd: i32,
        ref_qd: i32,
        ref_rd: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct STAB_ATTITUDE_REF_FLOAT_DATA {
        sp_phi: f32,
        sp_theta: f32,
        sp_psi: f32,
        ref_phi: f32,
        ref_theta: f32,
        ref_psi: f32,
        ref_p: f32,
        ref_q: f32,
        ref_r: f32,
        ref_pd: f32,
        ref_qd: f32,
        ref_rd: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ROTORCRAFT_CMD_DATA {
        cmd_roll: i32,
        cmd_pitch: i32,
        cmd_yaw: i32,
        cmd_thrust: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GUIDANCE_H_INT_DATA {
        sp_x: i32,
        sp_y: i32,
        ref_x: i32,
        ref_y: i32,
        est_x: i32,
        est_y: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct VERT_LOOP_DATA {
        z_sp: i32,
        zd_sp: i32,
        est_z: i32,
        est_zd: i32,
        est_zdd: i32,
        ref_z: i32,
        ref_zd: i32,
        ref_zdd: i32,
        adp_inv_m: i32,
        adp_cov: i32,
        adp_meas: i32,
        sum_err: i32,
        ff_cmd: i32,
        fb_cmd: i32,
        delta_t: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct HOVER_LOOP_DATA {
        sp_x: i32,
        sp_y: i32,
        est_x: i32,
        est_y: i32,
        est_xd: i32,
        est_yd: i32,
        est_xdd: i32,
        est_ydd: i32,
        err_x: i32,
        err_y: i32,
        err_xd: i32,
        err_yd: i32,
        err_sum_x: i32,
        err_sum_y: i32,
        cmd_x: i32,
        cmd_y: i32,
        cmd_heading: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ROTORCRAFT_FP_DATA {
        east: i32,
        north: i32,
        up: i32,
        veast: i32,
        vnorth: i32,
        vup: i32,
        phi: i32,
        theta: i32,
        psi: i32,
        carrot_east: i32,
        carrot_north: i32,
        carrot_up: i32,
        carrot_psi: i32,
        thrust: i32,
        flight_time: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TEMP_ADC_DATA {
        temp1: f32,
        temp2: f32,
        temp3: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GUIDANCE_H_REF_INT_DATA {
        sp_x: i32,
        ref_x: i32,
        sp_xd: i32,
        ref_xd: i32,
        ref_xdd: i32,
        sp_y: i32,
        ref_y: i32,
        sp_yd: i32,
        ref_yd: i32,
        ref_ydd: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ROTORCRAFT_TUNE_HOVER_DATA {
        rc_roll: i16,
        rc_pitch: i16,
        rc_yaw: i16,
        cmd_roll: i32,
        cmd_pitch: i32,
        cmd_yaw: i32,
        cmd_thrust: i32,
        body_phi: i32,
        body_theta: i32,
        body_psi: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct INS_Z_DATA {
        baro_z: f32,
        ins_z: i32,
        ins_zd: i32,
        ins_zdd: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct PCAP01_STATUS_DATA {
        ihumid: u32,
        itemp: u32,
        humid: f32,
        temp: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GEIGER_COUNTER_DATA {
        tube1: u32,
        tube2: u32,
        vsupply: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct INS_REF_DATA {
        ecef_x0: i32,
        ecef_y0: i32,
        ecef_z0: i32,
        lat0: i32,
        lon0: i32,
        alt0: i32,
        hmsl0: i32,
        baro_qfe: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GPS_INT_DATA {
        ecef_x: i32,
        ecef_y: i32,
        ecef_z: i32,
        lat: i32,
        lon: i32,
        /* altitude above WGS84 reference ellipsoid*/
        alt: i32,
        /* height above mean sea level (geoid)*/
        hmsl: i32,
        ecef_xd: i32,
        ecef_yd: i32,
        ecef_zd: i32,
        pacc: u32,
        sacc: u32,
        tow: u32,
        pdop: u16,
        numsv: u8,
        fix: u8,
        comp_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AHRS_EULER_INT_DATA {
        imu_phi: i32,
        imu_theta: i32,
        imu_psi: i32,
        body_phi: i32,
        body_theta: i32,
        body_psi: i32,
        comp_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AHRS_QUAT_INT_DATA {
        weight: f32,
        imu_qi: i32,
        imu_qx: i32,
        imu_qy: i32,
        imu_qz: i32,
        body_qi: i32,
        body_qx: i32,
        body_qy: i32,
        body_qz: i32,
        comp_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ROTORCRAFT_NAV_STATUS_DATA {
        block_time: u16,
        stage_time: u16,
        dist_home: f32,
        dist_wp: f32,
        cur_block: u8,
        cur_stage: u8,
        horizontal_mode: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ROTORCRAFT_RADIO_CONTROL_DATA {
        roll: i16,
        pitch: i16,
        yaw: i16,
        throttle: i16,
        mode: i16,
        kill: i16,
        status: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct VFF_EXTENDED_DATA {
        meas_baro: f32,
        meas_alt: f32,
        z: f32,
        zd: f32,
        zdd: f32,
        bias: f32,
        offset: f32,
        obs_height: f32,
        Pzz: f32,
        Pzdzd: f32,
        Pbb: f32,
        Poffsetoffset: f32,
        Pobsobs: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct VFF_DATA {
        measure: f32,
        z: f32,
        zd: f32,
        bias: f32,
        Pzz: f32,
        Pzdzd: f32,
        Pbb: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GEO_MAG_DATA {
        Hx: f32,
        Hy: f32,
        Hz: f32,
        comp_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct HFF_DATA {
        x: f32,
        y: f32,
        xd: f32,
        yd: f32,
        xdd: f32,
        ydd: f32,
        xbias: f32,
        ybias: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct HFF_DBG_DATA {
        x_measure: f32,
        y_measure: f32,
        xd_measure: f32,
        yd_measure: f32,
        Pxx: f32,
        Pyy: f32,
        Pxdxd: f32,
        Pydyd: f32,
        Pxbiasxbias: f32,
        Pybiasybias: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct HFF_GPS_DATA {
        lag_cnt: u16,
        lag_cnt_err: i16,
        save_cnt: i16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ROTORCRAFT_CAM_DATA {
        tilt: i16,
        pan: i16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AHRS_REF_QUAT_DATA {
        ref_qi: i32,
        ref_qx: i32,
        ref_qy: i32,
        ref_qz: i32,
        body_qi: i32,
        body_qx: i32,
        body_qy: i32,
        body_qz: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AHRS_EULER_DATA {
        phi: f32,
        theta: f32,
        psi: f32,
        comp_id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AHRS_MEASUREMENT_EULER_DATA {
        phi: f32,
        theta: f32,
        psi: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct WT_DATA {
        rpm: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct CSC_CAN_DEBUG_DATA {
        err_nb: u32,
        err_code: u32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct CSC_CAN_MSG_DATA {
        frame: u32,
        id: u32,
        data_a: u32,
        data_b: u32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AHRS_GYRO_BIAS_INT_DATA {
        bp: i32,
        bq: i32,
        br: i32,
        comp_id: u8,
    }
    /* Airflow data returned by OTF and uADC 3D probes from Aeroprobe. */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AEROPROBE_DATA {
        counter: u32,
        velocity: i16,
        a_attack: i16,
        a_sideslip: i16,
        altitude: i32,
        dynamic_p: i32,
        static_p: i32,
        checksum: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct FMS_TIME_DATA {
        tv_sec: u32,
        tv_nsec: u32,
        delay_ns: u32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AHRS_LKF_DATA {
        phi: f32,
        theta: f32,
        psi: f32,
        qi: f32,
        qx: f32,
        qy: f32,
        qz: f32,
        p: f32,
        q: f32,
        r: f32,
        ax: f32,
        ay: f32,
        az: f32,
        mx: f32,
        my: f32,
        mz: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct NPS_SENSORS_SCALED_DATA {
        acc_x: f32,
        acc_y: f32,
        acc_z: f32,
        mag_x: f32,
        mag_y: f32,
        mag_z: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct INS_DATA {
        ins_x: i32,
        ins_y: i32,
        ins_z: i32,
        ins_xd: i32,
        ins_yd: i32,
        ins_zd: i32,
        ins_xdd: i32,
        ins_ydd: i32,
        ins_zdd: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMU_GYRO_DATA {
        gp: f32,
        gq: f32,
        gr: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMU_MAG_DATA {
        mx: f32,
        my: f32,
        mz: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMU_ACCEL_DATA {
        ax: f32,
        ay: f32,
        az: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMU_GYRO_RAW_DATA {
        gp: i32,
        gq: i32,
        gr: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMU_ACCEL_RAW_DATA {
        ax: i32,
        ay: i32,
        az: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMU_MAG_RAW_DATA {
        mx: i32,
        my: i32,
        mz: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMU_MAG_SETTINGS_DATA {
        inclination: f32,
        declination: f32,
        hardiron_x: f32,
        hardiron_y: f32,
        hardiron_z: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMU_MAG_CURRENT_CALIBRATION_DATA {
        mx: i32,
        my: i32,
        mz: i32,
        electrical_current: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct UART_ERRORS_DATA {
        overrun_cnt: u16,
        noise_err_cnt: u16,
        framing_err_cnt: u16,
        bus_number: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMU_GYRO_LP_DATA {
        gp: f32,
        gq: f32,
        gr: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMU_PRESSURE_DATA {
        p: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TUNE_VERT_DATA {
        z_sp: i32,
        est_z: i32,
        ref_z: i32,
        ref_zd: i32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct MF_DAQ_STATE_DATA {
        flight_time: u16,
        p: f32,
        q: f32,
        r: f32,
        phi: f32,
        theta: f32,
        psi: f32,
        ax: f32,
        ay: f32,
        az: f32,
        ve: f32,
        vn: f32,
        vu: f32,
        lat: f32,
        lon: f32,
        alt: f32,
        we: f32,
        wn: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct INFO_MSG_DATA {
        msg: Vec<u8>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct STAB_ATTITUDE_INDI_DATA {
        angular_accel_p: f32,
        angular_accel_q: f32,
        angular_accel_r: f32,
        angular_accel_ref_p: f32,
        angular_accel_ref_q: f32,
        angular_accel_ref_r: f32,
        g1_p: f32,
        g1_q: f32,
        g1_r: f32,
        g2_r: f32,
    }
    /* Minimalistic message to track Rotorcraft over very low bandwidth links */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ROTORCRAFT_FP_MIN_DATA {
        east: i32,
        north: i32,
        up: i32,
        gspeed: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct BEBOP_ACTUATORS_DATA {
        cmd_thrust: i32,
        cmd_roll: i32,
        cmd_pitch: i32,
        cmd_yaw: i32,
        rpm_ref_lf: u16,
        rpm_ref_rf: u16,
        rpm_ref_rb: u16,
        rpm_ref_lb: u16,
        rpm_obs_lf: u16,
        rpm_obs_rf: u16,
        rpm_obs_rb: u16,
        rpm_obs_lb: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct WEATHER_DATA {
        p_amb: f32,
        t_amb: f32,
        windspeed: f32,
        wind_from: f32,
        humidity: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct IMU_TURNTABLE_DATA {
        omega: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct BARO_RAW_DATA {
        abs: f32,
        diff: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AIR_DATA_DATA {
        /* static pressure*/
        pressure: f32,
        /* differential pressure*/
        diff_p: f32,
        /* air temperature*/
        temp: f32,
        /* barometric pressure adjusted to sea level*/
        qnh: f32,
        /* barometric altitude above mean sea level*/
        amsl_baro: f32,
        /* Equivalent Air Speed (or Calibrated Air Speed at low speed/altitude)*/
        airspeed: f32,
        /* True Air Speed (when P, T and P_diff are available)*/
        tas: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct AMSL_DATA {
        AMSL_BARO: f32,
        AMSL_GPS: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DIVERGENCE_DATA {
        /*  vertical velocity / height from optitrack (unit = 1/sec)*/
        divergence: f32,
        /*  vertical velocity / height from vision (unit = 1/sec)*/
        divergence_vision: f32,
        /*  thrust / max thrust paparazzi (-)*/
        normalized_thrust: f32,
        /*  covariance of divergence and thrust, or past divergence depending on the mode (-)*/
        cov_div: f32,
        /*  gain state in adaptive gain control: indicative of height (-) */
        pstate: f32,
        /*  gain used for control, includes the effect of the p-gain of adaptive control (-) */
        pused: f32,
        /*  measurement from the sonar (mm)*/
        sonar: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct VIDEO_SYNC_DATA {
        id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct PERIODIC_TELEMETRY_ERR_DATA {
        process: u8,
        mode: u8,
        id: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TIME_DATA {
        /* an integral value representing the number of seconds elapsed since 00:00 hours, Jan 1, 1970 UTC*/
        t: u32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct OPTIC_FLOW_EST_DATA {
        fps: f32,
        corner_cnt: u16,
        tracked_cnt: u16,
        flow_x: i16,
        flow_y: i16,
        flow_der_x: i16,
        flow_der_y: i16,
        vel_x: f32,
        vel_y: f32,
        vel_z: f32,
        div_size: f32,
        surface_roughness: f32,
        divergence: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct STEREO_IMG_DATA {
        fieldtype: u8,
        width: u8,
        height: u8,
        package_nb: u8,
        image_data: Vec<u8>, /* unspecified */
    }
    /* Rover status message */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ROVER_STATUS_DATA {
        rc_status: u8,
        frame_rate: u8,
        gps_status: u8,
        /* Rover AP modes are generated from XML file*/
        ap_mode: u8,
        ap_motors_on: u8,
        vsupply: u16,
        cpu_time: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ROTORCRAFT_STATUS_DATA {
        link_imu_nb_err: u32,
        motor_nb_err: u8,
        rc_status: u8,
        frame_rate: u8,
        gps_status: u8,
        ap_mode: u8,
        ap_in_flight: u8,
        ap_motors_on: u8,
        arming_status: u8,
        ap_h_mode: u8,
        ap_v_mode: u8,
        vsupply: u16,
        cpu_time: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct STATE_FILTER_STATUS_DATA {
        id: u8,
        state_filter_mode: u8,
        value: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct PX4FLOW_DATA {
        time_sec: f32,
        sensor_id: u8,
        flow_x: i16,
        flow_y: i16,
        flow_comp_m_x: f32,
        flow_comp_m_y: f32,
        quality: u8,
        ground_distance: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct LIDAR_DATA {
        distance: f32,
        status: u8,
        trans_status: u8,
    }
    /* Generic message with pixel coordinates of detected targets */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct VISUALTARGET_DATA {
        /* Counter*/
        cnt: u16,
        /* Center pixel X*/
        x: i16,
        /* Center pixel X*/
        y: i16,
        /* Width in pixels*/
        w: i16,
        /* height in pixels*/
        h: i16,
        /* In case many detectors run, which one did find this target*/
        source: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SONAR_DATA {
        sonar_meas: u16,
        sonar_distance: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct PAYLOAD_FLOAT_DATA {
        values: Vec<f32>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct NPS_POS_LLH_DATA {
        pprz_lat: f32,
        lat_geod: f32,
        lat_geoc: f32,
        pprz_lon: f32,
        lon: f32,
        pprz_alt: f32,
        alt_geod: f32,
        agl: f32,
        asl: f32,
    }
    /* Message for key exchange during crypto initialization */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct KEY_EXCHANGE_UAV_DATA {
        msg_type: u8,
        msg_data: Vec<u8>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct NPS_SPEED_POS_DATA {
        ltpp_xdd: f32,
        ltpp_ydd: f32,
        ltpp_zdd: f32,
        ltpp_xd: f32,
        ltpp_yd: f32,
        ltpp_zd: f32,
        ltpp_x: f32,
        ltpp_y: f32,
        ltpp_z: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct NPS_RATE_ATTITUDE_DATA {
        p: f32,
        q: f32,
        r: f32,
        phi: f32,
        theta: f32,
        psi: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct NPS_GYRO_BIAS_DATA {
        bp: f32,
        bq: f32,
        br: f32,
    }
    /* This messages includes the messages send by the Optical Flow Hover module, providing data for all three axes. */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct OPTICAL_FLOW_HOVER_DATA {
        /*  Filtered Horizontal velocity in X / height used in Optical Flow Hover*/
        flowX: f32,
        /*  Filtered Horizontal velocity in X / height used in Optical Flow Hover*/
        flowY: f32,
        /*  Filtered vertical velocity / height used in Optical Flow Hover*/
        Divergence: f32,
        /*  covariance of Ventral flow in X and past ventral flow in X*/
        cov_flowX: f32,
        /*  covariance of Ventral flow in Y and past ventral flow in Y*/
        cov_flowY: f32,
        /*  covariance of divergence and past divergence or thrust history*/
        cov_divZ: f32,
        /*  Gain used in the X feedback*/
        pusedX: f32,
        /*  Gain used in the Y feedback*/
        pusedY: f32,
        /*  Gain used in the Z feedback*/
        pusedZ: f32,
        /*  Sum of errors in X*/
        sum_errX: f32,
        /*  Sum of errors in Y*/
        sum_errY: f32,
        /*  Sum of errors in Z*/
        sum_errZ: f32,
        /*  The thrust*/
        thrust: i32,
        /*  The desired phi angle in degrees*/
        phi_des: f32,
        /*  The desired theta angle in degrees*/
        theta_des: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct NPS_WIND_DATA {
        vx: f32,
        vy: f32,
        vz: f32,
    }
    /* Electronic Speed Controller data */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct ESC_DATA {
        /* Current consumption*/
        amps: f32,
        /* Input battery voltage*/
        bat_volts: f32,
        /* Electrical power*/
        power: f32,
        /* Motor rotation speed*/
        rpm: f32,
        /* Motor voltage*/
        motor_volts: f32,
        /* Accumulated consumed energy*/
        energy: f32,
        /* Motor ID*/
        motor_id: u8,
    }
    /* RTOS monitoring */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct RTOS_MON_DATA {
        /* Number of running threads*/
        nb_threads: u8,
        /* Global CPU load*/
        cpu_load: u8,
        /* Core free memory*/
        core_free: u32,
        /* Total fragmented free memory in the heap*/
        heap_free: u32,
        /* Number of fragments in the heap*/
        heap_frags: u32,
        /* Largest free block in the heap*/
        heap_largest: u32,
        cpu_time: f32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct PPRZ_DEBUG_DATA {
        module: u8,
        errno: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct BATTERY_MONITOR_DATA {
        /* Battery monitor version*/
        version: u8,
        /* Bus monitor status*/
        bus_dev_stat: u8,
        /* Bus monitor i2c transaction status*/
        bus_trans_stat: u8,
        /* Raw bus current reading*/
        current_raw: u16,
        /* Bus current*/
        current: f32,
        /* Bus voltage*/
        bus_voltage: u16,
        /* Raw temperature reading*/
        bus_temp_raw: Vec<u16>, /* unspecified */
        /* Battery temperatures*/
        bus_temp: Vec<f32>, /* unspecified */
        /* Cell bank 1 monitor i2c transaction status*/
        bal1_trans_stat: u8,
        /* Cell bank 1 voltage*/
        bal1_cells: Vec<u16>, /* unspecified */
        /* Cell bank 2 monitor i2c transaction status*/
        bal2_trans_stat: u8,
        /* Cell bank 2 voltage*/
        bal2_cells: Vec<u16>, /* unspecified */
        /* Safety plug voltage*/
        safety_plug: u16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GPS_RXMRTCM_DATA {
        Msg1005: u32,
        Msg1077: u32,
        Msg1087: u32,
        Crc1005: u32,
        Crc1077: u32,
        Crc1087: u32,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct INDI_G_DATA {
        G1_roll: Vec<f32>, /* unspecified */
        G1_pitch: Vec<f32>, /* unspecified */
        G1_yaw: Vec<f32>, /* unspecified */
        G1_thrust: Vec<f32>, /* unspecified */
        G2: Vec<f32>, /* unspecified */
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GPS_RTK_DATA {
        iTOW: u32,
        refStationId: u16,
        relPosN: i32,
        relPosE: i32,
        relPosD: i32,
        relPosHPN: i8,
        relPosHPE: i8,
        relPosHPD: i8,
        accN: u32,
        accE: u32,
        accD: u32,
        carrSoln: u8,
        relPosValid: u8,
        diffSoln: u8,
        gnssFixOK: u8,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct GPS_SMALL_DATA {
        /* bits 31-21 course in decideg : bits 20-10 ground speed in cm/s : bits 9-0 climb speed in cm/s*/
        multiplex_speed: u32,
        lat: i32,
        lon: i32,
        /* height above the ellipsoid*/
        alt: i16,
    }
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct I2C_ERRORS_DATA {
        wd_reset_cnt: u16,
        queue_full_cnt: u16,
        acknowledge_failure_cnt: u16,
        misplaced_start_or_stop_cnt: u16,
        arbitration_lost_cnt: u16,
        overrun_or_underrun_cnt: u16,
        pec_error_in_reception_cnt: u16,
        timeout_or_tlow_error_cnt: u16,
        smbus_alert_cnt: u16,
        unexpected_event_cnt: u16,
        last_unexpected_event: u32,
        bus_number: u8,
    }
    /* Air-to-air message for the Distributed Circular Formation algorithm. It transmits the ac's theta to its neighbor */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct DCF_THETA_DATA {
        theta: f32,
    }
    /* Message for monitoring key exchange status */
    #[derive(Clone, PartialEq, Default)]
    #[derive(Serialize, Deserialize)]
    pub struct SECURE_LINK_STATUS_DATA {
        sts_stage: u8,
        sts_error: u8,
        counter_err: u32,
        decrypt_err: u32,
        encrypt_err: u32,
    }
}
