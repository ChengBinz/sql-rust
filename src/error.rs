// 自定义 Result 类型
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    Parse(String),
}
