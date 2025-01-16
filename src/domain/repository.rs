use actix_web::Error;
use async_trait::async_trait;
use diesel::dsl::Limit;
use crate::utils::errors::AppError;
/// A trait defining the basic CRUD operations for a repository.
///
/// # Type Parameters
/// - `T`: The type of the entity.
/// - `ID`: The type of the entity's identifier.
/// - `U`: The type of the update data.
/// - `C`: The type of the creation data.
#[async_trait]
pub trait Repository<T, ID, U, C, J> {
    /// Retrieves all entities from the repository.
    ///
    /// # Returns
    /// A `Result` containing a vector of entities or an `AppError`.
    async fn find_all(&self, limit: Option<i16>, offset: Option<i16>, search: Option<String>) -> Result<Vec<T>, Error>;

    /// Retrieves an entity by its identifier.
    ///
    /// # Parameters
    /// - `id`: The identifier of the entity.
    ///
    /// # Returns
    /// A `Result` containing the entity or an `AppError`.
    async fn find_by_id(&self, id: ID) -> Result<T, Error>;

    /// Retrieves an entity by a specified identifier.
    ///
    /// # Parameters
    /// - `id`: The identifier used to find the entity.
    ///
    /// # Returns
    /// A `Result` containing the entity or an `AppError`.
    async fn find_by(&self, id: ID) -> Result<T, Error>;

    /// Retrieves an entity by its email.
    ///
    /// # Parameters
    /// - `email`: The email of the entity.
    ///
    /// # Returns
    /// A `Result` containing the entity or an `AppError`.
    async fn find_by_email(&self, email: &str) -> Result<T, Error>;

    async fn find_by_username(&self, username: &str) -> Result<J, Error>;


    /// Creates a new entity in the repository.
    ///
    /// # Parameters
    /// - `data`: The data used to create the entity.
    ///
    /// # Returns
    /// A `Result` containing the created entity or an `AppError`.
    async fn create(&self, data: C) -> Result<T, Error>;

    /// Updates an existing entity in the repository.
    ///
    /// # Parameters
    /// - `id`: The identifier of the entity to update.
    /// - `entry`: The update data.
    ///
    /// # Returns
    /// A `Result` containing the updated entity or an `AppError`.
    async fn update(&self, id: ID, entry: U) -> Result<T, Error>;

    /// Deletes an entity from the repository.
    ///
    /// # Parameters
    /// - `id`: The identifier of the entity to delete.
    ///
    /// # Returns
    /// A `Result` indicating success or an `AppError`.
    async fn delete(&self, id: ID) -> Result<(), Error>;
}