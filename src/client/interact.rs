use crate::dao::{message::source::Message, reply::cmd::Command};

use super::FilterClient;

/// # Interact 响应控制器
/// * 保存全部的响应器
/// * 保存一个有序队列，保存指令分析器优先级
///     * 可以通过优先级对数据进行过滤
/// * 保存指令到响应器的hashmap，隐射

trait MsgHandle {
    fn init<C: FilterClient>(client: C);
    /// ### `get_prioity()->i32`
    /// * 提供当前的控制器的优先级，数字越小优先级越高
    /// * 不实例化即可获取数据
    fn get_prioity() -> i32;

    fn do_analyse<M: Message>(msg: M) -> Option<Command>;
}
