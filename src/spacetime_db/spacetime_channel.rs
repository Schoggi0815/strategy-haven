use std::sync::Arc;

use bevy::{app::PluginGroupBuilder, prelude::*};
use crossbeam::channel::{Receiver, Sender, TryIter, unbounded};
use spacetimedb_sdk::{Table, TableWithPrimaryKey};

use crate::{
    module_bindings::RemoteTables,
    spacetime_db::{spacetime_server::ServerConnection, spacetime_state::SpacetimeState},
};

pub struct SpacetimeChannelPlugin<
    'a: 'b,
    'b,
    TTable: Table + TableWithPrimaryKey + 'b,
    TF: (Fn(&'a RemoteTables) -> TTable) + Send + Sync + 'static,
> where
    TTable::Row: Send + Sync + Clone + 'static,
{
    table_callback: TF,
}

impl<
    'a: 'b,
    'b,
    TTable: Table + TableWithPrimaryKey + 'b,
    TF: (Fn(&'a RemoteTables) -> TTable) + Send + Sync + 'static,
> SpacetimeChannelPlugin<'a, 'b, TTable, TF>
where
    TTable::Row: Send + Sync + Clone + 'static,
{
    pub fn new(table_callback: TF) -> Self {
        Self { table_callback }
    }
}

impl<
    'a: 'b,
    'b,
    TTable: Table + TableWithPrimaryKey + 'b,
    TF: (Fn(&'a RemoteTables) -> TTable) + Send + Sync + 'static,
> Plugin for SpacetimeChannelPlugin<'a, 'b, TTable, TF>
where
    TTable::Row: Send + Sync + Clone + 'static,
{
    fn build(&self, app: &mut App) {
        let callback = self.table_callback.clone();
        app.init_resource::<SpacetimeChannel<TTable::Row>>()
            .add_systems(
                OnEnter(SpacetimeState::Initialized),
                move |channel: Res<SpacetimeChannel<TTable::Row>>,
                      spacetime: Res<ServerConnection>| {
                    let table = (callback.clone())(&spacetime.0.db);
                    register_spacetime_channel(channel, &(*callback)(&spacetime.0.db));
                },
            )
            .add_systems(
                Update,
                (
                    write_insert_event::<TTable::Row>,
                    write_update_event::<TTable::Row>,
                    write_delete_event::<TTable::Row>,
                ),
            )
            .add_event::<InsertEvent<TTable::Row>>()
            .add_event::<UpdateEvent<TTable::Row>>()
            .add_event::<DeleteEvent<TTable::Row>>();
    }
}

fn write_insert_event<TEntity: Send + Sync + 'static>(
    mut events: EventWriter<InsertEvent<TEntity>>,
    channel: Res<SpacetimeChannel<TEntity>>,
) {
    events.write_batch(
        channel
            .try_iter_insert()
            .map(|entity| InsertEvent { entity }),
    );
}

fn write_update_event<TEntity: Send + Sync + 'static>(
    mut events: EventWriter<UpdateEvent<TEntity>>,
    channel: Res<SpacetimeChannel<TEntity>>,
) {
    events.write_batch(
        channel
            .try_iter_update()
            .map(|entity| UpdateEvent { entity }),
    );
}

fn write_delete_event<TEntity: Send + Sync + 'static>(
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
    insert_receiver: Receiver<TEntity>,
    update_receiver: Receiver<TEntity>,
    delete_receiver: Receiver<TEntity>,
    insert_sender: Sender<TEntity>,
    update_sender: Sender<TEntity>,
    delete_sender: Sender<TEntity>,
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

pub fn register_spacetime_channel<TTable: TableWithPrimaryKey>(
    channel: Res<SpacetimeChannel<TTable::Row>>,
    table: &TTable,
) where
    TTable::Row: Send + Clone,
{
    let insert_sender = channel.insert_sender.clone();
    let update_sender = channel.update_sender.clone();
    let delete_sender = channel.delete_sender.clone();

    table.on_insert(move |_ctx, entity| {
        insert_sender
            .try_send(entity.clone())
            .expect("Unbounded channel should never block!");
    });

    table.on_update(move |_ctx, _old, entity| {
        update_sender
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
