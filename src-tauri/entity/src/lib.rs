pub mod message {
  include!("proto/message.rs");
}

pub use crate::message::{*};
use crate::chat_message_pack::Obj;
pub use prost::Message as ProstMessage;


impl ChatMessagePack {
  pub fn new(token: &str, msg_type: MsgType, obj: Option<Obj>) -> Self {
    ChatMessagePack {
      token: token.to_string(),
      msg_type: msg_type.get_value(),
      obj,
    }
  }
}

impl MsgType {
  pub fn get_value(&self) -> i32 {
    match self {
      MsgType::LoginMessageType => 0,
      MsgType::SingleMessageType => 1,
      MsgType::GroupMessageType => 2,
      MsgType::AckMessageType => 3,
      MsgType::RollbackMessageType => 4,
      MsgType::TaskReadMessageType => 5,
      MsgType::ResponseMessageType => 6,
    }
  }
}


#[cfg(test)]
mod test {
  use crate::{ChatMessagePack, LoginMessage, MsgType, Obj};
  use prost::Message;

  #[test]
  fn testMessage() {
    let obj = Obj::LoginMessage(LoginMessage {
      user_id: 1,
      username: "admin".to_string(),
    });
    let mut pack = ChatMessagePack::new("123", MsgType::LoginMessageType, Some(obj));
    let len = Message::encoded_len(&pack);
    println!("{}", len);
    let mut buf: Vec<u8> = vec![];
    buf.reserve(len);
    pack.encode(&mut buf).unwrap();
    println!("{:?}", buf);
    let back: ChatMessagePack = Message::decode(&*buf).unwrap();

    println!("{:?}", back);
  }
}
