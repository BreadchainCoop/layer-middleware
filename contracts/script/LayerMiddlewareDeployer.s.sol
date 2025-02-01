// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/Test.sol";
import {LayerMiddlewareDeploymentLib} from "./utils/LayerMiddlewareDeplomentLib.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";
import {StrategyBase} from "@eigenlayer/contracts/strategies/StrategyBase.sol";
import {ERC20Mock} from "../test/ERC20Mock.sol";
import {TransparentUpgradeableProxy} from
    "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {StrategyFactory} from "@eigenlayer/contracts/strategies/StrategyFactory.sol";
import {StrategyManager} from "@eigenlayer/contracts/core/StrategyManager.sol";


import {
    Quorum,
    StrategyParams,
    IStrategy
} from "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";

contract LayerMiddlewareDeployer is Script {
    using CoreDeploymentLib for *;
    using UpgradeableProxyLib for address;
    address private deployer;
    address proxyAdmin;
    IStrategy helloWorldStrategy;
    CoreDeploymentLib.DeploymentData coreDeployment;
    LayerMiddlewareDeploymentLib.DeploymentData layerMiddlewareDeployment;
    Quorum internal quorum;
    ERC20Mock token;
    function setUp() public virtual {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");
        coreDeployment = CoreDeploymentLib.readDeploymentJson("deployments/core/", block.chainid);
       
        token = new ERC20Mock();
        helloWorldStrategy = IStrategy(StrategyFactory(coreDeployment.strategyFactory).deployNewStrategy(token));

        quorum.strategies.push(
            StrategyParams({strategy: helloWorldStrategy, multiplier: 10_000})
        );
    }

    function run() external {
        vm.startBroadcast(deployer);
        proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();

        layerMiddlewareDeployment =
            LayerMiddlewareDeploymentLib.deployContracts(proxyAdmin, coreDeployment, quorum);

        layerMiddlewareDeployment.strategy = address(helloWorldStrategy);
        layerMiddlewareDeployment.token = address(token);
        vm.stopBroadcast();

        verifyDeployment();
        LayerMiddlewareDeploymentLib.writeDeploymentJson(layerMiddlewareDeployment);
    }

    function verifyDeployment() internal view {
        require(
            layerMiddlewareDeployment.stakeRegistry != address(0), "StakeRegistry address cannot be zero"
        );
        require(
            layerMiddlewareDeployment.layerServiceManager != address(0),
            "layerServiceManager address cannot be zero"
        );
        require(layerMiddlewareDeployment.strategy != address(0), "Strategy address cannot be zero");
        require(proxyAdmin != address(0), "ProxyAdmin address cannot be zero");
        require(
            coreDeployment.delegationManager != address(0),
            "DelegationManager address cannot be zero"
        );
        require(coreDeployment.avsDirectory != address(0), "AVSDirectory address cannot be zero");
    }
}
