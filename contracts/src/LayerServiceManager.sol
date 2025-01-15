// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import {ECDSAServiceManagerBase} from
    "@eigenlayer-middleware/src/unaudited/ECDSAServiceManagerBase.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {IServiceManager} from "@eigenlayer-middleware/src/interfaces/IServiceManager.sol";
import {ECDSAUpgradeable} from
    "@openzeppelin-upgrades/contracts/utils/cryptography/ECDSAUpgradeable.sol";
import {IERC1271Upgradeable} from "@openzeppelin-upgrades/contracts/interfaces/IERC1271Upgradeable.sol";
import {ILayerServiceManager} from "./ILayerServiceManager.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {IAllocationManager, IAllocationManagerTypes} from "@eigenlayer/contracts/interfaces/IAllocationManager.sol";
import {ISignatureUtils} from "@eigenlayer/contracts/interfaces/ISignatureUtils.sol";
import {IAVSRegistrar} from "@eigenlayer/contracts/interfaces/IAVSRegistrar.sol";
import {IStrategy} from "@eigenlayer/contracts/interfaces/IStrategy.sol";

/**
 * @title Primary entrypoint for procuring services from LayerMiddleware.
 * @author Eigen Labs, Inc.
 */
contract LayerServiceManager is ECDSAServiceManagerBase, ILayerServiceManager {
    using ECDSAUpgradeable for bytes32;

    constructor(
        address _avsDirectory,
        address _stakeRegistry,
        address _rewardsCoordinator,
        address _delegationManager,
        address _allocationManager
    )
        ECDSAServiceManagerBase(
            _avsDirectory,
            _stakeRegistry,
            _rewardsCoordinator,
            _delegationManager,
            _allocationManager
        )
    {}

    /// NOTE: All OperatorSet functions are `onlyOwner`
    /// although `createOperatorSets` SHOULD be `onlyRegistryCoordinator`
    /// and `addStrategyToOperatorSet`, `removeStrategiesFromOperatorSet` SHOULD be `onlyStakeRegistry`
    /// ---
    /// There is a discrepency between `ServiceManagerBase.sol` and and `ECDSAServiceManagerBase.sol`
    /// and between `StakeRegistry.sol` and `ECDSAStakeRegistry.sol`

    /// @notice Creates new operator sets with the given parameters
    function createOperatorSets(IAllocationManager.CreateSetParams[] memory params) external onlyOwner {
        IAllocationManager(allocationManager).createOperatorSets(address(this), params);
    }

    /// @notice Adds strategies to an existing operator set
    function addStrategyToOperatorSet(uint32 operatorSetId, IStrategy[] memory strategies) external onlyOwner {
        IAllocationManager(allocationManager).addStrategiesToOperatorSet(address(this), operatorSetId, strategies);
    }

    /// @notice Removes strategies from an existing operator set
    function removeStrategiesFromOperatorSet(uint32 operatorSetId, IStrategy[] memory strategies) external onlyOwner {
        IAllocationManager(allocationManager).removeStrategiesFromOperatorSet(address(this), operatorSetId, strategies);
    }

    /// @notice Deregisters an operator from operator sets
    function deregisterOperatorFromOperatorSets(
        address operator,
        uint32[] calldata operatorSetIds
    ) external {
        // Implementation logic here
    }

    /// @notice Registers an operator to operator sets
    function registerOperatorToOperatorSets(
        address operator,
        uint32[] calldata operatorSetIds,
        ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature
    ) external {
        // Implementation logic here
    }

    /// @notice Creates AVS rewards submission
    function createAVSRewardsSubmission(IRewardsCoordinator.RewardsSubmission[] calldata rewardsSubmissions) external override {
        // Implementation logic here
    }

    /// @notice Slashes an operator
    function slashOperator(
        IAllocationManagerTypes.SlashingParams memory params
    ) external {
        // Implementation logic here
    }
}
