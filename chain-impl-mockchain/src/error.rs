//! Errors taht may happen in the ledger and mockchain.
use crate::transaction::*;
use chain_addr::Address;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum Error {
    /// If the Ledger could not find the given input in the UTxO list it will
    /// report this error.
    InputDoesNotResolve(UtxoPointer),

    /// if the Ledger finds that the input has already been used once in a given
    /// transaction or block of transactions it will report this error.
    ///
    /// the input here is the given input used twice,
    /// the output here is the output set in the first occurrence of the input, it
    /// will provide a bit of information to the user to figure out what went wrong
    DoubleSpend(UtxoPointer, Output<Address>),

    /// This error will happen if the input was already set and is now replaced
    /// by another output.
    ///
    /// I.E: the value output has changed but the input is the same. This should not
    /// happen since changing the output will change the transaction identifier
    /// associated to this output.
    ///
    /// first the input in common, then the original output and finally the new output
    InputWasAlreadySet(UtxoPointer, Output<Address>, Output<Address>),

    /// error occurs if the signature is invalid: either does not match the initial output
    /// or it is not cryptographically valid.
    InvalidSignature(UtxoPointer, Output<Address>, Witness),

    /// error occurs when one of the witness does not sing entire
    /// transaction properly.
    InvalidTxSignature(Witness),

    /// Transaction sum is not equal to zero, this means that we
    /// either generate or lose some money during the transaction.
    TransactionSumIsNonZero(u64, u64),

    /// Transaction does not have enough signatures.
    /// First value represents number of inputs (required signatures)
    /// Send value represents actual number of singatures.
    NotEnoughSignatures(usize, usize),

    /// Transaction output has a value of zero.
    ZeroOutput(Output<Address>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InputDoesNotResolve(_) => write!(f, "Input does not resolve to an UTxO"),
            Error::DoubleSpend(_, _) => write!(f, "UTxO spent twice in the same transaction"),
            Error::InputWasAlreadySet(_, _, _) => {
                write!(f, "Input was already present in the Ledger")
            }
            Error::InvalidSignature(_, _, _) => write!(f, "Input is not signed properly"),
            Error::InvalidTxSignature(_) => write!(f, "Transaction was not signed"),
            Error::TransactionSumIsNonZero(inp, out) => write!(
                f,
                "Transaction values do not match input is {}, output is {}",
                inp, out
            ),
            Error::NotEnoughSignatures(required, actual) => write!(
                f,
                "Transaction has not enough signatures: {} out of {}",
                actual, required
            ),
            Error::ZeroOutput(_) => write!(f, "Transaction output has a value of zero"),
        }
    }
}
impl std::error::Error for Error {}
