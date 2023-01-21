
macro_rules! type_name {
    ($ty:ty) => {
        println!("type name = {}", std::any::type_name::<$ty>())
    };
}

fn main() {


    let name = std::any::type_name::<String>();
    type_name!(String);
}