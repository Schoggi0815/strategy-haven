// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

// This was generated using spacetimedb cli version 1.2.0 (commit fb41e50eb73573b70eea532aeb6158eaac06fae0).

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct KickUserArgs {
    pub guild_id: u64,
    pub user_id: __sdk::Identity,
}

impl From<KickUserArgs> for super::Reducer {
    fn from(args: KickUserArgs) -> Self {
        Self::KickUser {
            guild_id: args.guild_id,
            user_id: args.user_id,
        }
    }
}

impl __sdk::InModule for KickUserArgs {
    type Module = super::RemoteModule;
}

pub struct KickUserCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `kick_user`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait kick_user {
    /// Request that the remote module invoke the reducer `kick_user` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_kick_user`] callbacks.
    fn kick_user(&self, guild_id: u64, user_id: __sdk::Identity) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `kick_user`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`KickUserCallbackId`] can be passed to [`Self::remove_on_kick_user`]
    /// to cancel the callback.
    fn on_kick_user(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &u64, &__sdk::Identity) + Send + 'static,
    ) -> KickUserCallbackId;
    /// Cancel a callback previously registered by [`Self::on_kick_user`],
    /// causing it not to run in the future.
    fn remove_on_kick_user(&self, callback: KickUserCallbackId);
}

impl kick_user for super::RemoteReducers {
    fn kick_user(&self, guild_id: u64, user_id: __sdk::Identity) -> __sdk::Result<()> {
        self.imp
            .call_reducer("kick_user", KickUserArgs { guild_id, user_id })
    }
    fn on_kick_user(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &u64, &__sdk::Identity) + Send + 'static,
    ) -> KickUserCallbackId {
        KickUserCallbackId(self.imp.on_reducer(
            "kick_user",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::KickUser { guild_id, user_id },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, guild_id, user_id)
            }),
        ))
    }
    fn remove_on_kick_user(&self, callback: KickUserCallbackId) {
        self.imp.remove_on_reducer("kick_user", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `kick_user`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_kick_user {
    /// Set the call-reducer flags for the reducer `kick_user` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn kick_user(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_kick_user for super::SetReducerFlags {
    fn kick_user(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("kick_user", flags);
    }
}
