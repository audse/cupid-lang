use cupid_util::ERR_EXPECTED_TYPE;
use crate::{env::symbol_table::SymbolType, Type, Address, PassErr, PassResult};

impl TryFrom<SymbolType> for Type {
    type Error = PassErr;
    fn try_from(value: SymbolType) -> PassResult<Type> {
        match value {
            SymbolType::Type(t) => Ok(t),
            _ => Err((0, ERR_EXPECTED_TYPE))
        }
    }
}

impl From<Address> for SymbolType {
    fn from(a: Address) -> Self {
        Self::Address(a)
    }
}

impl From<Type> for SymbolType {
    fn from(t: Type) -> Self {
        Self::Type(t)
    }
}