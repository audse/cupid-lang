use cupid_util::ERR_EXPECTED_TYPE;
use crate::{env::SymbolType, Type, Address, PassErr, PassResult};

impl TryFrom<SymbolType> for Type {
    type Error = PassErr;
    fn try_from(value: SymbolType) -> PassResult<Type> {
        match value {
            SymbolType::Type(t) => Ok(t),
            SymbolType::Address(a) => Err((a, ERR_EXPECTED_TYPE))
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