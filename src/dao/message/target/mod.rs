use serde::Serialize;

//use serde;
pub trait Target {
    fn get_body<T>(&self) -> T
    where
        T: Serialize,
    {
        self.get_body_with_auth_key("")
    }
    fn get_body_with_auth_key<T>(&self, auth_key: &str) -> T
    where
        T: Serialize;
    fn get_send_port() -> &'static str;
}
