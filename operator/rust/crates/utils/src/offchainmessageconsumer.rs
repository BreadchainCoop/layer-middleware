///Module containing a contract's types and functions.
/**

```solidity
library ILayerSDK {
    struct Task { bytes32 dataHash; bytes signatureData; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod ILayerSDK {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct Task { bytes32 dataHash; bytes signatureData; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Task {
        pub dataHash: alloy::sol_types::private::FixedBytes<32>,
        pub signatureData: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Task> for UnderlyingRustTuple<'_> {
            fn from(value: Task) -> Self {
                (value.dataHash, value.signatureData)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Task {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    dataHash: tuple.0,
                    signatureData: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Task {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Task {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.dataHash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signatureData,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Task {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Task {
            const NAME: &'static str = "Task";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Task(bytes32 dataHash,bytes signatureData)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.dataHash)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signatureData,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Task {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.dataHash,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signatureData,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.dataHash,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signatureData,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ILayerSDK`](self) contract instance.

See the [wrapper's documentation](`ILayerSDKInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ILayerSDKInstance<T, P, N> {
        ILayerSDKInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ILayerSDK`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ILayerSDK`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ILayerSDKInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ILayerSDKInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ILayerSDKInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ILayerSDKInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ILayerSDK`](self) contract instance.

See the [wrapper's documentation](`ILayerSDKInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> ILayerSDKInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ILayerSDKInstance<T, P, N> {
            ILayerSDKInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ILayerSDKInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ILayerSDKInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library ILayerSDK {
    struct Task {
        bytes32 dataHash;
        bytes signatureData;
    }
}

interface OffchainMessageConsumer {
    error InvalidLayerTask();

    constructor(address _stakeRegistry);

    function STAKE_REGISTRY() external view returns (address);
    function validateOffchainMessage(ILayerSDK.Task memory _task) external view returns (bool _isValid);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_stakeRegistry",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "STAKE_REGISTRY",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ECDSAStakeRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "validateOffchainMessage",
    "inputs": [
      {
        "name": "_task",
        "type": "tuple",
        "internalType": "struct ILayerSDK.Task",
        "components": [
          {
            "name": "dataHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "signatureData",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "_isValid",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "error",
    "name": "InvalidLayerTask",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod OffchainMessageConsumer {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a060405234801561000f575f80fd5b5060405161060738038061060783398101604081905261002e9161003f565b6001600160a01b031660805261006c565b5f6020828403121561004f575f80fd5b81516001600160a01b0381168114610065575f80fd5b9392505050565b60805161057d61008a5f395f818160650152610184015261057d5ff3fe608060405234801561000f575f80fd5b5060043610610034575f3560e01c80636ea1bd3814610038578063761de19f14610060575b5f80fd5b61004b6100463660046102db565b6100ac565b60405190151581526020015b60405180910390f35b6100877f000000000000000000000000000000000000000000000000000000000000000081565b60405173ffffffffffffffffffffffffffffffffffffffff9091168152602001610057565b5f6100b6826103b7565b6100bf8161013e565b6100f5576040517f100f934b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610137835f01356040518060400160405280600d81526020017f48656c6c6f2c20576f726c642100000000000000000000000000000000000000815250610242565b9392505050565b805160208201516040517f1626ba7e0000000000000000000000000000000000000000000000000000000081525f9273ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001692631626ba7e926101b9929190600401610496565b602060405180830381865afa1580156101d4573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101f891906104ed565b7fffffffff00000000000000000000000000000000000000000000000000000000167f1626ba7e000000000000000000000000000000000000000000000000000000001492915050565b5f8082604051602001610255919061052c565b604080518083037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe001815282825280516020918201207f19457468657265756d205369676e6564204d6573736167653a0a33320000000082850152603c8085019190915282518085039091018152605c9093019091528151910120939093149392505050565b5f602082840312156102eb575f80fd5b813567ffffffffffffffff811115610301575f80fd5b820160408185031215610137575f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040805190810167ffffffffffffffff8111828210171561036257610362610312565b60405290565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff811182821017156103af576103af610312565b604052919050565b5f604082360312156103c7575f80fd5b6103cf61033f565b8235815260208084013567ffffffffffffffff808211156103ee575f80fd5b9085019036601f830112610400575f80fd5b81358181111561041257610412610312565b610442847fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601610368565b91508082523684828501011115610457575f80fd5b80848401858401375f908201840152918301919091525092915050565b5f5b8381101561048e578181015183820152602001610476565b50505f910152565b828152604060208201525f82518060408401526104ba816060850160208701610474565b601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016919091016060019392505050565b5f602082840312156104fd575f80fd5b81517fffffffff0000000000000000000000000000000000000000000000000000000081168114610137575f80fd5b5f825161053d818460208701610474565b919091019291505056fea2646970667358221220de0b20ec09adec60a63e9eee14b6c7bcde82b3952fe1da2efeee39bcea1c584464736f6c63430008170033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x06\x078\x03\x80a\x06\x07\x839\x81\x01`@\x81\x90Ra\0.\x91a\0?V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0lV[_` \x82\x84\x03\x12\x15a\0OW_\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0eW_\x80\xFD[\x93\x92PPPV[`\x80Qa\x05}a\0\x8A_9_\x81\x81`e\x01Ra\x01\x84\x01Ra\x05}_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80cn\xA1\xBD8\x14a\08W\x80cv\x1D\xE1\x9F\x14a\0`W[_\x80\xFD[a\0Ka\0F6`\x04a\x02\xDBV[a\0\xACV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x87\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0WV[_a\0\xB6\x82a\x03\xB7V[a\0\xBF\x81a\x01>V[a\0\xF5W`@Q\x7F\x10\x0F\x93K\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x017\x83_\x015`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FHello, World!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x02BV[\x93\x92PPPV[\x80Q` \x82\x01Q`@Q\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\x16&\xBA~\x92a\x01\xB9\x92\x91\x90`\x04\x01a\x04\x96V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xD4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF8\x91\x90a\x04\xEDV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x92\x91PPV[_\x80\x82`@Q` \x01a\x02U\x91\x90a\x05,V[`@\x80Q\x80\x83\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x82\x85\x01R`<\x80\x85\x01\x91\x90\x91R\x82Q\x80\x85\x03\x90\x91\x01\x81R`\\\x90\x93\x01\x90\x91R\x81Q\x91\x01 \x93\x90\x93\x14\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x02\xEBW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x01W_\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x017W_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03bWa\x03ba\x03\x12V[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xAFWa\x03\xAFa\x03\x12V[`@R\x91\x90PV[_`@\x826\x03\x12\x15a\x03\xC7W_\x80\xFD[a\x03\xCFa\x03?V[\x825\x81R` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x03\xEEW_\x80\xFD[\x90\x85\x01\x906`\x1F\x83\x01\x12a\x04\0W_\x80\xFD[\x815\x81\x81\x11\x15a\x04\x12Wa\x04\x12a\x03\x12V[a\x04B\x84\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x03hV[\x91P\x80\x82R6\x84\x82\x85\x01\x01\x11\x15a\x04WW_\x80\xFD[\x80\x84\x84\x01\x85\x84\x017_\x90\x82\x01\x84\x01R\x91\x83\x01\x91\x90\x91RP\x92\x91PPV[_[\x83\x81\x10\x15a\x04\x8EW\x81\x81\x01Q\x83\x82\x01R` \x01a\x04vV[PP_\x91\x01RV[\x82\x81R`@` \x82\x01R_\x82Q\x80`@\x84\x01Ra\x04\xBA\x81``\x85\x01` \x87\x01a\x04tV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01``\x01\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x04\xFDW_\x80\xFD[\x81Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x017W_\x80\xFD[_\x82Qa\x05=\x81\x84` \x87\x01a\x04tV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xDE\x0B \xEC\t\xAD\xEC`\xA6>\x9E\xEE\x14\xB6\xC7\xBC\xDE\x82\xB3\x95/\xE1\xDA.\xFE\xEE9\xBC\xEA\x1CXDdsolcC\0\x08\x17\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610034575f3560e01c80636ea1bd3814610038578063761de19f14610060575b5f80fd5b61004b6100463660046102db565b6100ac565b60405190151581526020015b60405180910390f35b6100877f000000000000000000000000000000000000000000000000000000000000000081565b60405173ffffffffffffffffffffffffffffffffffffffff9091168152602001610057565b5f6100b6826103b7565b6100bf8161013e565b6100f5576040517f100f934b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610137835f01356040518060400160405280600d81526020017f48656c6c6f2c20576f726c642100000000000000000000000000000000000000815250610242565b9392505050565b805160208201516040517f1626ba7e0000000000000000000000000000000000000000000000000000000081525f9273ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001692631626ba7e926101b9929190600401610496565b602060405180830381865afa1580156101d4573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101f891906104ed565b7fffffffff00000000000000000000000000000000000000000000000000000000167f1626ba7e000000000000000000000000000000000000000000000000000000001492915050565b5f8082604051602001610255919061052c565b604080518083037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe001815282825280516020918201207f19457468657265756d205369676e6564204d6573736167653a0a33320000000082850152603c8085019190915282518085039091018152605c9093019091528151910120939093149392505050565b5f602082840312156102eb575f80fd5b813567ffffffffffffffff811115610301575f80fd5b820160408185031215610137575f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040805190810167ffffffffffffffff8111828210171561036257610362610312565b60405290565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff811182821017156103af576103af610312565b604052919050565b5f604082360312156103c7575f80fd5b6103cf61033f565b8235815260208084013567ffffffffffffffff808211156103ee575f80fd5b9085019036601f830112610400575f80fd5b81358181111561041257610412610312565b610442847fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601610368565b91508082523684828501011115610457575f80fd5b80848401858401375f908201840152918301919091525092915050565b5f5b8381101561048e578181015183820152602001610476565b50505f910152565b828152604060208201525f82518060408401526104ba816060850160208701610474565b601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016919091016060019392505050565b5f602082840312156104fd575f80fd5b81517fffffffff0000000000000000000000000000000000000000000000000000000081168114610137575f80fd5b5f825161053d818460208701610474565b919091019291505056fea2646970667358221220de0b20ec09adec60a63e9eee14b6c7bcde82b3952fe1da2efeee39bcea1c584464736f6c63430008170033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80cn\xA1\xBD8\x14a\08W\x80cv\x1D\xE1\x9F\x14a\0`W[_\x80\xFD[a\0Ka\0F6`\x04a\x02\xDBV[a\0\xACV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x87\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0WV[_a\0\xB6\x82a\x03\xB7V[a\0\xBF\x81a\x01>V[a\0\xF5W`@Q\x7F\x10\x0F\x93K\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x017\x83_\x015`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FHello, World!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x02BV[\x93\x92PPPV[\x80Q` \x82\x01Q`@Q\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\x16&\xBA~\x92a\x01\xB9\x92\x91\x90`\x04\x01a\x04\x96V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xD4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF8\x91\x90a\x04\xEDV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x92\x91PPV[_\x80\x82`@Q` \x01a\x02U\x91\x90a\x05,V[`@\x80Q\x80\x83\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x82\x85\x01R`<\x80\x85\x01\x91\x90\x91R\x82Q\x80\x85\x03\x90\x91\x01\x81R`\\\x90\x93\x01\x90\x91R\x81Q\x91\x01 \x93\x90\x93\x14\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x02\xEBW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x01W_\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x017W_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03bWa\x03ba\x03\x12V[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xAFWa\x03\xAFa\x03\x12V[`@R\x91\x90PV[_`@\x826\x03\x12\x15a\x03\xC7W_\x80\xFD[a\x03\xCFa\x03?V[\x825\x81R` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x03\xEEW_\x80\xFD[\x90\x85\x01\x906`\x1F\x83\x01\x12a\x04\0W_\x80\xFD[\x815\x81\x81\x11\x15a\x04\x12Wa\x04\x12a\x03\x12V[a\x04B\x84\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x03hV[\x91P\x80\x82R6\x84\x82\x85\x01\x01\x11\x15a\x04WW_\x80\xFD[\x80\x84\x84\x01\x85\x84\x017_\x90\x82\x01\x84\x01R\x91\x83\x01\x91\x90\x91RP\x92\x91PPV[_[\x83\x81\x10\x15a\x04\x8EW\x81\x81\x01Q\x83\x82\x01R` \x01a\x04vV[PP_\x91\x01RV[\x82\x81R`@` \x82\x01R_\x82Q\x80`@\x84\x01Ra\x04\xBA\x81``\x85\x01` \x87\x01a\x04tV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01``\x01\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x04\xFDW_\x80\xFD[\x81Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x017W_\x80\xFD[_\x82Qa\x05=\x81\x84` \x87\x01a\x04tV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xDE\x0B \xEC\t\xAD\xEC`\xA6>\x9E\xEE\x14\xB6\xC7\xBC\xDE\x82\xB3\x95/\xE1\xDA.\xFE\xEE9\xBC\xEA\x1CXDdsolcC\0\x08\x17\x003",
    );
    /**Custom error with signature `InvalidLayerTask()` and selector `0x100f934b`.
```solidity
error InvalidLayerTask();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidLayerTask {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidLayerTask> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidLayerTask) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidLayerTask {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidLayerTask {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidLayerTask()";
            const SELECTOR: [u8; 4] = [16u8, 15u8, 147u8, 75u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _stakeRegistry);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _stakeRegistry: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._stakeRegistry,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _stakeRegistry: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._stakeRegistry,
                    ),
                )
            }
        }
    };
    /**Function with signature `STAKE_REGISTRY()` and selector `0x761de19f`.
```solidity
function STAKE_REGISTRY() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct STAKE_REGISTRYCall {}
    ///Container type for the return parameters of the [`STAKE_REGISTRY()`](STAKE_REGISTRYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct STAKE_REGISTRYReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<STAKE_REGISTRYCall> for UnderlyingRustTuple<'_> {
                fn from(value: STAKE_REGISTRYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for STAKE_REGISTRYCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<STAKE_REGISTRYReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: STAKE_REGISTRYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for STAKE_REGISTRYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for STAKE_REGISTRYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = STAKE_REGISTRYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "STAKE_REGISTRY()";
            const SELECTOR: [u8; 4] = [118u8, 29u8, 225u8, 159u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `validateOffchainMessage((bytes32,bytes))` and selector `0x6ea1bd38`.
```solidity
function validateOffchainMessage(ILayerSDK.Task memory _task) external view returns (bool _isValid);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateOffchainMessageCall {
        pub _task: <ILayerSDK::Task as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`validateOffchainMessage((bytes32,bytes))`](validateOffchainMessageCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateOffchainMessageReturn {
        pub _isValid: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ILayerSDK::Task,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ILayerSDK::Task as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validateOffchainMessageCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: validateOffchainMessageCall) -> Self {
                    (value._task,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validateOffchainMessageCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _task: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validateOffchainMessageReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: validateOffchainMessageReturn) -> Self {
                    (value._isValid,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validateOffchainMessageReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _isValid: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validateOffchainMessageCall {
            type Parameters<'a> = (ILayerSDK::Task,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validateOffchainMessageReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validateOffchainMessage((bytes32,bytes))";
            const SELECTOR: [u8; 4] = [110u8, 161u8, 189u8, 56u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<ILayerSDK::Task as alloy_sol_types::SolType>::tokenize(&self._task),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`OffchainMessageConsumer`](self) function calls.
    pub enum OffchainMessageConsumerCalls {
        STAKE_REGISTRY(STAKE_REGISTRYCall),
        validateOffchainMessage(validateOffchainMessageCall),
    }
    #[automatically_derived]
    impl OffchainMessageConsumerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [110u8, 161u8, 189u8, 56u8],
            [118u8, 29u8, 225u8, 159u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for OffchainMessageConsumerCalls {
        const NAME: &'static str = "OffchainMessageConsumerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 2usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::STAKE_REGISTRY(_) => {
                    <STAKE_REGISTRYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validateOffchainMessage(_) => {
                    <validateOffchainMessageCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<OffchainMessageConsumerCalls>] = &[
                {
                    fn validateOffchainMessage(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OffchainMessageConsumerCalls> {
                        <validateOffchainMessageCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OffchainMessageConsumerCalls::validateOffchainMessage)
                    }
                    validateOffchainMessage
                },
                {
                    fn STAKE_REGISTRY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OffchainMessageConsumerCalls> {
                        <STAKE_REGISTRYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OffchainMessageConsumerCalls::STAKE_REGISTRY)
                    }
                    STAKE_REGISTRY
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::STAKE_REGISTRY(inner) => {
                    <STAKE_REGISTRYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validateOffchainMessage(inner) => {
                    <validateOffchainMessageCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::STAKE_REGISTRY(inner) => {
                    <STAKE_REGISTRYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validateOffchainMessage(inner) => {
                    <validateOffchainMessageCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`OffchainMessageConsumer`](self) custom errors.
    pub enum OffchainMessageConsumerErrors {
        InvalidLayerTask(InvalidLayerTask),
    }
    #[automatically_derived]
    impl OffchainMessageConsumerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[16u8, 15u8, 147u8, 75u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for OffchainMessageConsumerErrors {
        const NAME: &'static str = "OffchainMessageConsumerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::InvalidLayerTask(_) => {
                    <InvalidLayerTask as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<OffchainMessageConsumerErrors>] = &[
                {
                    fn InvalidLayerTask(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OffchainMessageConsumerErrors> {
                        <InvalidLayerTask as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OffchainMessageConsumerErrors::InvalidLayerTask)
                    }
                    InvalidLayerTask
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::InvalidLayerTask(inner) => {
                    <InvalidLayerTask as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InvalidLayerTask(inner) => {
                    <InvalidLayerTask as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`OffchainMessageConsumer`](self) contract instance.

See the [wrapper's documentation](`OffchainMessageConsumerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> OffchainMessageConsumerInstance<T, P, N> {
        OffchainMessageConsumerInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _stakeRegistry: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<OffchainMessageConsumerInstance<T, P, N>>,
    > {
        OffchainMessageConsumerInstance::<T, P, N>::deploy(provider, _stakeRegistry)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _stakeRegistry: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        OffchainMessageConsumerInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _stakeRegistry)
    }
    /**A [`OffchainMessageConsumer`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OffchainMessageConsumer`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OffchainMessageConsumerInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for OffchainMessageConsumerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OffchainMessageConsumerInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > OffchainMessageConsumerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`OffchainMessageConsumer`](self) contract instance.

See the [wrapper's documentation](`OffchainMessageConsumerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _stakeRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<OffchainMessageConsumerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _stakeRegistry);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _stakeRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { _stakeRegistry },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> OffchainMessageConsumerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OffchainMessageConsumerInstance<T, P, N> {
            OffchainMessageConsumerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > OffchainMessageConsumerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`STAKE_REGISTRY`] function.
        pub fn STAKE_REGISTRY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, STAKE_REGISTRYCall, N> {
            self.call_builder(&STAKE_REGISTRYCall {})
        }
        ///Creates a new call builder for the [`validateOffchainMessage`] function.
        pub fn validateOffchainMessage(
            &self,
            _task: <ILayerSDK::Task as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, validateOffchainMessageCall, N> {
            self.call_builder(
                &validateOffchainMessageCall {
                    _task,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > OffchainMessageConsumerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
