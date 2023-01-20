use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{AbstractModule, Injector, Binder};

struct Implements {
    implements      : HashMap<String,Arc<dyn AbstractModule>>
}

impl Implements {
    fn add_implement<M : AbstractModule + 'static>(&mut self, name : String, module : M)  {
        self.implements.insert(name, Arc::new(module));
    }
      
    fn has_implement(&mut self, name :String ) -> bool {
        self.implements.contains_key(&name)
    }
    
    fn get_implement(&mut self, name :String ) -> Option<Arc<dyn AbstractModule>> {
        return self.implements.get(&name).map(|x|x.clone());
    }
    
    fn add_implements(&mut self, other : &Implements)  {
        other.implements.iter().for_each(|(key,value)| {
            self.implements.insert(key.clone(), value.clone());
        })
    }


    fn new_injector(&self, enabled : Vec<String> ) -> Injector {


        let mut binder = Binder::new();
        
        enabled.iter().for_each(|name| {
            if let Some(module) = self.implements.get(name) {
                module.config(&mut binder);
            }
        });

        Injector  {
            binds : binder,
            instances : Arc::new(Mutex::new(HashMap::new()))
        }

        
    }
}

