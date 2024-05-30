extern crate alloc;

use bdk_chain::Append;
use core::convert::Infallible;
use core::fmt::{Debug, Display};

// /// `Persist` wraps a [`PersistBackend`] to create a convenient staging area for changes (`C`)
// /// before they are persisted.
// ///
// /// Not all changes to the in-memory representation needs to be written to disk right away, so
// /// [`Persist::stage`] can be used to *stage* changes first and then [`Persist::commit`] can be used
// /// to write changes to disk.
// pub struct Persist<C, B> {
//     backend: B,
//     stage: C,
// }
//
// impl<C: fmt::Debug, B> fmt::Debug for Persist<C, B> {
//     fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
//         write!(fmt, "{:?}", self.stage)?;
//         Ok(())
//     }
// }
//
// impl<C, B> Persist<C, B>
// where
//     C: Default + Append,
//     B: PersistBackend<C>,
//     // B::LoadError: fmt::Debug + fmt::Display,
//     // B::WriteError: fmt::Debug + fmt::Display,
// {
//     /// Create a new [`Persist`] from [`PersistBackend`].
//     pub fn new(backend: B) -> Self {
//         Self {
//             backend,
//             stage: Default::default(),
//         }
//     }
//
//     /// Stage a `changeset` to be committed later with [`commit`].
//     ///
//     /// [`commit`]: Self::commit
//     pub fn stage(&mut self, changeset: C) {
//         self.stage.append(changeset)
//     }
//
//     /// Get the changes that have not been committed yet.
//     pub fn staged(&self) -> &C {
//         &self.stage
//     }
//
//     /// Commit the staged changes to the underlying persistence backend.
//     ///
//     /// Changes that are committed (if any) are returned.
//     ///
//     /// # Error
//     ///
//     /// Returns a backend-defined error if this fails.
//     pub fn commit(&mut self) -> Result<Option<C>, B::WriteError> {
//         if self.stage.is_empty() {
//             return Ok(None);
//         }
//         self.backend
//             .write_changes(&self.stage)
//             // if written successfully, take and return `self.stage`
//             .map(|_| Some(core::mem::take(&mut self.stage)))
//     }
//
//     /// Stages a new changeset and commits it (along with any other previously staged changes) to
//     /// the persistence backend
//     ///
//     /// Convenience method for calling [`stage`] and then [`commit`].
//     ///
//     /// [`stage`]: Self::stage
//     /// [`commit`]: Self::commit
//     pub fn stage_and_commit(&mut self, changeset: C) -> Result<Option<C>, B::WriteError> {
//         self.stage(changeset);
//         self.commit()
//     }
// }

/// `Stage` adds a convenient staging area for changes (`C`) before they are persisted.
///
/// Not all changes to the in-memory representation needs to be written to disk right away, so
/// [`crate::persist::Stage::stage`] can be used to *stage* changes first and then
/// [`crate::persist::Stage::commit`] can be used to write changes to disk.
pub trait Stage<C: Default + Append>: Persist<C> {
    /// Stage a `changeset` to be committed later with [`commit`].
    ///
    /// [`commit`]: Self::commit
    fn stage(&mut self, changeset: C);

    /// Get the changes that have not been committed yet.
    fn staged(&self) -> &C;

    /// Take the changes that have not been committed yet.
    ///
    /// New staged is set to default;
    fn take_staged(&mut self) -> C;

    /// Commit the staged changes to the underlying persistence backend.
    ///
    /// Changes that are committed (if any) are returned.
    ///
    /// # Error
    ///
    /// Returns a backend-defined error if this fails.
    fn commit(&mut self) -> Result<Option<C>, Self::WriteError> {
        if self.staged().is_empty() {
            return Ok(None);
        }
        let staged = self.take_staged();
        self.write_changes(&staged)
            // if written successfully, take and return `self.stage`
            .map(|_| Some(staged))
    }

    /// Stages a new changeset and commits it (along with any other previously staged changes) to
    /// the persistence backend
    ///
    /// Convenience method for calling [`stage`] and then [`commit`].
    ///
    /// [`stage`]: Self::stage
    /// [`commit`]: Self::commit
    fn stage_and_commit(&mut self, changeset: C) -> Result<Option<C>, Self::WriteError> {
        self.stage(changeset);
        self.commit()
    }
}

/// A persistence backend for writing and loading changesets.
///
/// `C` represents the changeset; a datatype that records changes made to in-memory data structures
/// that are to be persisted, or retrieved from persistence.
pub trait Persist<C> {
    /// The error the backend returns when it fails to write.
    type WriteError: Debug + Display;

    /// The error the backend returns when it fails to load changesets `C`.
    type LoadError: Debug + Display;

    /// Writes a changeset to the persistence backend.
    ///
    /// It is up to the backend what it does with this. It could store every changeset in a list or
    /// it inserts the actual changes into a more structured database. All it needs to guarantee is
    /// that [`load_from_persistence`] restores a keychain tracker to what it should be if all
    /// changesets had been applied sequentially.
    ///
    /// [`load_from_persistence`]: Self::load_changes
    fn write_changes(&mut self, changeset: &C) -> Result<(), Self::WriteError>;

    /// Return the aggregate changeset `C` from persistence.
    fn load_changes(&mut self) -> Result<Option<C>, Self::LoadError>;
}

impl<C> Persist<C> for () {
    type WriteError = Infallible;
    type LoadError = Infallible;

    fn write_changes(&mut self, _changeset: &C) -> Result<(), Self::WriteError> {
        Ok(())
    }

    fn load_changes(&mut self) -> Result<Option<C>, Self::LoadError> {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use crate::persist::test::TestError::FailedWrite;
    use crate::{Persist, Stage};
    use alloc::string::{String, ToString};
    use bdk_chain::Append;
    use core::fmt::Formatter;
    use core::{fmt, mem};

    struct TestBackend<C: Default + Append + Clone + ToString> {
        changeset: C,
        staged: C,
    }

    #[derive(Debug, Eq, PartialEq)]
    enum TestError {
        FailedWrite,
        FailedLoad,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Clone, Default)]
    struct TestChangeSet(Option<String>);

    impl fmt::Display for TestChangeSet {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.clone().0.unwrap_or_default())
        }
    }

    impl Append for TestChangeSet {
        fn append(&mut self, other: Self) {
            if other.0.is_some() {
                self.0 = other.0
            }
        }

        fn is_empty(&self) -> bool {
            self.0.is_none()
        }
    }

    impl<C> Persist<C> for TestBackend<C>
    where
        C: Default + Append + Clone + ToString,
    {
        type WriteError = TestError;
        type LoadError = TestError;

        fn write_changes(&mut self, changeset: &C) -> Result<(), Self::WriteError> {
            if changeset.to_string() == "ERROR" {
                Err(FailedWrite)
            } else {
                self.changeset = changeset.clone();
                Ok(())
            }
        }

        fn load_changes(&mut self) -> Result<Option<C>, Self::LoadError> {
            if self.changeset.to_string() == "ERROR" {
                Err(Self::LoadError::FailedLoad)
            } else {
                Ok(Some(self.changeset.clone()))
            }
        }
    }

    impl<C> Stage<C> for TestBackend<C>
    where
        C: Default + Append + Clone + ToString,
    {
        fn stage(&mut self, changeset: C) {
            self.staged.append(changeset)
        }

        fn staged(&self) -> &C {
            &self.staged
        }

        fn take_staged(&mut self) -> C {
            mem::take(&mut self.staged)
        }
    }

    #[test]
    fn test_persist_stage_commit() {
        let mut backend = TestBackend {
            changeset: TestChangeSet(None),
            staged: Default::default(),
        };
        backend.stage(TestChangeSet(Some("ONE".to_string())));
        backend.stage(TestChangeSet(None));
        backend.stage(TestChangeSet(Some("TWO".to_string())));
        let result = backend.commit();
        assert!(matches!(result, Ok(Some(TestChangeSet(Some(v)))) if v == *"TWO".to_string()));

        let result = backend.commit();
        assert!(matches!(result, Ok(None)));

        backend.stage(TestChangeSet(Some("TWO".to_string())));
        let result = backend.stage_and_commit(TestChangeSet(Some("ONE".to_string())));
        assert!(matches!(result, Ok(Some(TestChangeSet(Some(v)))) if v == *"ONE".to_string()));
    }

    #[test]
    fn test_persist_commit_error() {
        let mut backend = TestBackend {
            changeset: TestChangeSet(None),
            staged: Default::default(),
        };
        backend.stage(TestChangeSet(Some("ERROR".to_string())));
        let result = backend.commit();
        assert!(matches!(result, Err(e) if e == FailedWrite));
    }
}
