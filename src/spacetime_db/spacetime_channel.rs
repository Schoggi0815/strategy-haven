use bevy::prelude::*;
use crossbeam::channel::{Receiver, Sender, TryIter, unbounded};
use spacetimedb_sdk::{Table, TableWithPrimaryKey};

use crate::spacetime_db::spacetime_server::ServerConnection;

pub trait SpacetimeChannelRegisterer<TEntity: Send + Sync + Clone + 'static> {
    fn register_channels(channel: Res<SpacetimeChannel<TEntity>>, spacetime: Res<ServerConnection>);
}

pub fn write_insert_event<TEntity: Send + Sync + 'static>(
    mut events: EventWriter<InsertEvent<TEntity>>,
    channel: Res<SpacetimeChannel<TEntity>>,
) {
    events.write_batch(
        channel
            .try_iter_insert()
            .map(|entity| InsertEvent { entity }),
    );
}

pub fn write_update_event<TEntity: Send + Sync + 'static>(
    mut events: EventWriter<UpdateEvent<TEntity>>,
    channel: Res<SpacetimeChannel<TEntity>>,
) {
    events.write_batch(
        channel
            .try_iter_update()
            .map(|entity| UpdateEvent { entity }),
    );
}

pub fn write_delete_event<TEntity: Send + Sync + 'static>(
    mut events: EventWriter<DeleteEvent<TEntity>>,
    channel: Res<SpacetimeChannel<TEntity>>,
) {
    events.write_batch(
        channel
            .try_iter_delete()
            .map(|entity| DeleteEvent { entity }),
    );
}

#[derive(Resource, Clone)]
pub struct SpacetimeChannel<TEntity> {
    pub insert_receiver: Receiver<TEntity>,
    pub update_receiver: Receiver<TEntity>,
    pub delete_receiver: Receiver<TEntity>,
    pub insert_sender: Sender<TEntity>,
    pub update_sender: Sender<TEntity>,
    pub delete_sender: Sender<TEntity>,
}

#[derive(Event)]
pub struct InsertEvent<TEntity: Send + Sync + 'static> {
    pub entity: TEntity,
}

#[derive(Event)]
pub struct UpdateEvent<TEntity: Send + Sync + 'static> {
    pub entity: TEntity,
}

#[derive(Event)]
pub struct DeleteEvent<TEntity: Send + Sync + 'static> {
    pub entity: TEntity,
}

impl<TEntity> SpacetimeChannel<TEntity> {
    pub fn try_iter_insert(&self) -> TryIter<'_, TEntity> {
        self.insert_receiver.try_iter()
    }

    pub fn try_iter_update(&self) -> TryIter<'_, TEntity> {
        self.update_receiver.try_iter()
    }

    pub fn try_iter_delete(&self) -> TryIter<'_, TEntity> {
        self.delete_receiver.try_iter()
    }
}

pub fn register_insert_and_delete_channel<TTable: Table>(
    channel: Res<SpacetimeChannel<TTable::Row>>,
    table: &TTable,
) where
    TTable::Row: Send + Clone,
{
    let insert_sender = channel.insert_sender.clone();
    let delete_sender = channel.delete_sender.clone();

    table.on_insert(move |_ctx, entity| {
        insert_sender
            .try_send(entity.clone())
            .expect("Unbounded channel should never block!");
    });

    table.on_delete(move |_ctx, entity| {
        delete_sender
            .try_send(entity.clone())
            .expect("Unbounded channel should never block!");
    });
}

pub fn register_update_channel<TTable: TableWithPrimaryKey>(
    channel: Res<SpacetimeChannel<TTable::Row>>,
    table: &TTable,
) where
    TTable::Row: Send + Clone,
{
    let update_sender = channel.update_sender.clone();

    table.on_update(move |_ctx, _old, entity| {
        update_sender
            .try_send(entity.clone())
            .expect("Unbounded channel should never block!");
    });
}

impl<TEntity> Default for SpacetimeChannel<TEntity> {
    fn default() -> Self {
        let (delete_sender, delete_receiver) = unbounded::<TEntity>();
        let (insert_sender, insert_receiver) = unbounded::<TEntity>();
        let (update_sender, update_receiver) = unbounded::<TEntity>();

        Self {
            insert_receiver,
            update_receiver,
            delete_receiver,
            insert_sender,
            update_sender,
            delete_sender,
        }
    }
}
