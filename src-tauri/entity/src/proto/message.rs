#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessagePack {
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
    #[prost(enumeration = "MsgType", tag = "3")]
    pub msg_type: i32,
    #[prost(oneof = "chat_message_pack::Obj", tags = "4, 5, 6, 7, 8, 9, 10")]
    pub obj: ::core::option::Option<chat_message_pack::Obj>,
}
/// Nested message and enum types in `ChatMessagePack`.
pub mod chat_message_pack {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Obj {
        #[prost(message, tag = "4")]
        LoginMessage(super::LoginMessage),
        #[prost(message, tag = "5")]
        SingleMessage(super::SingleMessage),
        #[prost(message, tag = "6")]
        GroupMessage(super::GroupMessage),
        #[prost(message, tag = "7")]
        AckMessage(super::AckMessage),
        #[prost(message, tag = "8")]
        RollbackMessage(super::RollbackMessage),
        #[prost(message, tag = "9")]
        TaskReadMessage(super::TaskReadMessage),
        #[prost(message, tag = "10")]
        ResponseMessage(super::ResponseMessage),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginMessage {
    #[prost(int32, tag = "1")]
    pub user_id: i32,
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleMessage {
    #[prost(int32, tag = "1")]
    pub receiver_id: i32,
    #[prost(string, tag = "2")]
    pub receiver_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub chat_message: ::core::option::Option<ChatMessage>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMessage {
    #[prost(int32, tag = "1")]
    pub group_id: i32,
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub chat_message: ::core::option::Option<ChatMessage>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AckMessage {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub msg_content: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub msg_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub room_id: i32,
    #[prost(int32, tag = "5")]
    pub user_id: i32,
    #[prost(int32, tag = "6")]
    pub msg_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackMessage {
    #[prost(string, tag = "1")]
    pub msg_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub room_id: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskReadMessage {
    #[prost(string, tag = "1")]
    pub msg_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub room_id: i32,
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMessage {
    #[prost(int32, tag = "1")]
    pub code: i32,
    /// 该msg为服务端返回响应的消息内容，例如: "成功" ,"失败"
    #[prost(string, tag = "2")]
    pub msg: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub msg_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessage {
    /// 消息id uuid
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// 聊天室id
    #[prost(int32, tag = "2")]
    pub chat_room_id: i32,
    /// 消息发送者
    #[prost(int32, tag = "4")]
    pub sender_id: i32,
    /// 消息内容
    #[prost(string, tag = "5")]
    pub content: ::prost::alloc::string::String,
    /// 图片路径
    #[prost(string, tag = "6")]
    pub images_path: ::prost::alloc::string::String,
    /// 附件路径
    #[prost(string, tag = "7")]
    pub files_path: ::prost::alloc::string::String,
    /// 视频路径
    #[prost(string, tag = "8")]
    pub videos_path: ::prost::alloc::string::String,
    /// 发送时间
    #[prost(int64, tag = "9")]
    pub send_time: i64,
    /// 创建时间
    #[prost(int64, tag = "10")]
    pub created_time: i64,
    /// 发送人的名称
    #[prost(string, tag = "11")]
    pub sender_name: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub sender_avatar: ::prost::alloc::string::String,
    /// 接收人id
    #[prost(int32, tag = "13")]
    pub receiver_id: i32,
    /// 是否已读
    #[prost(bool, tag = "14")]
    pub is_read: bool,
    /// 聊天室名称
    #[prost(string, tag = "15")]
    pub chat_room_name: ::prost::alloc::string::String,
    /// 已读消息人数
    #[prost(int32, tag = "16")]
    pub read_count: i32,
    /// 接收消息总人数
    #[prost(int32, tag = "21")]
    pub receiver_count: i32,
    /// 图片路径集合
    #[prost(string, repeated, tag = "22")]
    pub image_path_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 文件路径集合
    #[prost(string, repeated, tag = "23")]
    pub file_path_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 视频路径集合
    #[prost(string, repeated, tag = "24")]
    pub video_path_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgType {
    /// 登录消息,用于身份验证
    LoginMessageType = 0,
    /// 私聊消息
    SingleMessageType = 1,
    /// 群聊消息
    GroupMessageType = 2,
    /// 应答消息,用于确认消息是否已发送,由服务器返回
    AckMessageType = 3,
    /// 撤回消息
    RollbackMessageType = 4,
    /// 任务消息已读
    TaskReadMessageType = 5,
    /// 通知消息，例如错误消息和消息发送成功地回告
    ResponseMessageType = 6,
}
impl MsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MsgType::LoginMessageType => "LOGIN_MESSAGE_TYPE",
            MsgType::SingleMessageType => "SINGLE_MESSAGE_TYPE",
            MsgType::GroupMessageType => "GROUP_MESSAGE_TYPE",
            MsgType::AckMessageType => "ACK_MESSAGE_TYPE",
            MsgType::RollbackMessageType => "ROLLBACK_MESSAGE_TYPE",
            MsgType::TaskReadMessageType => "TASK_READ_MESSAGE_TYPE",
            MsgType::ResponseMessageType => "RESPONSE_MESSAGE_TYPE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOGIN_MESSAGE_TYPE" => Some(Self::LoginMessageType),
            "SINGLE_MESSAGE_TYPE" => Some(Self::SingleMessageType),
            "GROUP_MESSAGE_TYPE" => Some(Self::GroupMessageType),
            "ACK_MESSAGE_TYPE" => Some(Self::AckMessageType),
            "ROLLBACK_MESSAGE_TYPE" => Some(Self::RollbackMessageType),
            "TASK_READ_MESSAGE_TYPE" => Some(Self::TaskReadMessageType),
            "RESPONSE_MESSAGE_TYPE" => Some(Self::ResponseMessageType),
            _ => None,
        }
    }
}
