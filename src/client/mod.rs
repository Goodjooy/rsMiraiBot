pub mod interact;
pub trait InteractClient {
    //TODO:数据库连接功能
    fn get_db_handle(&mut self)->Option<()>{
        todo!()
    }

}
pub trait FilterClient {
    
}