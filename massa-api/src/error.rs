// Copyright (c) 2022 MASSA LABS <info@massa.net>

use displaydoc::Display;
use massa_consensus_exports::error::ConsensusError;
use massa_execution_exports::ExecutionError;
use massa_hash::MassaHashError;
use massa_models::error::ModelsError;
use massa_network_exports::NetworkError;
use massa_protocol_exports::ProtocolError;
use massa_time::TimeError;
use massa_wallet::WalletError;
use thiserror::Error;

#[non_exhaustive]
#[derive(Display, Error, Debug)]
pub enum ApiError {
    /// too many arguments error: {0}
    TooManyArguments(String),
    /// send channel error: {0}
    SendChannelError(String),
    /// receive channel error: {0}
    ReceiveChannelError(String),
    /// `massa_hash` error: {0}
    MassaHashError(#[from] MassaHashError),
    /// consensus error: {0}
    ConsensusError(#[from] Box<ConsensusError>),
    /// execution error: {0}
    ExecutionError(#[from] ExecutionError),
    /// network error: {0}
    NetworkError(#[from] NetworkError),
    /// protocol error: {0}
    ProtocolError(#[from] ProtocolError),
    /// models error: {0}
    ModelsError(#[from] ModelsError),
    /// time error: {0}
    TimeError(#[from] TimeError),
    /// wallet error: {0}
    WalletError(#[from] WalletError),
    /// not found
    NotFound,
    /// inconsistency: {0}
    InconsistencyError(String),
    /// missing command sender {0}
    MissingCommandSender(String),
    /// missing configuration {0}
    MissingConfig(String),
    /// the wrong API (either Public or Private) was called
    WrongAPI,
}

impl From<ApiError> for jsonrpc_core::Error {
    fn from(err: ApiError) -> Self {
        jsonrpc_core::Error {
            code: jsonrpc_core::ErrorCode::ServerError(500),
            message: err.to_string(),
            data: None,
        }
    }
}

impl std::convert::From<ConsensusError> for ApiError {
    fn from(err: ConsensusError) -> Self {
        ApiError::ConsensusError(Box::new(err))
    }
}
