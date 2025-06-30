use bevy::ecs::{resource::Resource, system::Res};
use crossbeam::channel::{Receiver, Sender, TryIter, unbounded};
use spacetimedb_sdk::TableWithPrimaryKey;

#[derive(Resource, Clone)]
pub struct SpacetimeChannel<TEntity> {
    receiver: Receiver<TEntity>,
    delete_receiver: Receiver<TEntity>,
    sender: Sender<TEntity>,
    delete_sender: Sender<TEntity>,
}

impl<TEntity> SpacetimeChannel<TEntity> {
    pub fn try_iter(&self) -> TryIter<'_, TEntity> {
        self.receiver.try_iter()
    }

    pub fn try_iter_delete(&self) -> TryIter<'_, TEntity> {
        self.delete_receiver.try_iter()
    }
}

pub fn register_spacetime_channel<TTable: TableWithPrimaryKey>(
    channel: Res<SpacetimeChannel<TTable::Row>>,
    table: TTable,
) where
    TTable::Row: Send + Clone,
{
    let sender = channel.sender.clone();
    let sender_2 = channel.sender.clone();
    let delete_sender = channel.delete_sender.clone();

    table.on_insert(move |_ctx, entity| {
        sender
            .try_send(entity.clone())
            .expect("Unbounded channel should never block!");
    });

    table.on_update(move |_ctx, _old, entity| {
        sender_2
            .try_send(entity.clone())
            .expect("Unbounded channel should never block!");
    });

    table.on_delete(move |_ctx, entity| {
        delete_sender
            .try_send(entity.clone())
            .expect("Unbounded channel should never block!");
    });
}

impl<TEntity> Default for SpacetimeChannel<TEntity> {
    fn default() -> Self {
        let (delete_sender, delete_receiver) = unbounded::<TEntity>();
        let (sender, receiver) = unbounded::<TEntity>();

        Self {
            receiver,
            delete_receiver,
            sender,
            delete_sender,
        }
    }
}
