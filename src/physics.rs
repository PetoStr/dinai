use crate::math::Vector2f;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Entity {
    pub position: Vector2f,
    pub speed: Vector2f,
}

pub struct World {
    gravity: Vector2f,
    entities: Vec<Rc<RefCell<Entity>>>,
}

impl World {
    pub fn new(gravity: Vector2f) -> Self {
        Self {
            gravity,
            entities: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> Rc<RefCell<Entity>> {
        let entity = Rc::new(RefCell::new(Entity {
            position: Vector2f::new(),
            speed: Vector2f::new(),
        }));

        let cloned_entity = Rc::clone(&entity);

        self.entities.push(entity);

        cloned_entity
    }

    pub fn update(&self) {
        for entity in &self.entities {
            self.update_entity(entity);
        }
    }

    fn update_entity(&self, entity: &Rc<RefCell<Entity>>) {
        let borrowed_entity = &mut *entity.borrow_mut();

        let speed = borrowed_entity.speed.clone();
        borrowed_entity.position += &speed;

        borrowed_entity.speed += &self.gravity;
    }
}
