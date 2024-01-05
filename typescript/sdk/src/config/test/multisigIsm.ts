import { IsmType, MultisigConfig, MultisigIsmConfig } from '../../ism/types';
import { ChainMap } from '../../types';
import { buildMultisigIsmConfig } from '../ism';

// the addresses here must line up with the e2e test's validator addresses
// Validators are anvil accounts 4-6
export const chainToValidator: Record<string, string> = {
  test1: '0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65',
  test2: '0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc',
  test3: '0x976EA74026E726554dB657fA54763abd0C3a0aa9',
};

export const chainToMultisig: ChainMap<MultisigConfig> = {
  test1: {
    validators: [chainToValidator['test1']],
    threshold: 1,
  },
  test2: {
    validators: [chainToValidator['test2']],
    threshold: 1,
  },
  test3: {
    validators: [chainToValidator['test3']],
    threshold: 1,
  },
};

// the addresses here must line up with the e2e test's validator addresses
export const multisigIsm: ChainMap<MultisigIsmConfig> = {
  // Validators are anvil accounts 4-6
  test1: buildMultisigIsmConfig(
    IsmType.MESSAGE_ID_MULTISIG,
    chainToMultisig['test1'],
  ),
  test2: buildMultisigIsmConfig(
    IsmType.MERKLE_ROOT_MULTISIG,
    chainToMultisig['test2'],
  ),
  test3: buildMultisigIsmConfig(
    IsmType.MESSAGE_ID_MULTISIG,
    chainToMultisig['test3'],
  ),
};