/**
 * @docs
 * Contains internal types not exported by taquito
 */

export interface BlockResponse {
  chain_id: string;
  hash: string;
  protocol: string;
}

export interface TransferConfirmationResponse {
  completed: boolean;
  currentConfirmation: number;
  expectedConfirmation: number;
  block: BlockResponse;
  isInCurrentBranch: () => Promise<boolean>;
}

export interface Estimate {
  burnFeeMutez: number;
  gasLimit: number;
  minimalFeeMutez: number;
  opSize: number | string;
  storageLimit: number;
  suggestedFeeMutez: number;
  totalCost: number;
  usingBaseFeeMutez: number;
  consumedMilligas: number;
}

export interface RevealParams {
  fee?: number;
  gasLimit?: number;
  storageLimit?: number;
}

export interface TransferParams {
  to: string;
  amount: number;
  source?: string;
  fee?: number;
  gasLimit?: number;
  storageLimit?: number;
  mutez?: boolean;
}

export interface OriginateParams {
  code: string;
  storage: any;
  balance?: string;
  delegate?: string;
  fee?: number;
  gasLimit?: number;
  storageLimit?: number;
  mutez?: boolean;
  init?: string;
}

export interface Sign {
  bytes: string;
  sig: string;
  prefixSig: string;
  sbytes: string;
}