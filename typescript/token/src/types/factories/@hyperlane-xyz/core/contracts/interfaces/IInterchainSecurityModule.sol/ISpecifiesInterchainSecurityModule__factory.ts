/* Autogenerated file. Do not edit manually. */

/* tslint:disable */

/* eslint-disable */
import type { Provider } from '@ethersproject/providers';
import { Contract, Signer, utils } from 'ethers';

import type {
  ISpecifiesInterchainSecurityModule,
  ISpecifiesInterchainSecurityModuleInterface,
} from '../../../../../../@hyperlane-xyz/core/contracts/interfaces/IInterchainSecurityModule.sol/ISpecifiesInterchainSecurityModule';

const _abi = [
  {
    inputs: [],
    name: 'interchainSecurityModule',
    outputs: [
      {
        internalType: 'contract IInterchainSecurityModule',
        name: '',
        type: 'address',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
];

export class ISpecifiesInterchainSecurityModule__factory {
  static readonly abi = _abi;
  static createInterface(): ISpecifiesInterchainSecurityModuleInterface {
    return new utils.Interface(
      _abi,
    ) as ISpecifiesInterchainSecurityModuleInterface;
  }
  static connect(
    address: string,
    signerOrProvider: Signer | Provider,
  ): ISpecifiesInterchainSecurityModule {
    return new Contract(
      address,
      _abi,
      signerOrProvider,
    ) as ISpecifiesInterchainSecurityModule;
  }
}