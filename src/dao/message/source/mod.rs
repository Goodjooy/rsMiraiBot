use serde::Serialize;

pub trait Message {
    fn get_type(&self) -> &MsgType;
    fn get_data<T: MsgChainVec>(&self) -> &T;
    fn get_sender<S: MsgSender>(&self) -> &S;
}

pub trait MsgChainVec {
    fn get<T: serde::Serialize>(&self, key: &str) -> MsgChain<T>;
}

pub trait MsgSender {
    fn get_type() -> MsgType;
    fn get_sender(&self) -> u64;
    fn get_group(&self) -> u64 {
        0
    }
}

pub enum MsgType {
    GroupMessage,
    FriendMessage,
    TempleMessage,
}

pub enum MsgChain<T>
where
    T: Serialize,
{
    Text(T),
}
