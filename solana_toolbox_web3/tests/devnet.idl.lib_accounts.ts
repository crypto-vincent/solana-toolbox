import { PublicKey } from '@solana/web3.js';
import { ToolboxEndpoint } from '../src/ToolboxEndpoint';
import { ToolboxIdlService } from '../src';

let systemProgramId = new PublicKey('11111111111111111111111111111111');
let tokenProgramId = new PublicKey(
  'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
);
let ataProgramId = new PublicKey(
  'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
);

it('run', async () => {
  // Create the endpoint
  let endpoint = new ToolboxEndpoint('devnet', 'confirmed');
  // Prepare known accounts available on devnet
  let programId = new PublicKey('UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j');
  let programData = new PublicKey(
    '9rtcXuviJngSZTRSCXxsHyd6qaWpqWSQ56SNumXAuLJ1',
  );
  let mintAuthority = new PublicKey(
    '7poxwHXi62Cwa57xdrpfoW2bUF7s8iXm1CU4jJqYPhu',
  );
  let user = new PublicKey('Ady55LhZxWFABzdg8NCNTAZv5XstBqyNZYCMfWqW3Rq9');
  let collateralMint = new PublicKey(
    'EsQycjp856vTPvrxMuH1L6ymd5K63xT7aULGepiTcgM3',
  );
  // TODO - should this be a util function?
  let userCollateral = PublicKey.findProgramAddressSync(
    [user.toBuffer(), tokenProgramId.toBuffer(), collateralMint.toBuffer()],
    ataProgramId,
  )[0];
  let nameRecordHeader = new PublicKey(
    '8EodedXFv8DAJ6jGTg4DVXaBVJTVL3o4T2BWwTJTTJjw',
  );
  let nameRecordOwner = new PublicKey(
    '8aU2gq8XgzNZr8z4noV87Sx8a3EV29gmi645qQERsaTD',
  );
  // Prepare the IDL service
  let idlService = new ToolboxIdlService();
  // TODO - check ATA resolution
  // Check the state of a system account
  await assertAccountInfo(idlService, endpoint, user, 'system', 'Wallet', null);
  // Check the state of the collateral mint
  await assertAccountInfo(
    idlService,
    endpoint,
    collateralMint,
    'spl_token',
    'TokenMint',
    {
      mint_authority: mintAuthority.toBase58(),
      supply: BigInt(1000000000000000),
      decimals: 9,
      is_initialized: true,
      freeze_authority: null,
    },
  );
  // Check the state of the collateral ATA
  await assertAccountInfo(
    idlService,
    endpoint,
    userCollateral,
    'spl_token',
    'TokenAccount',
    {
      mint: collateralMint.toBase58(),
      owner: user.toBase58(),
      amount: BigInt(996906108000000),
      delegate: null,
      state: 'Initialized',
      is_native: null,
      delegated_amount: BigInt(0),
      close_authority: null,
    },
  );
  // Check the state of a known program
  await assertAccountInfo(
    idlService,
    endpoint,
    programId,
    'bpf_loader_upgradeable',
    'Program',
    {
      program_data: programData.toBase58(),
    },
  );
  // Check the state of a known program's executable data
  await assertAccountInfo(
    idlService,
    endpoint,
    programData,
    'bpf_loader_upgradeable',
    'ProgramData',
    {
      slot: BigInt(347133692),
      upgrade_authority: mintAuthority.toBase58(),
    },
  );
  // Check the state of a known name record header
  await assertAccountInfo(
    idlService,
    endpoint,
    nameRecordHeader,
    'spl_name_service',
    'NameRecordHeader',
    {
      class: systemProgramId.toBase58(),
      owner: nameRecordOwner.toBase58(),
      parent_name: systemProgramId.toBase58(),
    },
  );
});

async function assertAccountInfo(
  idlService: ToolboxIdlService,
  endpoint: ToolboxEndpoint,
  address: PublicKey,
  programName: string,
  accountName: string,
  accountState: any,
) {
  let accountInfo = await idlService.getAndInferAndDecodeAccount(
    endpoint,
    address,
  );
  expect(accountInfo.program.metadata.name).toStrictEqual(programName);
  expect(accountInfo.account.name).toStrictEqual(accountName);
  expect(accountInfo.state).toStrictEqual(accountState);
}
