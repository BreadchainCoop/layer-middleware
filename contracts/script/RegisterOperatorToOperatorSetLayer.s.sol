// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

// use forge:
// RUST_LOG=forge,foundry=trace forge script script/tasks/register_operator_to_operatorSet.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast --sig "run(string memory configFile)" -- <DEPLOYMENT_OUTPUT_JSON>
// RUST_LOG=forge,foundry=trace forge script script/tasks/register_operator_to_operatorSet.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast --sig "run(string memory configFile)" -- local/slashing_output.json
import {RegisterOperatorToOperatorSets} from "@eigenlayer-scripts/tasks/register_operator_to_operatorSet.s.sol";

// reimport of eigen's operator set script that can be later modified to layer's needs
contract RegisterOperatorToOperatorSetLayer is RegisterOperatorToOperatorSets {}
