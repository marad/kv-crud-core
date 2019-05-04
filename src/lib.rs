use std::fmt::Debug;

/// Represents an entity that can be stored in the system.
/// You have to implement this trait for your custom type to be able to store it.
/// Keep in mind that different implementations may require additional type constraints.
pub trait Entity<IdType>: Clone + Debug {
    fn get_id(&self) -> IdType;
}

/// Represents ability to save entity in the storage.
/// This trait is used by custom storage implementations.
pub trait Create<I, E: Entity<I>> {
    type Error;
    fn save(&mut self, entity: &E) -> Result<(), Self::Error>;
}

/// Represents the ability to find single entity by its id in the system.
/// This trait is used by custom storage implementations.
pub trait Read<I, E: Entity<I>> {
    type Error;
    fn find_by_id(&self, id: &I) -> Result<E, Self::Error>;
}

/// Represents the ability to list items with pagination and sorting.
/// This trait is used by custom storage implementations.
pub trait ReadWithPaginationAndSort<I, E: Entity<I>> {
    type Error;
    fn find_all_with_page(&self, page: &Page) -> Result<Vec<E>, Self::Error>;
    fn find_all_with_page_and_sort(&self, page: &Page, sort: &Sort) -> Result<Vec<E>, Self::Error>;
}

/// Represents the ability to update the entity.
/// This trait is used by custom storage implementations.
pub trait Update<I, E: Entity<I>> {
    type Error;
    fn update(&mut self, entity: &E) -> Result<(), Self::Error>;
}

/// Represents the ability to remove the entity from the storage.
/// This trait is used by custom storage implementations.
pub trait Delete<I, E: Entity<I>> {
    type Error;
    fn remove_by_id(&mut self, id: &I) -> Result<(), Self::Error>;
    fn remove(&mut self, entity: &E) -> Result<(), Self::Error>;
}

pub trait Crud<I, E: Entity<I>>: Create<I, E> + Read<I, E> + ReadWithPaginationAndSort<I, E> + Update<I, E> + Delete<I, E> {

}

/// Used for result pagination.
pub struct Page {
    /// Page number (starting from 0)
    pub number: u32,
    /// Results per page
    pub size: u32,
}

impl Page {
    /// Creates new page
    pub fn new(number: u32, size: u32) -> Self {
        Self { number, size }
    }

    /// Calculates the offset for given page
    pub fn offset(&self) -> u32 {
        self.number * self.size
    }
}

/// Determines sorting order
#[derive(PartialEq)]
pub enum Sort {
    ASCENDING,
    DESCENDING,
}
