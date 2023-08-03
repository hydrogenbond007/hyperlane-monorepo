import type {
  EventFragment,
  FunctionFragment,
  Result,
} from '@ethersproject/abi';
import type { Listener, Provider } from '@ethersproject/providers';
import type {
  BaseContract,
  BigNumber,
  BigNumberish,
  BytesLike,
  CallOverrides,
  ContractTransaction,
  Overrides,
  PopulatedTransaction,
  Signer,
  utils,
} from 'ethers';

import type {
  OnEvent,
  TypedEvent,
  TypedEventFilter,
  TypedListener,
} from '../../../../../common';

export interface ERC721URIStorageUpgradeableInterface extends utils.Interface {
  functions: {
    'approve(address,uint256)': FunctionFragment;
    'balanceOf(address)': FunctionFragment;
    'getApproved(uint256)': FunctionFragment;
    'isApprovedForAll(address,address)': FunctionFragment;
    'name()': FunctionFragment;
    'ownerOf(uint256)': FunctionFragment;
    'safeTransferFrom(address,address,uint256)': FunctionFragment;
    'safeTransferFrom(address,address,uint256,bytes)': FunctionFragment;
    'setApprovalForAll(address,bool)': FunctionFragment;
    'supportsInterface(bytes4)': FunctionFragment;
    'symbol()': FunctionFragment;
    'tokenURI(uint256)': FunctionFragment;
    'transferFrom(address,address,uint256)': FunctionFragment;
  };
  getFunction(
    nameOrSignatureOrTopic:
      | 'approve'
      | 'balanceOf'
      | 'getApproved'
      | 'isApprovedForAll'
      | 'name'
      | 'ownerOf'
      | 'safeTransferFrom(address,address,uint256)'
      | 'safeTransferFrom(address,address,uint256,bytes)'
      | 'setApprovalForAll'
      | 'supportsInterface'
      | 'symbol'
      | 'tokenURI'
      | 'transferFrom',
  ): FunctionFragment;
  encodeFunctionData(
    functionFragment: 'approve',
    values: [string, BigNumberish],
  ): string;
  encodeFunctionData(functionFragment: 'balanceOf', values: [string]): string;
  encodeFunctionData(
    functionFragment: 'getApproved',
    values: [BigNumberish],
  ): string;
  encodeFunctionData(
    functionFragment: 'isApprovedForAll',
    values: [string, string],
  ): string;
  encodeFunctionData(functionFragment: 'name', values?: undefined): string;
  encodeFunctionData(
    functionFragment: 'ownerOf',
    values: [BigNumberish],
  ): string;
  encodeFunctionData(
    functionFragment: 'safeTransferFrom(address,address,uint256)',
    values: [string, string, BigNumberish],
  ): string;
  encodeFunctionData(
    functionFragment: 'safeTransferFrom(address,address,uint256,bytes)',
    values: [string, string, BigNumberish, BytesLike],
  ): string;
  encodeFunctionData(
    functionFragment: 'setApprovalForAll',
    values: [string, boolean],
  ): string;
  encodeFunctionData(
    functionFragment: 'supportsInterface',
    values: [BytesLike],
  ): string;
  encodeFunctionData(functionFragment: 'symbol', values?: undefined): string;
  encodeFunctionData(
    functionFragment: 'tokenURI',
    values: [BigNumberish],
  ): string;
  encodeFunctionData(
    functionFragment: 'transferFrom',
    values: [string, string, BigNumberish],
  ): string;
  decodeFunctionResult(functionFragment: 'approve', data: BytesLike): Result;
  decodeFunctionResult(functionFragment: 'balanceOf', data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: 'getApproved',
    data: BytesLike,
  ): Result;
  decodeFunctionResult(
    functionFragment: 'isApprovedForAll',
    data: BytesLike,
  ): Result;
  decodeFunctionResult(functionFragment: 'name', data: BytesLike): Result;
  decodeFunctionResult(functionFragment: 'ownerOf', data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: 'safeTransferFrom(address,address,uint256)',
    data: BytesLike,
  ): Result;
  decodeFunctionResult(
    functionFragment: 'safeTransferFrom(address,address,uint256,bytes)',
    data: BytesLike,
  ): Result;
  decodeFunctionResult(
    functionFragment: 'setApprovalForAll',
    data: BytesLike,
  ): Result;
  decodeFunctionResult(
    functionFragment: 'supportsInterface',
    data: BytesLike,
  ): Result;
  decodeFunctionResult(functionFragment: 'symbol', data: BytesLike): Result;
  decodeFunctionResult(functionFragment: 'tokenURI', data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: 'transferFrom',
    data: BytesLike,
  ): Result;
  events: {
    'Approval(address,address,uint256)': EventFragment;
    'ApprovalForAll(address,address,bool)': EventFragment;
    'Initialized(uint8)': EventFragment;
    'Transfer(address,address,uint256)': EventFragment;
  };
  getEvent(nameOrSignatureOrTopic: 'Approval'): EventFragment;
  getEvent(nameOrSignatureOrTopic: 'ApprovalForAll'): EventFragment;
  getEvent(nameOrSignatureOrTopic: 'Initialized'): EventFragment;
  getEvent(nameOrSignatureOrTopic: 'Transfer'): EventFragment;
}
export interface ApprovalEventObject {
  owner: string;
  approved: string;
  tokenId: BigNumber;
}
export declare type ApprovalEvent = TypedEvent<
  [string, string, BigNumber],
  ApprovalEventObject
>;
export declare type ApprovalEventFilter = TypedEventFilter<ApprovalEvent>;
export interface ApprovalForAllEventObject {
  owner: string;
  operator: string;
  approved: boolean;
}
export declare type ApprovalForAllEvent = TypedEvent<
  [string, string, boolean],
  ApprovalForAllEventObject
>;
export declare type ApprovalForAllEventFilter =
  TypedEventFilter<ApprovalForAllEvent>;
export interface InitializedEventObject {
  version: number;
}
export declare type InitializedEvent = TypedEvent<
  [number],
  InitializedEventObject
>;
export declare type InitializedEventFilter = TypedEventFilter<InitializedEvent>;
export interface TransferEventObject {
  from: string;
  to: string;
  tokenId: BigNumber;
}
export declare type TransferEvent = TypedEvent<
  [string, string, BigNumber],
  TransferEventObject
>;
export declare type TransferEventFilter = TypedEventFilter<TransferEvent>;
export interface ERC721URIStorageUpgradeable extends BaseContract {
  connect(signerOrProvider: Signer | Provider | string): this;
  attach(addressOrName: string): this;
  deployed(): Promise<this>;
  interface: ERC721URIStorageUpgradeableInterface;
  queryFilter<TEvent extends TypedEvent>(
    event: TypedEventFilter<TEvent>,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined,
  ): Promise<Array<TEvent>>;
  listeners<TEvent extends TypedEvent>(
    eventFilter?: TypedEventFilter<TEvent>,
  ): Array<TypedListener<TEvent>>;
  listeners(eventName?: string): Array<Listener>;
  removeAllListeners<TEvent extends TypedEvent>(
    eventFilter: TypedEventFilter<TEvent>,
  ): this;
  removeAllListeners(eventName?: string): this;
  off: OnEvent<this>;
  on: OnEvent<this>;
  once: OnEvent<this>;
  removeListener: OnEvent<this>;
  functions: {
    approve(
      to: string,
      tokenId: BigNumberish,
      overrides?: Overrides & {
        from?: string | Promise<string>;
      },
    ): Promise<ContractTransaction>;
    balanceOf(owner: string, overrides?: CallOverrides): Promise<[BigNumber]>;
    getApproved(
      tokenId: BigNumberish,
      overrides?: CallOverrides,
    ): Promise<[string]>;
    isApprovedForAll(
      owner: string,
      operator: string,
      overrides?: CallOverrides,
    ): Promise<[boolean]>;
    name(overrides?: CallOverrides): Promise<[string]>;
    ownerOf(
      tokenId: BigNumberish,
      overrides?: CallOverrides,
    ): Promise<[string]>;
    'safeTransferFrom(address,address,uint256)'(
      from: string,
      to: string,
      tokenId: BigNumberish,
      overrides?: Overrides & {
        from?: string | Promise<string>;
      },
    ): Promise<ContractTransaction>;
    'safeTransferFrom(address,address,uint256,bytes)'(
      from: string,
      to: string,
      tokenId: BigNumberish,
      data: BytesLike,
      overrides?: Overrides & {
        from?: string | Promise<string>;
      },
    ): Promise<ContractTransaction>;
    setApprovalForAll(
      operator: string,
      approved: boolean,
      overrides?: Overrides & {
        from?: string | Promise<string>;
      },
    ): Promise<ContractTransaction>;
    supportsInterface(
      interfaceId: BytesLike,
      overrides?: CallOverrides,
    ): Promise<[boolean]>;
    symbol(overrides?: CallOverrides): Promise<[string]>;
    tokenURI(
      tokenId: BigNumberish,
      overrides?: CallOverrides,
    ): Promise<[string]>;
    transferFrom(
      from: string,
      to: string,
      tokenId: BigNumberish,
      overrides?: Overrides & {
        from?: string | Promise<string>;
      },
    ): Promise<ContractTransaction>;
  };
  approve(
    to: string,
    tokenId: BigNumberish,
    overrides?: Overrides & {
      from?: string | Promise<string>;
    },
  ): Promise<ContractTransaction>;
  balanceOf(owner: string, overrides?: CallOverrides): Promise<BigNumber>;
  getApproved(
    tokenId: BigNumberish,
    overrides?: CallOverrides,
  ): Promise<string>;
  isApprovedForAll(
    owner: string,
    operator: string,
    overrides?: CallOverrides,
  ): Promise<boolean>;
  name(overrides?: CallOverrides): Promise<string>;
  ownerOf(tokenId: BigNumberish, overrides?: CallOverrides): Promise<string>;
  'safeTransferFrom(address,address,uint256)'(
    from: string,
    to: string,
    tokenId: BigNumberish,
    overrides?: Overrides & {
      from?: string | Promise<string>;
    },
  ): Promise<ContractTransaction>;
  'safeTransferFrom(address,address,uint256,bytes)'(
    from: string,
    to: string,
    tokenId: BigNumberish,
    data: BytesLike,
    overrides?: Overrides & {
      from?: string | Promise<string>;
    },
  ): Promise<ContractTransaction>;
  setApprovalForAll(
    operator: string,
    approved: boolean,
    overrides?: Overrides & {
      from?: string | Promise<string>;
    },
  ): Promise<ContractTransaction>;
  supportsInterface(
    interfaceId: BytesLike,
    overrides?: CallOverrides,
  ): Promise<boolean>;
  symbol(overrides?: CallOverrides): Promise<string>;
  tokenURI(tokenId: BigNumberish, overrides?: CallOverrides): Promise<string>;
  transferFrom(
    from: string,
    to: string,
    tokenId: BigNumberish,
    overrides?: Overrides & {
      from?: string | Promise<string>;
    },
  ): Promise<ContractTransaction>;
  callStatic: {
    approve(
      to: string,
      tokenId: BigNumberish,
      overrides?: CallOverrides,
    ): Promise<void>;
    balanceOf(owner: string, overrides?: CallOverrides): Promise<BigNumber>;
    getApproved(
      tokenId: BigNumberish,
      overrides?: CallOverrides,
    ): Promise<string>;
    isApprovedForAll(
      owner: string,
      operator: string,
      overrides?: CallOverrides,
    ): Promise<boolean>;
    name(overrides?: CallOverrides): Promise<string>;
    ownerOf(tokenId: BigNumberish, overrides?: CallOverrides): Promise<string>;
    'safeTransferFrom(address,address,uint256)'(
      from: string,
      to: string,
      tokenId: BigNumberish,
      overrides?: CallOverrides,
    ): Promise<void>;
    'safeTransferFrom(address,address,uint256,bytes)'(
      from: string,
      to: string,
      tokenId: BigNumberish,
      data: BytesLike,
      overrides?: CallOverrides,
    ): Promise<void>;
    setApprovalForAll(
      operator: string,
      approved: boolean,
      overrides?: CallOverrides,
    ): Promise<void>;
    supportsInterface(
      interfaceId: BytesLike,
      overrides?: CallOverrides,
    ): Promise<boolean>;
    symbol(overrides?: CallOverrides): Promise<string>;
    tokenURI(tokenId: BigNumberish, overrides?: CallOverrides): Promise<string>;
    transferFrom(
      from: string,
      to: string,
      tokenId: BigNumberish,
      overrides?: CallOverrides,
    ): Promise<void>;
  };
  filters: {
    'Approval(address,address,uint256)'(
      owner?: string | null,
      approved?: string | null,
      tokenId?: BigNumberish | null,
    ): ApprovalEventFilter;
    Approval(
      owner?: string | null,
      approved?: string | null,
      tokenId?: BigNumberish | null,
    ): ApprovalEventFilter;
    'ApprovalForAll(address,address,bool)'(
      owner?: string | null,
      operator?: string | null,
      approved?: null,
    ): ApprovalForAllEventFilter;
    ApprovalForAll(
      owner?: string | null,
      operator?: string | null,
      approved?: null,
    ): ApprovalForAllEventFilter;
    'Initialized(uint8)'(version?: null): InitializedEventFilter;
    Initialized(version?: null): InitializedEventFilter;
    'Transfer(address,address,uint256)'(
      from?: string | null,
      to?: string | null,
      tokenId?: BigNumberish | null,
    ): TransferEventFilter;
    Transfer(
      from?: string | null,
      to?: string | null,
      tokenId?: BigNumberish | null,
    ): TransferEventFilter;
  };
  estimateGas: {
    approve(
      to: string,
      tokenId: BigNumberish,
      overrides?: Overrides & {
        from?: string | Promise<string>;
      },
    ): Promise<BigNumber>;
    balanceOf(owner: string, overrides?: CallOverrides): Promise<BigNumber>;
    getApproved(
      tokenId: BigNumberish,
      overrides?: CallOverrides,
    ): Promise<BigNumber>;
    isApprovedForAll(
      owner: string,
      operator: string,
      overrides?: CallOverrides,
    ): Promise<BigNumber>;
    name(overrides?: CallOverrides): Promise<BigNumber>;
    ownerOf(
      tokenId: BigNumberish,
      overrides?: CallOverrides,
    ): Promise<BigNumber>;
    'safeTransferFrom(address,address,uint256)'(
      from: string,
      to: string,
      tokenId: BigNumberish,
      overrides?: Overrides & {
        from?: string | Promise<string>;
      },
    ): Promise<BigNumber>;
    'safeTransferFrom(address,address,uint256,bytes)'(
      from: string,
      to: string,
      tokenId: BigNumberish,
      data: BytesLike,
      overrides?: Overrides & {
        from?: string | Promise<string>;
      },
    ): Promise<BigNumber>;
    setApprovalForAll(
      operator: string,
      approved: boolean,
      overrides?: Overrides & {
        from?: string | Promise<string>;
      },
    ): Promise<BigNumber>;
    supportsInterface(
      interfaceId: BytesLike,
      overrides?: CallOverrides,
    ): Promise<BigNumber>;
    symbol(overrides?: CallOverrides): Promise<BigNumber>;
    tokenURI(
      tokenId: BigNumberish,
      overrides?: CallOverrides,
    ): Promise<BigNumber>;
    transferFrom(
      from: string,
      to: string,
      tokenId: BigNumberish,
      overrides?: Overrides & {
        from?: string | Promise<string>;
      },
    ): Promise<BigNumber>;
  };
  populateTransaction: {
    approve(
      to: string,
      tokenId: BigNumberish,
      overrides?: Overrides & {
        from?: string | Promise<string>;
      },
    ): Promise<PopulatedTransaction>;
    balanceOf(
      owner: string,
      overrides?: CallOverrides,
    ): Promise<PopulatedTransaction>;
    getApproved(
      tokenId: BigNumberish,
      overrides?: CallOverrides,
    ): Promise<PopulatedTransaction>;
    isApprovedForAll(
      owner: string,
      operator: string,
      overrides?: CallOverrides,
    ): Promise<PopulatedTransaction>;
    name(overrides?: CallOverrides): Promise<PopulatedTransaction>;
    ownerOf(
      tokenId: BigNumberish,
      overrides?: CallOverrides,
    ): Promise<PopulatedTransaction>;
    'safeTransferFrom(address,address,uint256)'(
      from: string,
      to: string,
      tokenId: BigNumberish,
      overrides?: Overrides & {
        from?: string | Promise<string>;
      },
    ): Promise<PopulatedTransaction>;
    'safeTransferFrom(address,address,uint256,bytes)'(
      from: string,
      to: string,
      tokenId: BigNumberish,
      data: BytesLike,
      overrides?: Overrides & {
        from?: string | Promise<string>;
      },
    ): Promise<PopulatedTransaction>;
    setApprovalForAll(
      operator: string,
      approved: boolean,
      overrides?: Overrides & {
        from?: string | Promise<string>;
      },
    ): Promise<PopulatedTransaction>;
    supportsInterface(
      interfaceId: BytesLike,
      overrides?: CallOverrides,
    ): Promise<PopulatedTransaction>;
    symbol(overrides?: CallOverrides): Promise<PopulatedTransaction>;
    tokenURI(
      tokenId: BigNumberish,
      overrides?: CallOverrides,
    ): Promise<PopulatedTransaction>;
    transferFrom(
      from: string,
      to: string,
      tokenId: BigNumberish,
      overrides?: Overrides & {
        from?: string | Promise<string>;
      },
    ): Promise<PopulatedTransaction>;
  };
}
//# sourceMappingURL=ERC721URIStorageUpgradeable.d.ts.map