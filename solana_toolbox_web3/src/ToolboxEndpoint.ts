import {
  AccountInfo,
  Blockhash,
  Connection,
  PublicKey,
  TransactionSignature,
  VersionedTransaction,
} from '@solana/web3.js';
import { ToolboxEndpointExecution } from './ToolboxEndpointExecution';
import {
  decompileTransactionInstructions,
  decompileTransactionPayerAddress,
} from './ToolboxEndpointExecution.decompile';

export class ToolboxEndpoint {
  public static readonly PUBLIC_RPC_URL_MAINNET_BETA =
    'https://api.mainnet-beta.solana.com';
  public static readonly PUBLIC_RPC_URL_TESTNET =
    'https://api.testnet.solana.com';
  public static readonly PUBLIC_RPC_URL_DEVNET =
    'https://api.devnet.solana.com';

  private static urlOrMonikerToUrl = new Map<string, string>([
    ['m', ToolboxEndpoint.PUBLIC_RPC_URL_MAINNET_BETA],
    ['mainnet', ToolboxEndpoint.PUBLIC_RPC_URL_MAINNET_BETA],
    ['mainnet-beta', ToolboxEndpoint.PUBLIC_RPC_URL_MAINNET_BETA],
    ['t', ToolboxEndpoint.PUBLIC_RPC_URL_TESTNET],
    ['testnet', ToolboxEndpoint.PUBLIC_RPC_URL_TESTNET],
    ['d', ToolboxEndpoint.PUBLIC_RPC_URL_DEVNET],
    ['devnet', ToolboxEndpoint.PUBLIC_RPC_URL_DEVNET],
  ]);
  private static urlOrMonikerToCluster = new Map<string, string>([
    [ToolboxEndpoint.PUBLIC_RPC_URL_MAINNET_BETA, 'mainnet-beta'],
    ['mainnet-beta', 'mainnet-beta'],
    ['mainnet', 'mainnet-beta'],
    ['m', 'mainnet-beta'],
    [ToolboxEndpoint.PUBLIC_RPC_URL_TESTNET, 'testnet'],
    ['testnet', 'testnet'],
    ['t', 'testnet'],
    [ToolboxEndpoint.PUBLIC_RPC_URL_DEVNET, 'devnet'],
    ['devnetnet', 'devnetnet'],
    ['d', 'devnetnet'],
  ]);

  private connection: Connection;
  private commitment: 'finalized' | 'confirmed';

  public constructor(
    urlOrMoniker: string,
    commitment: 'finalized' | 'confirmed',
  ) {
    this.connection = new Connection(
      ToolboxEndpoint.getUrlFromUrlOrMoniker(urlOrMoniker),
      commitment,
    );
    this.commitment = commitment;
  }

  public static getUrlFromUrlOrMoniker(urlOrMoniker: string): string {
    return (
      ToolboxEndpoint.urlOrMonikerToUrl.get(urlOrMoniker.toLowerCase()) ??
      urlOrMoniker
    );
  }

  public static getClusterFromUrlOrMoniker(
    urlOrMoniker: string,
  ): string | undefined {
    return ToolboxEndpoint.urlOrMonikerToCluster.get(
      urlOrMoniker.toLowerCase(),
    );
  }

  public async getLatestBlockhash(): Promise<Blockhash> {
    return (await this.connection.getLatestBlockhash()).blockhash;
  }

  public async getBalance(address: PublicKey): Promise<number> {
    return await this.connection.getBalance(address);
  }

  public async getAccount(
    address: PublicKey,
  ): Promise<AccountInfo<Buffer> | undefined> {
    const account = await this.connection.getAccountInfo(address);
    if (account === null) {
      return undefined;
    }
    return account;
  }

  public async simulateTransaction(
    versionedTransaction: VersionedTransaction,
    verifySignatures: boolean,
  ): Promise<ToolboxEndpointExecution> {
    // TODO - resolved lookup tables
    const response = await this.connection.simulateTransaction(
      versionedTransaction,
      {
        sigVerify: verifySignatures,
        replaceRecentBlockhash: false,
        commitment: this.commitment,
        accounts: undefined,
      },
    );
    console.log('simulateTransaction.response', response);
    throw new Error(
      'ToolboxEndpoint.simulateTransaction is not implemented yet. ' +
        'Please use processTransaction instead.',
    );
    /*
    // TODO - convert to executution
    return new ToolboxEndpointExecution({
      slot: -1,
      payer: response.value.accounts?.[0] ?? new PublicKey(0),
      instructions: response.value?.transaction.message.instructions ?? [],
      logs: response.value?.logs ?? null,
      error: response.value?.err ?? null,
      returnData: response.value?.returnData
        ? Buffer.from(response.value.returnData.data, 'base64')
        : null,
      unitsConsumed: response.value?.unitsConsumed ?? null, // TODO - handle this better
    });
    */
  }

  public async processTransaction(
    versionedTransaction: VersionedTransaction,
    verifyPreflight: boolean,
  ): Promise<{
    signature: TransactionSignature;
    execution: ToolboxEndpointExecution;
  }> {
    const signature = await this.connection.sendTransaction(
      versionedTransaction,
      {
        skipPreflight: !verifyPreflight,
        preflightCommitment: this.commitment,
      },
    );
    return { signature, execution: await this.waitUntilExecution(signature) };
  }

  public async getExecution(
    signature: TransactionSignature,
  ): Promise<ToolboxEndpointExecution> {
    const execution = await this.getExecutionIfExists(signature);
    if (execution) {
      return execution;
    }
    throw new Error(`Execution for transaction ${signature} does not exist`);
  }

  private async waitUntilExecution(
    signature: TransactionSignature,
  ): Promise<ToolboxEndpointExecution> {
    const timeoutMs = 60 * 1000;
    const sleepMs = 100;
    let startTime = Date.now();
    while (true) {
      let execution = await this.getExecutionIfExists(signature);
      if (execution) {
        return execution;
      }
      if (Date.now() > startTime + timeoutMs) {
        throw new Error(
          `Timeout while waiting for execution of transaction ${signature}`,
        );
      }
      await new Promise((resolve) => setTimeout(resolve, sleepMs));
    }
  }

  private async getExecutionIfExists(
    signature: TransactionSignature,
  ): Promise<ToolboxEndpointExecution | undefined> {
    const response = await this.connection.getTransaction(signature, {
      commitment: this.commitment,
      maxSupportedTransactionVersion: 0,
    });
    if (!response) {
      return undefined;
    }
    const staticAddresses = response.transaction.message.staticAccountKeys;
    const payer = decompileTransactionPayerAddress(staticAddresses);
    const header = response.transaction.message.header;
    const compiledInstructions = [];
    for (const responseInstruction of response.transaction.message
      .compiledInstructions) {
      compiledInstructions.push({
        programIdIndex: responseInstruction.programIdIndex,
        accountsIndexes: responseInstruction.accountKeyIndexes,
        data: Buffer.from(responseInstruction.data),
      });
    }
    const loadedAddresses = response.meta?.loadedAddresses;
    const instructions = decompileTransactionInstructions(
      header.numRequiredSignatures,
      header.numReadonlySignedAccounts,
      header.numReadonlyUnsignedAccounts,
      staticAddresses,
      loadedAddresses?.writable ?? [],
      loadedAddresses?.readonly ?? [],
      compiledInstructions,
    );
    return new ToolboxEndpointExecution({
      processedTime: response.blockTime
        ? new Date(response.blockTime * 1000)
        : null,
      slot: response.slot,
      payer: payer,
      instructions: instructions,
      logs: response?.meta?.logMessages ?? null,
      error: response?.meta?.err ?? null,
      unitsConsumed: response?.meta?.computeUnitsConsumed ?? null,
    });
  }

  public async searchAddresses(
    programId: PublicKey,
    dataLength?: number,
    dataChunks?: { offset: number; bytes: Buffer }[],
  ): Promise<Set<PublicKey>> {
    const filters = [];
    if (dataLength !== undefined) {
      filters.push({
        dataSize: dataLength,
      });
    }
    if (dataChunks !== undefined) {
      for (const dataChunk of dataChunks) {
        filters.push({
          memcmp: {
            offset: dataChunk.offset,
            encoding: 'base64' as const,
            bytes: dataChunk.bytes.toString('base64'),
          },
        });
      }
    }
    const response = await this.connection.getProgramAccounts(programId, {
      commitment: this.commitment,
      dataSlice: {
        offset: 0,
        length: 0,
      },
      filters: filters,
    });
    const addresses = new Set<PublicKey>();
    for (const finding of response) {
      addresses.add(finding.pubkey);
    }
    return addresses;
  }

  public async searchSignatures(
    address: PublicKey,
    limit: number,
    startBefore?: TransactionSignature,
    rewindUntil?: TransactionSignature,
  ): Promise<TransactionSignature[]> {
    const orderedSignatures = [];
    let oldestKnownSignature = startBefore;
    let retries = 0;
    while (true) {
      const batchSize = Math.min(
        1000,
        rewindUntil ? (retries == 0 ? 10 : 1000) : limit,
      );
      retries++;
      const signatures = await this.connection.getSignaturesForAddress(
        address,
        {
          before: oldestKnownSignature,
          limit: batchSize,
        },
        this.commitment,
      );
      if (signatures.length == 0) {
        return orderedSignatures;
      }
      for (const signature of signatures) {
        const foundSignature = signature.signature;
        orderedSignatures.push(foundSignature);
        if (orderedSignatures.length >= limit) {
          return orderedSignatures;
        }
        if (rewindUntil && foundSignature == rewindUntil) {
          return orderedSignatures;
        }
        oldestKnownSignature = foundSignature;
      }
    }
  }
}
