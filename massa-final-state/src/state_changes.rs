//! Copyright (c) 2022 MASSA LABS <info@massa.net>

//! This file provides structures representing changes to the final state

use massa_async_pool::{
    AsyncPoolChanges, AsyncPoolChangesDeserializer, AsyncPoolChangesSerializer,
};
use massa_ledger::{LedgerChanges, LedgerChangesDeserializer, LedgerChangesSerializer};
use massa_serialization::{Deserializer, SerializeError, Serializer};
use nom::{
    error::{context, ContextError, ParseError},
    sequence::tuple,
    IResult,
};

/// represents changes that can be applied to the execution state
#[derive(Default, Debug, Clone)]
pub struct StateChanges {
    /// ledger changes
    pub ledger_changes: LedgerChanges,
    /// asynchronous pool changes
    pub async_pool_changes: AsyncPoolChanges,
}

/// Basic `StateChanges` serializer.
pub struct StateChangesSerializer {
    ledger_changes_serializer: LedgerChangesSerializer,
    async_pool_changes_serializer: AsyncPoolChangesSerializer,
}

impl StateChangesSerializer {
    /// Creates a `StateChangesSerializer`
    pub fn new() -> Self {
        Self {
            ledger_changes_serializer: LedgerChangesSerializer::new(),
            async_pool_changes_serializer: AsyncPoolChangesSerializer::new(),
        }
    }
}

impl Default for StateChangesSerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl Serializer<StateChanges> for StateChangesSerializer {
    fn serialize(&self, value: &StateChanges, buffer: &mut Vec<u8>) -> Result<(), SerializeError> {
        self.ledger_changes_serializer
            .serialize(&value.ledger_changes, buffer)?;
        self.async_pool_changes_serializer
            .serialize(&value.async_pool_changes, buffer)?;
        Ok(())
    }
}

/// Basic `StateChanges` deserializer
pub struct StateChangesDeserializer {
    ledger_changes_deserializer: LedgerChangesDeserializer,
    async_pool_changes_deserializer: AsyncPoolChangesDeserializer,
}

impl StateChangesDeserializer {
    /// Creates a `StateChangesDeserializer`
    pub fn new() -> Self {
        Self {
            ledger_changes_deserializer: LedgerChangesDeserializer::new(),
            async_pool_changes_deserializer: AsyncPoolChangesDeserializer::new(),
        }
    }
}

impl Default for StateChangesDeserializer {
    fn default() -> Self {
        Self::new()
    }
}

impl Deserializer<StateChanges> for StateChangesDeserializer {
    fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        &self,
        buffer: &'a [u8],
    ) -> IResult<&'a [u8], StateChanges, E> {
        context("Failed StateChanges deserialization", |input| {
            tuple((
                context("Failed ledger_changes deserialization", |input| {
                    self.ledger_changes_deserializer.deserialize(input)
                }),
                context("Failed async_pool_changes deserialization", |input| {
                    self.async_pool_changes_deserializer.deserialize(input)
                }),
            ))(input)
        })(buffer)
        .map(|(rest, (ledger_changes, async_pool_changes))| {
            (
                rest,
                StateChanges {
                    ledger_changes,
                    async_pool_changes,
                },
            )
        })
    }
}

impl StateChanges {
    /// extends the current `StateChanges` with another one
    pub fn apply(&mut self, changes: StateChanges) {
        use massa_ledger::Applicable;
        self.ledger_changes.apply(changes.ledger_changes);
        self.async_pool_changes.extend(changes.async_pool_changes);
    }
}
