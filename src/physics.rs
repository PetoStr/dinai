//! An engine that handles entity physics.

use crate::math::Vector2f;
use std::cell::RefCell;
use std::rc::Rc;

/// Specifies with what an object should collide.
#[derive(Debug, Clone, Default)]
pub struct CollFilter {
    /// Thanks to this `group_id` other objects can collide with this group. Note that this value
    /// must be in power of two, therefore there are 33 possible groups. Value of 0 means that
    /// no other object can collide with this object.
    pub group_id: u32,

    /// With which `group_id`'s should collision be detected. It is a subset of ids using
    /// bitwise OR operation, for example `1 | 2 | 8`.
    pub check_mask: u32,
}

/// Transformation details that describe where an object is in screen space.
#[derive(Debug, Clone, Default)]
pub struct Transform {
    /// Position in screen space.
    pub pos: Vector2f,

    /// Width and height in screen space.
    pub size: Vector2f,
}

impl Transform {
    /// Creates a new `Transform` with default values.
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks whether this `Transform` intersects with other.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dinai::physics::Transform;
    /// # use dinai::math::Vector2f;
    /// let left = Transform {
    ///     pos: Vector2f::from_coords(0.0, 0.0),
    ///     size: Vector2f::from_coords(25.0, 25.0),
    /// };
    ///
    /// let right = Transform {
    ///     pos: Vector2f::from_coords(20.0, 0.0),
    ///     size: Vector2f::from_coords(25.0, 25.0),
    /// };
    ///
    /// assert!(left.intersects(&right));
    /// ```
    pub fn intersects(&self, other: &Transform) -> bool {
        self.pos.x + self.size.x > other.pos.x
            && other.pos.x + other.size.x > self.pos.x
            && self.pos.y + self.size.y > other.pos.y
            && other.pos.y + other.size.y > self.pos.y
    }
}

/// Contains basic physical properties.
#[derive(Debug, Clone, Default)]
pub struct Physics {
    /// Speed vector in 2D space.
    pub speed: Vector2f,

    /// Whether the gravity for an object should be disabled.
    pub disable_gravity: bool,

    /// Collision filter details.
    pub coll_filter: CollFilter,
}

impl Physics {
    /// Creates a new `Physics` with default values.
    pub fn new() -> Self {
        Default::default()
    }
}

/// An object in the world that can interact with other entities.
#[derive(Clone)]
pub struct Entity {
    /// Transformation of this entity in screen space.
    pub transform: Transform,

    /// Physical properties of this entity.
    pub physics: Physics,

    /// Function pointer to function that handles entity's collision. First parameter is this
    /// entity and the second parameter is the entity with which this entity collided.
    pub collision: fn(this: &mut Self, other: &Self),
}

/// A container for handling entities.
pub struct World {
    gravity: Vector2f,
    entities: Vec<Rc<RefCell<Entity>>>,
}

impl World {
    /// Creates a new `World` for entities with given gravity.
    ///
    /// # Examples
    /// ```
    /// # use dinai::physics::World;
    /// # use dinai::math::Vector2f;
    /// let mut world = World::new(Vector2f::from_coords(0.0, 0.05));
    /// ```
    pub fn new(gravity: Vector2f) -> Self {
        Self {
            gravity,
            entities: Vec::new(),
        }
    }

    /// Adds an entity into this world.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// # use dinai::physics::{Entity, World};
    /// # use dinai::math::Vector2f;
    ///
    /// let mut world = World::new(Vector2f::from_coords(0.0, 0.05));
    ///
    /// let entity = Rc::new(RefCell::new(Entity {
    ///     transform: Default::default(),
    ///     physics: Default::default(),
    ///     collision: |_this, _other| {},
    /// }));
    ///
    /// world.add_entity(Rc::clone(&entity));
    pub fn add_entity(&mut self, entity: Rc<RefCell<Entity>>) {
        self.entities.push(entity);
    }

    /// Update entity physics. This includes movement and collision detection.
    pub fn update(&self) {
        for entity in &self.entities {
            self.update_entity(&mut entity.borrow_mut());
        }

        for entity in &self.entities {
            self.check_collisions(entity);
        }
    }

    /// Returns a reference to a vector of entities in this world.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let left = Transform {
            pos: Vector2f::from_coords(-20.0, 0.0),
            size: Vector2f::from_coords(45.0, 25.0),
        };

        let right = Transform {
            pos: Vector2f::from_coords(20.0, 0.0),
            size: Vector2f::from_coords(25.0, 25.0),
        };

        assert!(left.intersects(&right));
    }

    #[test]
    fn test_no_intersection() {
        let left = Transform {
            pos: Vector2f::from_coords(-20.0, 0.0),
            size: Vector2f::from_coords(45.0, 25.0),
        };

        let right = Transform {
            pos: Vector2f::from_coords(25.1, 0.0),
            size: Vector2f::from_coords(25.0, 25.0),
        };

        assert!(!left.intersects(&right));
    }
}
