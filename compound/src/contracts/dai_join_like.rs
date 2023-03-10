pub use dai_join_like::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod dai_join_like {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "DaiJoinLike was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static DAIJOINLIKE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"dai\",\"outputs\":[{\"internalType\":\"contract GemLike\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"join\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"vat\",\"outputs\":[{\"internalType\":\"contract VatLike\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct DaiJoinLike<M>(ethers::contract::Contract<M>);
    impl<M> Clone for DaiJoinLike<M> {
        fn clone(&self) -> Self {
            DaiJoinLike(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for DaiJoinLike<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for DaiJoinLike<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DaiJoinLike))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> DaiJoinLike<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), DAIJOINLIKE_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `dai` (0xf4b9fa75) function"]
        pub fn dai(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([244, 185, 250, 117], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exit` (0xef693bed) function"]
        pub fn exit(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 105, 59, 237], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `join` (0x3b4da69f) function"]
        pub fn join(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 77, 166, 159], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vat` (0x36569e77) function"]
        pub fn vat(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([54, 86, 158, 119], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for DaiJoinLike<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `dai` function with signature `dai()` and selector `[244, 185, 250, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "dai", abi = "dai()")]
    pub struct DaiCall;
    #[doc = "Container type for all input parameters for the `exit` function with signature `exit(address,uint256)` and selector `[239, 105, 59, 237]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "exit", abi = "exit(address,uint256)")]
    pub struct ExitCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `join` function with signature `join(address,uint256)` and selector `[59, 77, 166, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "join", abi = "join(address,uint256)")]
    pub struct JoinCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `vat` function with signature `vat()` and selector `[54, 86, 158, 119]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "vat", abi = "vat()")]
    pub struct VatCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum DaiJoinLikeCalls {
        Dai(DaiCall),
        Exit(ExitCall),
        Join(JoinCall),
        Vat(VatCall),
    }
    impl ethers::core::abi::AbiDecode for DaiJoinLikeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <DaiCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DaiJoinLikeCalls::Dai(decoded));
            }
            if let Ok(decoded) = <ExitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DaiJoinLikeCalls::Exit(decoded));
            }
            if let Ok(decoded) = <JoinCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DaiJoinLikeCalls::Join(decoded));
            }
            if let Ok(decoded) = <VatCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DaiJoinLikeCalls::Vat(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for DaiJoinLikeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                DaiJoinLikeCalls::Dai(element) => element.encode(),
                DaiJoinLikeCalls::Exit(element) => element.encode(),
                DaiJoinLikeCalls::Join(element) => element.encode(),
                DaiJoinLikeCalls::Vat(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for DaiJoinLikeCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DaiJoinLikeCalls::Dai(element) => element.fmt(f),
                DaiJoinLikeCalls::Exit(element) => element.fmt(f),
                DaiJoinLikeCalls::Join(element) => element.fmt(f),
                DaiJoinLikeCalls::Vat(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DaiCall> for DaiJoinLikeCalls {
        fn from(var: DaiCall) -> Self {
            DaiJoinLikeCalls::Dai(var)
        }
    }
    impl ::std::convert::From<ExitCall> for DaiJoinLikeCalls {
        fn from(var: ExitCall) -> Self {
            DaiJoinLikeCalls::Exit(var)
        }
    }
    impl ::std::convert::From<JoinCall> for DaiJoinLikeCalls {
        fn from(var: JoinCall) -> Self {
            DaiJoinLikeCalls::Join(var)
        }
    }
    impl ::std::convert::From<VatCall> for DaiJoinLikeCalls {
        fn from(var: VatCall) -> Self {
            DaiJoinLikeCalls::Vat(var)
        }
    }
    #[doc = "Container type for all return fields from the `dai` function with signature `dai()` and selector `[244, 185, 250, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DaiReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `vat` function with signature `vat()` and selector `[54, 86, 158, 119]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct VatReturn(pub ethers::core::types::Address);
}
