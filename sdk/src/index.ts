import { Connection, PublicKey, Transaction, SystemProgram } from "@solana/web3.js";
import { Program, AnchorProvider, BN } from "@coral-xyz/anchor";
import { PrivacyState, TransactionRecord, EclipxConfig } from "./types";

export class EclipxClient {
  private connection: Connection;
  private programId: PublicKey;

  constructor(config: EclipxConfig) {
    this.connection = new Connection(config.rpcEndpoint, "confirmed");
    this.programId = new PublicKey(config.programId);
  }

  async getPrivacyState(authority: PublicKey): Promise<PrivacyState | null> {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("privacy"), authority.toBuffer()],
      this.programId
    );

    const accountInfo = await this.connection.getAccountInfo(pda);
    if (!accountInfo) return null;

    return this.deserializePrivacyState(accountInfo.data);
  }

  async getTransactionRecord(
    authority: PublicKey,
    index: number
  ): Promise<TransactionRecord | null> {
    const indexBuffer = Buffer.alloc(8);
    indexBuffer.writeBigUInt64LE(BigInt(index));

    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("tx_record"), authority.toBuffer(), indexBuffer],
      this.programId
    );

    const accountInfo = await this.connection.getAccountInfo(pda);
    if (!accountInfo) return null;

    return this.deserializeTransactionRecord(accountInfo.data);
  }

  async getPrivacyScore(authority: PublicKey): Promise<number> {
    const state = await this.getPrivacyState(authority);
    if (!state) return 0;
    return state.privacyScore;
  }

  getPrivacyStatePDA(authority: PublicKey): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("privacy"), authority.toBuffer()],
      this.programId
    );
    return pda;
  }

  private deserializePrivacyState(data: Buffer): PrivacyState {
    const offset = 8;
    return {
      authority: new PublicKey(data.subarray(offset, offset + 32)),
      relayCount: data[offset + 32],
      obfuscationLevel: data[offset + 33],
      isActive: data[offset + 34] === 1,
      totalTransactions: Number(data.readBigUInt64LE(offset + 35)),
      privacyScore: data.readUInt16LE(offset + 43),
      bump: data[offset + 45],
      createdAt: Number(data.readBigInt64LE(offset + 46)),
      activatedAt: Number(data.readBigInt64LE(offset + 54)),
    };
  }

  private deserializeTransactionRecord(data: Buffer): TransactionRecord {
    const offset = 8;
    return {
      authority: new PublicKey(data.subarray(offset, offset + 32)),
      zkProofHash: Array.from(data.subarray(offset + 32, offset + 64)),
      merkleRoot: Array.from(data.subarray(offset + 64, offset + 96)),
      relayPathHash: Array.from(data.subarray(offset + 96, offset + 128)),
      encryptedAmount: Number(data.readBigUInt64LE(offset + 128)),
      timestamp: Number(data.readBigInt64LE(offset + 136)),
      status: data[offset + 144],
      bump: data[offset + 145],
    };
  }
}

export { PrivacyState, TransactionRecord, EclipxConfig } from "./types";

// touch: bcfbbc6d

// touch: ab7596ea

// touch: c3afb7bf
