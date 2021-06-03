use crate::client::InteractClient;
use crate::dao::reply::cmd::Command;
use std::sync::mpsc::Sender;


use crate::dao::message::source::Message;
use crate::dao::message::target::Target;
trait SingleInteract {
    fn handle_msg<M, T>(cmd: Command, msg: Box<M>, sender: &mut Sender<Box<T>>)
    where
        M: Message,
        T: Target;
}

trait ContextInteract {
    fn init_context<M, T>(cmd: Command, msg: Box<M>, sender: &mut Sender<Box<T>>) -> Self
    where
        M: Message,
        T: Target;

    fn goon_context<M, T>(&mut self, msg: Box<T>, sender: &mut Sender<Box<T>>)
    where
        M: Message,
        T: Target;
}

trait SideMsg {
    fn init_interact<CLIENT>(_client:CLIENT)where CLIENT:InteractClient{
    }
    fn get_activate_signs() -> Vec<&'static str>;
    fn get_name() -> &'static str {
        "Empty"
    }
    fn get_usage() -> String {
        format!("名称：{}\n\n该功能的使用方法还未定义", Self::get_name())
    }
}
