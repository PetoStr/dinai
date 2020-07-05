use crate::math::Vector2f;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub struct CollFilter {
    pub group_id: u32,
    pub check_mask: u32,
}

#[derive(Debug, Clone, Default)]
pub struct Transform {
    pub pos: Vector2f,
    pub size: Vector2f,
}

impl Transform {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn intersects(&self, other: &Transform) -> bool {
        self.pos.x + self.size.x > other.pos.x
            && other.pos.x + other.size.x > self.pos.x
            && self.pos.y + self.size.y > other.pos.y
            && other.pos.y + other.size.y > self.pos.y
    }
}

#[derive(Debug, Clone, Default)]
pub struct Physics {
    pub speed: Vector2f,
    pub disable_gravity: bool,
    pub coll_filter: CollFilter,
}

impl Physics {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Clone)]
pub struct Entity {
    pub transform: Transform,
    pub physics: Physics,
    pub collision: fn(&mut Self, &Self),
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

    pub fn add_entity(&mut self, entity: Rc<RefCell<Entity>>) {
        self.entities.push(entity);
    }

    pub fn update(&self) {
        for entity in &self.entities {
            self.update_entity(&mut entity.borrow_mut());
        }

        for entity in &self.entities {
            self.check_collisions(entity);
        }
    }

    pub fn entities(&self) -> &Vec<Rc<RefCell<Entity>>> {
        &self.entities
    }

    fn update_entity(&self, entity: &mut Entity) {
        let speed = entity.physics.speed.clone();
        let transform = &mut entity.transform;

        transform.pos += &speed;

        let physics = &mut entity.physics;
        if !physics.disable_gravity {
            physics.speed += &self.gravity;
        }
    }

    fn check_collisions(&self, entity: &Rc<RefCell<Entity>>) {
        let mut borrowed_entity = entity.borrow_mut();
        for other in &self.entities {
            if entity as *const _ == other as *const _ {
                continue;
            }

            let other = other.borrow();
            let check_mask = borrowed_entity.physics.coll_filter.check_mask;
            let group_id = other.physics.coll_filter.group_id;

            if (check_mask & group_id) != 0
                && borrowed_entity.transform.intersects(&other.transform)
            {
                (borrowed_entity.collision)(&mut borrowed_entity, &other);
            }
        }
    }
}
