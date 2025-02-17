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

    function initialize(
        address _initialOwner,
        address _rewardsInitiator
    ) public initializer {
        __ServiceManagerBase_init(_initialOwner, _rewardsInitiator);
    }

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

    /// @inheritdoc IServiceManager
    function addPendingAdmin(
        address admin
    ) external onlyOwner {
        // _permissionController.addPendingAdmin({account: address(this), admin: admin});
    }

    /// @inheritdoc IServiceManager
    function removePendingAdmin(
        address pendingAdmin
    ) external onlyOwner {
        // _permissionController.removePendingAdmin({account: address(this), admin: pendingAdmin});
    }

    /// @inheritdoc IServiceManager
    function removeAdmin(
        address admin
    ) external onlyOwner {
        // _permissionController.removeAdmin({account: address(this), admin: admin});
    }

    /// @inheritdoc IServiceManager
    function setAppointee(address appointee, address target, bytes4 selector) external onlyOwner {
        // _permissionController.setAppointee({
        //     account: address(this),
        //     appointee: appointee,
        //     target: target,
        //     selector: selector
        // });
    }

    /// @inheritdoc IServiceManager
    function removeAppointee(
        address appointee,
        address target,
        bytes4 selector
    ) external onlyOwner {
        // _permissionController.removeAppointee({
        //     account: address(this),
        //     appointee: appointee,
        //     target: target,
        //     selector: selector
        // });
    }
        function validate(bytes calldata data, bytes calldata signature) external view
    {
        bytes32 message = keccak256(data);
        bytes32 ethSignedMessageHash = ECDSAUpgradeable.toEthSignedMessageHash(message);
        bytes4 magicValue = IERC1271Upgradeable.isValidSignature.selector;

        // If the registry returns the magicValue, signature is considered valid
        if( magicValue !=
            ECDSAStakeRegistry(stakeRegistry).isValidSignature(
                ethSignedMessageHash,
                signature
            )
        ) {
            revert ILayerServiceManager.InvalidSignature();
        }
    }

}
