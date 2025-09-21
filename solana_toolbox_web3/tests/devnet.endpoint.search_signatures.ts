import { PublicKey } from '@solana/web3.js';
import { ToolboxEndpoint } from '../src/ToolboxEndpoint';

it('run', async () => {
  // Create the endpoint pointing to devnet
  const endpoint = new ToolboxEndpoint('devnet', 'confirmed');
  // Tests constants
  const programId = new PublicKey(
    'UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j',
  );
  const signatureN4 =
    '4gqmT5jrEZ35BEkq2x1K8WHwhWVz9Z46Un5w1sddLvmx1c5fUTyzd4J389bcCsHgCBzQam4Qn5MdKuw5ydUyJ62L';
  const signatureN3 =
    '5KUaXrTAjeuHg3XPYo8ve6UJR5u5vP8cS9JDEKoG6Cq3V2gBp52QHQcfKkeHLcfDDMpEf27aRrZ5EtG8bBjHAXf5';
  const signatureN2 =
    'LWzVhua28HoamZ81JuB1EQJ8JLsLdtTTNVXUWJcUzUwqVPSu4SpQhjiUfaxhFdL2TPPcmmeN8sJKe1QSeMRiP4L';
  const signatureN1 =
    '3eHgwNJHqSHYHroGZimcCQKSWzyr3rrohRfXG3YmtpL4FAkbkZ8G4STwVBXsd3QTrURkNiUqttqfRCxuc6s7NJzP';
  const unknownSignature =
    '5gqmT5jrEZ35BEkq2x1K8WHwhWVz9Z46Un5w1sddLvmx1c5fUTyzd4J389bcCsHgCBzQam4Qn5MdKuw5ydUyJ62L';
  // Search all the way through the history until transaction n2
  const searchUntilN2 = await endpoint.searchSignatures(
    programId,
    10000,
    undefined,
    signatureN2,
  );
  expect(searchUntilN2.length).toBeGreaterThan(200);
  expect(searchUntilN2[searchUntilN2.length - 3]).toBe(signatureN4);
  expect(searchUntilN2[searchUntilN2.length - 2]).toBe(signatureN3);
  expect(searchUntilN2[searchUntilN2.length - 1]).toBe(signatureN2);
  // Search from before the 4th transaction all the way to the start
  const searchBeforeN4 = await endpoint.searchSignatures(
    programId,
    10000,
    signatureN4,
  );
  expect(searchBeforeN4.length).toBe(3);
  expect(searchBeforeN4[0]).toBe(signatureN3);
  expect(searchBeforeN4[1]).toBe(signatureN2);
  expect(searchBeforeN4[2]).toBe(signatureN1);
  // Search from before the 4th transaction until the 2nd
  const searchBeforeN4UntilN2 = await endpoint.searchSignatures(
    programId,
    10000,
    signatureN4,
    signatureN2,
  );
  expect(searchBeforeN4UntilN2.length).toBe(2);
  expect(searchBeforeN4UntilN2[0]).toBe(signatureN3);
  expect(searchBeforeN4UntilN2[1]).toBe(signatureN2);
  // Search from before an invalid signature (must return nothing)
  const searchBeforeInvalid = await endpoint.searchSignatures(
    programId,
    10000,
    unknownSignature,
  );
  expect(searchBeforeInvalid.length).toBe(0);
  // Search until an invalid signature (must return everything)
  const searchUntilInvalid = await endpoint.searchSignatures(
    programId,
    10000,
    undefined,
    unknownSignature,
  );
  expect(searchUntilInvalid).toStrictEqual(searchUntilN2.concat([signatureN1]));
  // Search with a limit
  const searchLimited = await endpoint.searchSignatures(programId, 100);
  expect(searchLimited).toStrictEqual(searchUntilN2.slice(0, 100));
  // Search invalid order
  const searchOrderInvalid = await endpoint.searchSignatures(
    programId,
    100,
    signatureN3,
    signatureN4,
  );
  expect(searchOrderInvalid).toStrictEqual([signatureN2, signatureN1]);
});
