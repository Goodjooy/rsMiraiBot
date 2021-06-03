use serde::Serialize;
use std::error::Error;
use std::sync::mpsc::Sender;

use crate::dao::message::source::Message;
use crate::dao::{message::target, reply::cmd::Command};
use std::sync::mpsc;
trait SingleInteract {
    fn handle_msg<T>(cmd: Command, msg: Box<T>, sender: &mut Sender<Box<dyn target::Target>>)
    where
        T: Message;
}

trait ContextInteract {
    fn init_context<T>(
        cmd: Command,
        msg: Box<T>,
        sender: &mut Sender<Box<dyn target::Target>>,
    ) -> Self
    where
        T: Message;

    fn goon_context<T>(msg: Box<T>, sender: &mut Sender<Box<dyn target::Target>>)
    where
        T: Message;
}
