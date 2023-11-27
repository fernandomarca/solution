use super::entity::Entity;

pub trait AggregateRoot
where
    Self: Entity,
{
}
