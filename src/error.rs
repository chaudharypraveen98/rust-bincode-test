use bincode::ErrorKind as BinErr;
use core::fmt;
use serde_json::Error as SerdeErr;
use std::io::Error;

#[derive(Debug)]
pub enum AppErrorType {
    SerdeSerializeError,
    BincodeSerializeError,
    IOError,
}
#[derive(Debug)]
pub struct AppError {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: AppErrorType,
}

impl From<Error> for AppError {
    fn from(error: Error) -> AppError {
        AppError {
            cause: Some(error.to_string()),
            message: None,
            error_type: AppErrorType::IOError,
        }
    }
}
impl From<SerdeErr> for AppError {
    fn from(error: SerdeErr) -> AppError {
        AppError {
            cause: Some(error.to_string()),
            message: None,
            error_type: AppErrorType::SerdeSerializeError,
        }
    }
}
impl From<BinErr> for AppError {
    fn from(error: BinErr) -> AppError {
        AppError {
            cause: Some(error.to_string()),
            message: None,
            error_type: AppErrorType::BincodeSerializeError,
        }
    }
}
impl From<Box<BinErr>> for AppError {
    fn from(error: Box<BinErr>) -> AppError {
        AppError {
            cause: Some(error.to_string()),
            message: None,
            error_type: AppErrorType::BincodeSerializeError,
        }
    }
}
// impl fmt::Display for AppError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}",self.message())
//     }
// }
// impl AppError {
//     // we are handling the none. function name should match field name
//     fn message(&self) -> String {
//         match &*self {
//             // Error message is found then clone otherwise default message
//             AppError {
//                 cause: _,
//                 message: Some(message),
//                 error_type: _,
//             } => message.clone(),
//             AppError {
//                 cause: _,
//                 message: None,
//                 error_type: AppErrorType::BincodeSerializeError,
//             } => "Bincode serialization error".to_string(),
//             AppError {
//                 cause: Some(cause),
//                 message: None,
//                 error_type: AppErrorType::SerdeSerializeError,
//             } => cause.clone(),
//             _ => "An unexpected error has occured".to_string(),
//         }
//     }
// }
