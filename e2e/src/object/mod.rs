//! Browser-side objects.

pub mod connection;
pub mod connections_store;
pub mod jason;
pub mod local_track;
pub mod remote_track;
pub mod room;
pub mod tracks_store;

use std::{marker::PhantomData, sync::mpsc};

use derive_more::with_trait::{Display, Error as StdError, From};
use serde_json::Value as Json;
use tokio::task;
use uuid::Uuid;

pub use self::{
    jason::Jason,
    room::{MediaKind, MediaSourceKind, Room},
};
use crate::browser::{self, Statement};

/// All errors which can happen while working with [`Object`]s.
#[derive(Debug, Display, From, StdError)]
pub enum Error {
    /// Error while interacting with a browser.
    Browser(browser::Error),

    /// Failed JS object type casting.
    TypeCast,
}

/// Policy applied to [`Object`]'s functions spawning promises.
#[derive(Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum AwaitCompletion {
    /// Wait for the spawned promise to complete completion.
    #[display("await")]
    Do,

    /// Don't wait for the spawned promise completion.
    #[display("")]
    Dont,
}

/// Pointer to a JS object on a browser's side.
// TODO: Needs refactoring.
#[expect(clippy::module_name_repetitions, reason = "needs refactoring")]
#[derive(Clone, Debug, Display)]
pub struct ObjectPtr(String);

/// Representation of some JS object on a browser's side.
///
/// JS object on browser's side will be removed on this [`Object`]'s [`Drop`].
#[derive(Debug)]
pub struct Object<T> {
    /// Pointer to the JS object on a browser's side.
    ptr: ObjectPtr,

    /// [`browser::Window`] where this [`Object`] exists.
    window: browser::Window,

    /// Type of this [`Object`].
    _type: PhantomData<T>,
}

impl<T> Drop for Object<T> {
    /// Removes this [`Object`] on a browser's side.
    fn drop(&mut self) {
        let ptr = self.ptr.clone();
        let window = self.window.clone();
        let (tx, rx) = mpsc::channel();
        drop(tokio::spawn(async move {
            window
                .execute(Statement::new(
                    // language=JavaScript
                    "
                    async () => {
                        const [id] = args;
                        window.registry.delete(id);
                    }
                    ",
                    [ptr.to_string().into()],
                ))
                .await
                .map(drop)
                .unwrap();
            tx.send(()).unwrap();
        }));
        task::block_in_place(move || {
            rx.recv().unwrap();
        });
    }
}

impl<T> Object<T> {
    /// Returns a new [`Object`] with the provided ID and [`browser::Window`].
    #[must_use]
    pub const fn new(id: String, window: browser::Window) -> Self {
        Self { ptr: ObjectPtr(id), window, _type: PhantomData }
    }

    /// Returns an [`ObjectPtr`] to this [`Object`].
    #[must_use]
    pub fn ptr(&self) -> ObjectPtr {
        self.ptr.clone()
    }

    /// Executes the provided [`Statement`] and returns the resulting
    /// [`Object`].
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn execute_and_fetch<O>(
        &self,
        statement: Statement,
    ) -> Result<Object<O>, Error> {
        let id = Uuid::new_v4().to_string();
        self.execute(statement.and_then(Statement::new(
            // language=JavaScript
            "
            async (obj) => {
                const [id] = args;
                window.registry.set(id, obj);
            }
            ",
            [id.clone().into()],
        )))
        .await
        .map(drop)?;

        Ok(Object::new(id, self.window.clone()))
    }

    /// Indicates whether this [`Object`] is `undefined`.
    ///
    /// # Errors
    ///
    /// - If failed to execute JS statement.
    /// - If failed to parse result as [`bool`].
    pub async fn is_undefined(&self) -> Result<bool, Error> {
        self.execute(Statement::new(
            // language=JavaScript
            "async (o) => o === undefined",
            [],
        ))
        .await?
        .as_bool()
        .ok_or(Error::TypeCast)
    }

    /// Executes the provided [`Statement`] in a browser.
    ///
    /// JS object representing this [`Object`] will be passed to the provided
    /// [`Statement`] as a lambda argument.
    async fn execute(&self, js: Statement) -> Result<Json, Error> {
        self.window
            .execute(self.get_obj().and_then(js))
            .await
            .map_err(Error::Browser)
    }

    /// Returns a [`Statement`] obtaining JS object of this [`Object`].
    fn get_obj(&self) -> Statement {
        Statement::new(
            // language=JavaScript
            "
            async () => {
                const [id] = args;
                return window.registry.get(id);
            }
            ",
            [self.ptr.to_string().into()],
        )
    }
}

impl<T: Builder> Object<T> {
    /// Spawns the provided [`Object`] in the provided [`browser::Window`].
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn spawn(obj: T, window: browser::Window) -> Result<Self, Error> {
        let id = Uuid::new_v4().to_string();
        window
            .execute(obj.build().and_then(Statement::new(
                // language=JavaScript
                "
                async (obj) => {
                    const [id] = args;
                    window.registry.set(id, obj);
                }
                ",
                [id.clone().into()],
            )))
            .await
            .map(drop)?;

        Ok(Self::new(id, window))
    }
}

/// JS object builder for an [`Object`].
pub trait Builder {
    /// Returns a [`Statement`] creating a desired JS object.
    #[must_use]
    fn build(self) -> Statement;
}
