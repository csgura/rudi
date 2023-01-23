macro_rules! type_name {
    ($ty:ty) => {
        println!("type name = {}", std::any::type_name::<$ty>())
    };
}

fn main() {
    type_name!(String);
}
