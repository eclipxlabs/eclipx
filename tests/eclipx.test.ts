import { PublicKey } from "@solana/web3.js";
import { EclipxClient } from "../sdk/src/index";
import { TxStatus } from "../sdk/src/types";

describe("eclipx SDK", () => {
  const config = {
    rpcEndpoint: "https://api.devnet.solana.com",
    programId: "Ec1pXmjiR5DVFhbYr4urnUdFKNhKwgjJXwbkFCfCa5sM",
  };

  const client = new EclipxClient(config);

  test("should derive correct PDA for privacy state", () => {
    const authority = new PublicKey(
      "11111111111111111111111111111111"
    );
    const pda = client.getPrivacyStatePDA(authority);
    expect(pda).toBeInstanceOf(PublicKey);
  });

  test("should return null for non-existent privacy state", async () => {
    const authority = new PublicKey(
      "11111111111111111111111111111111"
    );
    const state = await client.getPrivacyState(authority);
    expect(state).toBeNull();
  });

  test("should return zero score for non-existent account", async () => {
    const authority = new PublicKey(
      "11111111111111111111111111111111"
    );
    const score = await client.getPrivacyScore(authority);
    expect(score).toBe(0);
  });

  test("TxStatus enum values should be correct", () => {
    expect(TxStatus.Pending).toBe(0);
    expect(TxStatus.Routing).toBe(1);
    expect(TxStatus.Compressing).toBe(2);
    expect(TxStatus.Confirmed).toBe(3);
    expect(TxStatus.Failed).toBe(4);
  });
});
