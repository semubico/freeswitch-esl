use std::collections::HashMap;
use compact_str::CompactString;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "Event-Name", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EslEvent {

    Heartbeat {
        #[serde(rename="Max-Sessions")]
        max_sessions: CompactString,
        #[serde(rename="FreeSWITCH-Version")]
        freeswitch_version: CompactString,
        #[serde(rename="Up-Time")]
        up_time: CompactString,
        #[serde(rename="Session-Peak-Max")]
        session_peak_max: CompactString,
        #[serde(rename="Session-Peak-FiveMin")]
        session_peak_fivemin: CompactString,
        #[serde(rename="Session-Count")]
        session_count: CompactString,
        #[serde(rename="Session-Per-Sec")]
        session_per_sec: CompactString,
        #[serde(rename="Session-Per-Sec-Max")]
        session_per_sec_max: CompactString, 
        #[serde(rename="Session-Per-Sec-FiveMin")]
        session_per_sec_fivemin: CompactString,
        #[serde(rename="Session-Per-Sec-Last")]
        session_per_sec_last: CompactString,
        #[serde(rename="Session-Since-Startup")]
        session_since_startup: CompactString,
        #[serde(rename="Idle-CPU")]
        idle_cpu: CompactString,
        #[serde(rename="Event-Info")]
        event_info: CompactString,
        #[serde(rename="Uptime-msec")]
        uptime_msec: CompactString,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ReSchedule {
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(rename="Task-Runtime")]
        task_runtime: CompactString,
        #[serde(rename="Task-Group")]
        task_group: CompactString,
        #[serde(rename="Task-ID")]
        task_id: CompactString,
        #[serde(rename="Task-Desc")]
        task_desc: CompactString,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelCallstate {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelCreate {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelDestroy {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelState {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelAnswer {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelProgress {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelProgressMedia {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelOriginate {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelOutgoing {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelExecute {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelExecuteComplete {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelHangup {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelBridge {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelUnbridge {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelPark {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelUnpark {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelApplication {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelHold {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelUnhold {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelUuid {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ChannelHangupComplete {
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Shutdown {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    ModuleLoad {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    ModuleUnload {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Reloadxml {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Notify {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    SendMessage {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    RecvMessage {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    RequestParams {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    ChannelData {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    General {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Command {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    SessionHeartbeat {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    ClientDisconnected {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    ServerDisconnected {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    SendInfo {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    RecvInfo {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    CallSecure {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Nat {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    RecordStart {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    RecordStop {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    PlaybackStart {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    PlaybackStop {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    CallUpdate {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    BackgroundJob {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    DetectedTone {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Log {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    InboundChan {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    OutboundChan {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Startup {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Publish {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Unpublish {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Talk {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Notalk {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    SessionCrash {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Dtmf {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Message {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    PresenceIn {
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        channel: EslChannelCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    PresenceOut {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    PresenceProbe {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    MessageWaiting {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    MessageQuery {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Roster {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    RecvRtcpMessage {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Codec {    
        #[serde(flatten)]
        caller: EslCallerCommonFields,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    DetectedSpeech {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    PrivateCommand {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    Trap {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    AddSchedule {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
    DelSchedule {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    ExecSchedule {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    MediaBugStart {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    MediaBugStop {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    Api {
        #[serde(rename = "API-Command")]
        api_command: CompactString,
        #[serde(rename = "API-Command-Argument")]
        api_command_argument: CompactString,
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    Custom(EslCustomEvent),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "Event-Subclass")]
pub enum EslCustomEvent {
    #[serde(rename = "sofia::error")]
    SofiaError {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::expire")]
    SofiaExpire {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::gateway_add")]
    SofiaGatewayAdd {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::gateway_delete")]
    SofiaGatewayDelete {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::gateway_state")]
    SofiaGatewayState {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::intercepted")]
    SofiaIntercepted {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::notify_refer")]
    SofiaNotifyReferer {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::notify_watched_header")]
    SofiaNotifyWatchedHeader {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::pre_register")]
    SofiaPreRegister {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::profile_start")]
    SofiaProfielStart {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::recovery_recovered")]
    SofiaRecoveryRecovered {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::recovery_recv")]
    SofiaRecoveryRecv {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::recovery_send")]
    SofiaRecoverySend {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::register")]
    SofiaRegister {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::register_attempt")]
    SofiaRegisterAttempt {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::register_failure")]
    SofiaRegisterFailure {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::reinvite")]
    SofiaReinvite {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::replaced")]
    SofiaReplaced {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::sip_user_state")]
    SofiaSipUserState {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::transferee")]
    SofiaTransferee {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::transferor")]
    SofiaTransferror {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::unregister")]
    SofiaUnregister {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },

    #[serde(rename = "sofia::wrong_call_state")]
    SofiaWrongCallState {
        #[serde(flatten)]
        event: EslEventCommonFields,
        #[serde(flatten)]
        freeswitch: EslFreeswitchCommonFields,
        #[serde(flatten)]
        other: HashMap<CompactString, CompactString>,
    },
}

#[derive(Debug, Deserialize, PartialEq)]
enum EslChannelState {
    /// Channel is newly created.
    #[serde(rename = "CS_NEW")]
    New,
    #[serde(rename = "CS_INIT")]
    Init,
    #[serde(rename = "CS_ROUTING")]
    Routing,
    #[serde(rename = "CS_EXECUTE")]
    Execute,
    #[serde(rename = "CS_EXCHANGE_MEDIA")]
    ExchangeMedia,
    #[serde(rename = "CS_PARK")]
    Park,
    #[serde(rename = "CS_CONSUME_MEDIA")]
    ConsumeMedia,
    #[serde(rename = "CS_HIBERNATE")]
    Hibernate,
    #[serde(rename = "CS_RESET")]
    Reset,
    #[serde(rename = "CS_HANGUP")]
    Hangup,
    #[serde(rename = "CS_REPORTING")]
    Reporting,
    #[serde(rename = "CS_DESTROY")]
    Destroy,
    #[serde(rename = "CS_NONE")]
    None,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum EslCallState {
    Down,
    Dialing,
    Ringing,
    Early,
    Active,
    Held,
    RingWait,
    Hangup,
    Unheld,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EslCallDirection {
    Inbound,
    Outbound,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EslAnswerState {
    Hangup,
    Answered,
    Early,
    Ringing,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EslHangupCause {
    Unspecified,
    UnallocatedNumber,
    NoRouteTransitNet,
    NoRouteDestination,
    ChannelUnacceptable,
    CallAwardedDelivered,
    NormalClearing,
    UserBusy,
    NoUserResponse,
    NoAnswer,
    SubscriberAbsent,
    CallRejected,
    NumberChanged,
    RedirectionToNewDestination,
    ExchangeRoutingError,
    DestinationOutOfOrder,
    InvalidNumberFormat,
    FacilityRejected,
    ResponseToStatusEnquiry,
    NormalUnspecified,
    NormalCircuitCongestion,
    NetworkOutOfOrder,
    NormalTemporaryFailure,
    SwitchCongestion,
    AccessInfoDiscarded,
    RequestedChanUnavail,
    PreEmpted,
    FacilityNotSubscribed,
    OutgoingCallBarred,
    IncomingCallBarred,
    BearerCapabilityNotAuth,
    BearerCapabilityNotAvail,
    ServiceUnavailable,
    BearerCapabilityNotImpl,
    ChanNotImplemented,
    FacilityNotImplemented,
    ServiceNotImplemented,
    InvalidCallReference,
    IncompatibleDestination,
    InvalidMsgUnspecified,
    MandatoryIeMissing,
    MessageTypeNonexist,
    WrongMessage,
    IeNonexist,
    InvalidIeContents,
    WrongCallState,
    RecoveryOnTimerExpire,
    MandatoryIeLengthError,
    ProtocolError,
    Interworking,
    OriginatorCancel,
    Crash,
    SystemShutdown,
    LoseRace,
    ManagerRequest,
    BlindTransfer,
    AttendedTransfer,
    AllocatedTimeout,
    UserChallenge,
    MediaTimeout,
    PickedOff,
    UserNotRegistered,
    ProgressTimeout,
    GatewayDown,
}

#[derive(Debug, Deserialize)]
pub struct EslChannelCommonFields {
    #[serde(rename = "Channel-State-Number")]
    pub state_number: CompactString,
    #[serde(rename = "Channel-State")]
    pub state: Option<EslChannelState>,
    #[serde(rename = "Channel-Call-State")]
    pub call_state: EslCallState,
    #[serde(rename = "Unique-ID")]
    pub unique_id: CompactString,
    #[serde(rename = "Call-Direction")]
    pub call_direction: EslCallDirection,
    #[serde(rename = "Presence-Call-Direction")]
    pub presence_call_direction: EslCallDirection,
    #[serde(rename = "Channel-HIT-Dialplan")]
    pub hit_dialplan: CompactString,
    #[serde(rename = "Channel-Name")]
    pub name: CompactString,
    #[serde(rename = "Channel-Presence-ID")]
    pub presence_id: Option<CompactString>,
    #[serde(rename = "Channel-Presence-Data")]
    pub presence_data: Option<CompactString>,
    #[serde(rename = "Presence-Data-Cols")]
    pub presence_data_cols: Option<CompactString>,
    #[serde(rename = "Channel-Call-UUID")]
    pub call_uuid: CompactString,
    #[serde(rename = "Answer-State")]
    pub answer_state: EslAnswerState,
    #[serde(rename = "Hangup-Cause")]
    pub hangup_cause: Option<EslHangupCause>,
}

#[derive(Debug, Deserialize)]
pub struct EslFreeswitchCommonFields {
    #[serde(rename = "Core-UUID")]
    pub core_uuid: CompactString,
    #[serde(rename = "FreeSWITCH-Hostname")]
    pub hostname: CompactString,
    #[serde(rename = "FreeSWITCH-Switchname")]
    pub switchname: CompactString,
    #[serde(rename = "FreeSWITCH-IPv4")]
    pub ipv4: CompactString,
    #[serde(rename = "FreeSWITCH-IPv6")]
    pub ipv6: CompactString,
}

#[derive(Debug, Deserialize)]
pub struct EslEventCommonFields {
    #[serde(rename = "Event-Date-Local")]
    pub date_local: CompactString,
    #[serde(rename = "Event-Date-GMT")]
    pub date_gmt: CompactString,
    #[serde(rename = "Event-Date-Timestamp")]
    pub date_timestamp: CompactString,
    #[serde(rename = "Event-Calling-File")]
    pub calling_file: CompactString,
    #[serde(rename = "Event-Calling-Function")]
    pub calling_function: CompactString,
    #[serde(rename = "Event-Calling-Line-Number")]
    pub calling_line_number: CompactString,
    #[serde(rename = "Event-Sequence")]
    pub sequence: CompactString,
}

#[derive(Debug, Deserialize)]
pub struct EslCallerCommonFields {
    #[serde(rename="Caller-ANI")] 
    pub ani: CompactString,
    #[serde(rename="Caller-Caller-ID-Name")] 
    pub caller_id_name: CompactString,
    #[serde(rename="Caller-Caller-ID-Number")] 
    pub caller_id_number: CompactString,
    #[serde(rename="Caller-Channel-Answered-Time")] 
    pub channel_answered_time: CompactString,
    #[serde(rename="Caller-Channel-Bridged-Time")] 
    pub channel_bridged_time: CompactString,
    #[serde(rename="Caller-Channel-Created-Time")] 
    pub channel_created_time: CompactString,
    #[serde(rename="Caller-Channel-Hangup-Time")] 
    pub channel_hangup_time: CompactString,
    #[serde(rename="Caller-Channel-Hold-Accum")] 
    pub channel_hold_accum: CompactString,
    #[serde(rename="Caller-Channel-Last-Hold")] 
    pub channel_last_hold: CompactString,
    #[serde(rename="Caller-Channel-Name")] 
    pub channel_name: CompactString,
    #[serde(rename="Caller-Channel-Progress-Media-Time")] 
    pub channel_progress_media_time: CompactString,
    #[serde(rename="Caller-Channel-Progress-Time")] 
    pub channel_progress_time: CompactString,
    #[serde(rename="Caller-Channel-Resurrect-Time")] 
    pub channel_resurrect_time: CompactString,
    #[serde(rename="Caller-Channel-Transfer-Time")] 
    pub channel_transfer_time: CompactString,
    #[serde(rename="Caller-Context")] 
    pub context: CompactString,
    #[serde(rename="Caller-Destination-Number")] 
    pub destination_number: CompactString,
    #[serde(rename="Caller-Dialplan")] 
    pub dialplan: CompactString,
    #[serde(rename="Caller-Direction")] 
    pub direcrion: EslCallDirection,
    #[serde(rename="Caller-Logical-Direction")]
    pub logical_direction: Option<EslCallDirection>,
    #[serde(rename="Caller-Screen-Bit")]
    pub screen_bit: Option<CompactString>,
    #[serde(rename="Caller-Callee-ID-Name")]
    pub callee_id_name: Option<CompactString>,
    #[serde(rename="Caller-Callee-ID-Number")]
    pub callee_id_number: Option<CompactString>,
    #[serde(rename="Caller-Network-Addr")] 
    pub network_addr: CompactString,
    #[serde(rename="Caller-Orig-Caller-ID-Name")] 
    pub orig_caller_id_name: CompactString,
    #[serde(rename="Caller-Orig-Caller-ID-Number")] 
    pub orig_caller_id_number: CompactString,
    #[serde(rename="Caller-Privacy-Hide-Name")] 
    pub privacy_hide_name: CompactString,
    #[serde(rename="Caller-Privacy-Hide-Number")] 
    pub privacy_hide_number: CompactString,
    #[serde(rename="Caller-Profile-Created-Time")] 
    pub profile_created_time: CompactString,
    #[serde(rename="Caller-Profile-Index")] 
    pub profile_index: CompactString,
    #[serde(rename="Caller-Source")] 
    pub source: CompactString,
    #[serde(rename="Caller-Unique-ID")] 
    pub unique_id: CompactString,
    #[serde(rename="Caller-Username")] 
    pub username: CompactString,
}
