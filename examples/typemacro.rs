use std::sync::Arc;

trait Hello {

}
macro_rules! type_name {
    ($tt:tt) => {
        println!("type name = {}", std::any::type_name::<Arc<dyn $tt>>())
    };
}

fn main() {
    type_name!(Hello);
}
