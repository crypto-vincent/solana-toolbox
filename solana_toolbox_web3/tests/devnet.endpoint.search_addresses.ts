import { PublicKey } from '@solana/web3.js';
import { ToolboxEndpoint } from '../src/ToolboxEndpoint';

it('run', async () => {
  // Create the endpoint pointing to devnet
  const endpoint = new ToolboxEndpoint('devnet', 'confirmed');
  // Tests constants
  const programId = new PublicKey(
    'UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j',
  );
  const discriminator = Buffer.from([50, 40, 49, 11, 157, 220, 229, 192]);
  const blobFromAddress1 = new PublicKey(
    'Ady55LhZxWFABzdg8NCNTAZv5XstBqyNZYCMfWqW3Rq9',
  ).toBuffer();
  const blobFromAddress2 = new PublicKey(
    'EsQycjp856vTPvrxMuH1L6ymd5K63xT7aULGepiTcgM3',
  ).toBuffer();
  // Searching accounts with no filters, will return all the program's accounts
  const searchUnfiltered = await endpoint.searchAddresses(programId);
  expect(searchUnfiltered.size).not.toBe(0);
  // Searching accounts by matching on the discriminator
  const searchByDiscriminator = await endpoint.searchAddresses(
    programId,
    undefined,
    [{ offset: 0, bytes: discriminator }],
  );
  expect(searchByDiscriminator.size).toBeLessThan(searchUnfiltered.size);
  expect(searchByDiscriminator.size).toBe(6);
  // Searching accounts by matching the exact account size
  const searchByDataLength = await endpoint.searchAddresses(programId, 680);
  expect(searchByDataLength).toStrictEqual(searchByDiscriminator);
  // Some account are known to be the exception
  const searchByDiscriminatorWithoutOther = new Set(searchByDiscriminator);
  for (const account of searchByDiscriminatorWithoutOther) {
    if (
      account.equals(
        new PublicKey('GAHCWMw8Uc1wpXJS23bL1Hxtb2XFGNDmBvEN12gUiArq'),
      )
    ) {
      searchByDiscriminatorWithoutOther.delete(account);
    }
  }
  // Searching accounts by matching a public key from the data content
  const searchByDataBlob1 = await endpoint.searchAddresses(
    programId,
    undefined,
    [{ offset: 17, bytes: blobFromAddress1 }],
  );
  expect(searchByDataBlob1).toStrictEqual(searchByDiscriminatorWithoutOther);
  // Searching accounts by matching a public key from the data content
  const searchByDataBlob2 = await endpoint.searchAddresses(
    programId,
    undefined,
    [{ offset: 49, bytes: blobFromAddress2 }],
  );
  expect(searchByDataBlob2).toStrictEqual(searchByDiscriminator);
  // Searching accounts by applying all the restrictions at once
  const searchByEverything = await endpoint.searchAddresses(programId, 680, [
    { offset: 17, bytes: blobFromAddress1 },
    { offset: 0, bytes: discriminator },
    { offset: 49, bytes: blobFromAddress2 },
  ]);
  expect(searchByEverything).toStrictEqual(searchByDiscriminatorWithoutOther);
  // Searching accounts by applying one correct and one wrong filter
  const searchByFailure = await endpoint.searchAddresses(programId, 680, [
    { offset: 0, bytes: discriminator },
    { offset: 8, bytes: Buffer.from([42]) },
  ]);
  expect(searchByFailure.size).toBe(0);
});
