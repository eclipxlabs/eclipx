import BN from 'bn.js';

import { createClient, PredicateKind } from '../src';

describe('createClient', () => {
  test('wires prover and disclosure together for a happy path', () => {
    const { prover, disclosure } = createClient();
    const predicate = disclosure.minSol(5);
    const blinding = new Uint8Array(32).fill(5);
    const proof = prover.prove({
      predicate,
      blinding,
      witness: { balanceLamports: new BN(20_000_000_000), saltHex: 'beef' },
    });
    expect(proof.predicateKind).toBe(PredicateKind.MinBalance);
    expect(proof.proofHash).toHaveLength(32);
    expect(proof.publicInputsHash).toHaveLength(32);
  });

  test('namespace affects commitment', () => {
    const clientA = createClient('fixr-policy-v1');
    const clientB = createClient('fixr-policy-v2');
    const predicate = clientA.disclosure.minSol(1);
    const blinding = new Uint8Array(32).fill(9);
    const commitmentA = clientA.prover.commitment(predicate.kind, predicate.threshold, blinding);
    const commitmentB = clientB.prover.commitment(predicate.kind, predicate.threshold, blinding);
    expect(commitmentA).not.toEqual(commitmentB);
  });
});
// rev-01074
