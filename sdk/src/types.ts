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

// touch: 100c02fa

// touch: 7dc65438

// touch: fdd232f0

// touch: 5ec357b6

// touch: 9ec9d9b9

// touch: 7d26c2f2

// touch: 114d5106

// touch: bdb0e90f

// touch: afc27a6f

// touch: 40355884

// touch: 50a3bbdb

// touch: dad73c5b
