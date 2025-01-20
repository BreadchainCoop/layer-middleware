// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import {IAVSRegistrar} from "@eigenlayer/contracts/interfaces/IAVSRegistrar.sol";

// TODO: decide on AVSRegistar logic
// Dummy AVSRegistrar contract for now
contract LayerAVSRegistrar is IAVSRegistrar {
    function registerOperator(address operator, uint32[] calldata operatorSetIds, bytes calldata data) external {}
    function deregisterOperator(address operator, uint32[] calldata operatorSetIds) external {}
    fallback () external {}
}
