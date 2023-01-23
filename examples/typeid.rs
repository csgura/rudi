use std::{
    any::{Any, TypeId},
    sync::Arc,
};

trait WidgetTrait {
    fn demo(&self);
}

fn example(widget: Arc<dyn Any>) {
    let tid = TypeId::of::<Arc<dyn WidgetTrait>>();
    let tid2 = TypeId::of::<dyn WidgetTrait>();

    println!("tid = {tid:?} , {tid2:?} , {:?}", widget.type_id());
    match widget.downcast_ref::<Arc<dyn WidgetTrait>>() {
        Some(w) => w.demo(),
        None => println!("Not here!"),
    }
}

struct Alpha(u8);
impl WidgetTrait for Alpha {
    fn demo(&self) {
        dbg!(self.0);
    }
}

fn main() {
    let b: Arc<dyn WidgetTrait> = Arc::new(Alpha(0));
    let b2: Arc<dyn Any> = Arc::new(b);
    example(b2);
}
