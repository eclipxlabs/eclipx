import { PublicKey } from "@solana/web3.js";

export interface EclipxConfig {
  rpcEndpoint: string;
  programId: string;
}

export interface PrivacyState {
  authority: PublicKey;
  relayCount: number;
  obfuscationLevel: number;
  isActive: boolean;
  totalTransactions: number;
  privacyScore: number;
  bump: number;
  createdAt: number;
  activatedAt: number;
}

export interface TransactionRecord {
  authority: PublicKey;
  zkProofHash: number[];
  merkleRoot: number[];
  relayPathHash: number[];
  encryptedAmount: number;
  timestamp: number;
  status: number;
  bump: number;
}

export enum TxStatus {
  Pending = 0,
  Routing = 1,
  Compressing = 2,
  Confirmed = 3,
  Failed = 4,
}

export interface RelayNode {
  id: string;
  region: string;
  latencyMs: number;
  isHealthy: boolean;
}

export interface StealthTransferParams {
  recipient: PublicKey;
  amount: number;
  obfuscationLevel: number;
  relayCount: number;
}

// touch: c12c5b9b

// touch: 5a8dbd0a

// touch: b9a32a9b

// touch: 40ec53fc

// touch: 036684e2

// touch: a4077297
