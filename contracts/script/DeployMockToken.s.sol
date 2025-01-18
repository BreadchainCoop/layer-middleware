// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import { Script } from "forge-std/Script.sol";
import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockToken is ERC20 {
    constructor(
        string memory _name,
        string memory _symbol,
        uint256 _initialSupply
    ) ERC20(_name, _symbol) {
        _mint(msg.sender, _initialSupply);
    }
}

contract DeployMockTokenScript is Script {
    function run(address recipient) external {
        vm.startBroadcast();

        MockToken mockToken = new MockToken(
            "Mock Token",
            "MKT",
            1_000_000 ether
        );
        mockToken.transfer(recipient, 1_000_000 ether);

        vm.stopBroadcast();
    }
}
