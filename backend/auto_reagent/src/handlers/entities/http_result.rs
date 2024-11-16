use serde::Deserialize;
struct HttpResult<T:Deserialize<'static>> {
    msg: String,
    data: T,
}