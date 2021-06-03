pub trait Message {
    fn get_type(&self)->&MsgType;
    fn get_data<T>(&self)->&dyn MsgChain<T>;
    fn get_sender(&self)-> &dyn MsgSender;
}

pub trait MsgChain<T> {
    fn get_type(&self)->MsgChainType<T>;
    fn get(&self)->T;
}

pub trait MsgSender {
    
}

pub enum MsgType {
    GroupMessage,
    FriendMessage,
    TempleMessage
}

pub enum MsgChainType<T> {
    Text(T),

}