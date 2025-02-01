// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

interface ILayerServiceManager {
    struct SignedPayload {
        bytes data;
        bytes signature;
    }
}

