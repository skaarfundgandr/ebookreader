use async_trait::async_trait;
// TODO: Make repo operations use this trait
#[async_trait]
pub trait Repository {
    type Item;
    type NewItem;
    type Form;
    type Id: Send + Sync;

    async fn get_all(&self) -> Result<Option<Vec<Self::Item>>, diesel::result::Error>;
    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::Item>, diesel::result::Error>;
    async fn add(&self, item: Self::Item) -> Result<Self::Item, diesel::result::Error>;
    async fn update(&self, id: Self::Id, item: Self::Item) -> Result<(), diesel::result::Error>;
    async fn delete(&self, id: Self::Id) -> Result<(), diesel::result::Error>;
}
