// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity >=0.8.0;
// ============ Internal Imports ============
import {AbstractRoutingIsm} from "./AbstractRoutingIsm.sol";
import {IMailbox} from "../../../interfaces/IMailbox.sol";
import {IInterchainSecurityModule} from "../../../interfaces/IInterchainSecurityModule.sol";
import {Message} from "../../libs/Message.sol";
import {InterchainAccountMessage} from "../../libs/middleware/InterchainAccountMessage.sol";

/**
 * @title InterchainAccountIsm
 */
contract InterchainAccountIsm is AbstractRoutingIsm {
    IMailbox private immutable mailbox;

    // ============ Constructor ============
    constructor(address _mailbox) {
        mailbox = IMailbox(_mailbox);
    }

    // ============ Public Functions ============

    function route(bytes calldata _message)
        public
        view
        virtual
        override
        returns (IInterchainSecurityModule)
    {
        address _ism = InterchainAccountMessage.ism(Message.body(_message));
        if (_ism == address(0)) {
            return mailbox.defaultIsm();
        } else {
            return IInterchainSecurityModule(_ism);
        }
    }
}
