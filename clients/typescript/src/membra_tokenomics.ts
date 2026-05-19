/**
 * MEMBRA Tokenomics Solana TypeScript Client SDK
 * Interact with the membra_tokenomics Anchor program.
 */
import { Connection, PublicKey, Keypair } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";

// ============================================================================
// Constants
// ============================================================================

export const TOKENOMICS_PROGRAM_ID = new PublicKey(
  "38tgbireEP2AFq5YQBroNN9wQTqAECUuDNYmRjkogKca"
);

export const SPLITS = {
  treasuryBps: 8000,
  protocolBps: 1000,
  validatorBps: 500,
  earlyRewardBps: 500,
} as const;

// ============================================================================
// Types
// ============================================================================

export interface TokenSaleData {
  authority: PublicKey;
  saleId: number;
  status: number;
  basePriceLamports: number;
  slopeBps: number;
  maxBonusBps: number;
  saleDurationSec: number;
  startTime: number;
  endTime: number;
  totalRaisedLamports: number;
  totalTokensAllocated: number;
  contributionCount: number;
  treasury: PublicKey;
  protocolWallet: PublicKey;
  validatorPool: PublicKey;
  earlyRewardCapLamports: number;
  earlyRewardDistributedLamports: number;
  maxRebatePerBuyerLamports: number;
  rebateRateBps: number;
  hardCapLamports: number;
  minContributionLamports: number;
}

export interface ContributionData {
  sale: PublicKey;
  buyer: PublicKey;
  amountLamports: number;
  baseTokens: number;
  bonusTokens: number;
  totalTokens: number;
  bonusBps: number;
  priceAtContribution: number;
  contributionIndex: number;
  createdAt: number;
}

export interface BuyerReceiptData {
  sale: PublicKey;
  buyer: PublicKey;
  totalContributedLamports: number;
  totalTokensAllocated: number;
  rebateClaimedLamports: number;
  rebateClaimStatus: number;
  lastUpdatedAt: number;
}

export enum SaleStatus {
  Draft = 0,
  Active = 1,
  Paused = 2,
  Finalized = 3,
  Cancelled = 4,
  LiquidityMigrated = 5,
}

export enum RebateClaimStatus {
  Pending = 0,
  Eligible = 1,
  Claimed = 2,
  Expired = 3,
  Denied = 4,
}

export interface ContributionQuote {
  baseTokens: number;
  bonusTokens: number;
  totalTokens: number;
  bonusBps: number;
  priceAtContribution: number;
}

export interface SplitQuote {
  treasury: number;
  protocol: number;
  validator: number;
  earlyReward: number;
}

export interface InitializeSaleParams {
  basePriceLamports: number;
  slopeBps: number;
  maxBonusBps: number;
  saleDurationSec: number;
  earlyRewardCapLamports: number;
  maxRebatePerBuyerLamports: number;
  rebateRateBps: number;
  hardCapLamports: number;
  minContributionLamports: number;
  treasury: PublicKey;
  protocolWallet: PublicKey;
  validatorPool: PublicKey;
}

export interface ContributeParams {
  buyer: Keypair;
  saleId: number;
  amountLamports: number;
  contributionIndex?: number;
}

// ============================================================================
// Client
// ============================================================================

export class MembraTokenomicsClient {
  program: anchor.Program<any>;
  provider: anchor.AnchorProvider;

  constructor(
    connection: Connection,
    wallet: anchor.Wallet,
    programId?: PublicKey
  ) {
    this.provider = new anchor.AnchorProvider(connection, wallet, {
      commitment: "confirmed",
    });
    anchor.setProvider(this.provider);

    // IDL would be loaded from target/types after anchor build
    const idl = {
      version: "0.1.0",
      name: "membra_tokenomics",
      instructions: [],
      accounts: [],
      types: [],
      errors: [],
    };
    this.program = new anchor.Program(
      idl as any,
      programId || TOKENOMICS_PROGRAM_ID,
      this.provider
    );
  }

  // ==========================================================================
  // PDA Helpers
  // ==========================================================================

  tokenSalePda(saleId: number): [PublicKey, number] {
    const idBytes = Buffer.alloc(8);
    idBytes.writeBigUInt64LE(BigInt(saleId), 0);
    return PublicKey.findProgramAddressSync(
      [Buffer.from("token_sale"), idBytes],
      this.program.programId
    );
  }

  earlyRewardPoolPda(sale: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("early_reward_pool"), sale.toBuffer()],
      this.program.programId
    );
  }

  contributionPda(
    sale: PublicKey,
    buyer: PublicKey,
    index: number
  ): [PublicKey, number] {
    const idxBytes = Buffer.alloc(8);
    idxBytes.writeBigUInt64LE(BigInt(index), 0);
    return PublicKey.findProgramAddressSync(
      [Buffer.from("contribution"), sale.toBuffer(), buyer.toBuffer(), idxBytes],
      this.program.programId
    );
  }

  buyerReceiptPda(sale: PublicKey, buyer: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("buyer_receipt"), sale.toBuffer(), buyer.toBuffer()],
      this.program.programId
    );
  }

  // ==========================================================================
  // Math Helpers
  // ==========================================================================

  /**
   * Linear bonding curve: price = base + slope * total_raised / 10_000
   */
  calculatePrice(
    basePrice: number,
    slopeBps: number,
    totalRaised: number
  ): number {
    const slopeComponent = Math.floor((totalRaised * slopeBps) / 10000);
    return basePrice + slopeComponent;
  }

  /**
   * Time-decay bonus: max_bonus * (1 - elapsed / duration)
   * Returns basis points.
   */
  calculateTimeDecayBonus(
    maxBonusBps: number,
    elapsedSec: number,
    durationSec: number
  ): number {
    if (elapsedSec >= durationSec) return 0;
    const remainingRatio = Math.floor(
      ((durationSec - elapsedSec) * 10000) / durationSec
    );
    return Math.floor((maxBonusBps * remainingRatio) / 10000);
  }

  /**
   * Compute token allocation for a contribution.
   */
  computeAllocation(
    amountLamports: number,
    basePrice: number,
    slopeBps: number,
    totalRaised: number,
    maxBonusBps: number,
    elapsedSec: number,
    durationSec: number
  ): { baseTokens: number; bonusTokens: number; bonusBps: number } {
    const price = this.calculatePrice(basePrice, slopeBps, totalRaised);
    const baseTokens = Math.floor((amountLamports * 1_000_000) / price);
    const bonusBps = this.calculateTimeDecayBonus(
      maxBonusBps,
      elapsedSec,
      durationSec
    );
    const bonusTokens = Math.floor((baseTokens * bonusBps) / 10000);
    return { baseTokens, bonusTokens, bonusBps };
  }

  /**
   * Compute contribution split amounts.
   */
  computeSplit(amountLamports: number): {
    treasury: number;
    protocol: number;
    validator: number;
    earlyReward: number;
  } {
    const treasury = Math.floor((amountLamports * SPLITS.treasuryBps) / 10000);
    const protocol = Math.floor((amountLamports * SPLITS.protocolBps) / 10000);
    const validator = Math.floor(
      (amountLamports * SPLITS.validatorBps) / 10000
    );
    const earlyReward =
      amountLamports - treasury - protocol - validator;
    return { treasury, protocol, validator, earlyReward };
  }

  // ==========================================================================
  // Instruction Builders (return transaction methods for signing)
  // ==========================================================================

  async initializeSale(
    authority: Keypair,
    saleId: number,
    params: {
      basePriceLamports: number;
      slopeBps: number;
      maxBonusBps: number;
      saleDurationSec: number;
      earlyRewardCapLamports: number;
      maxRebatePerBuyerLamports: number;
      rebateRateBps: number;
      hardCapLamports: number;
      minContributionLamports: number;
      treasury: PublicKey;
      protocolWallet: PublicKey;
      validatorPool: PublicKey;
    }
  ) {
    const [salePda] = this.tokenSalePda(saleId);
    const [poolPda] = this.earlyRewardPoolPda(salePda);
    const idBytes = Buffer.alloc(8);
    idBytes.writeBigUInt64LE(BigInt(saleId), 0);
    return this.program.methods
      .initializeSale(
        new anchor.BN(saleId),
        Array.from(idBytes) as any,
        new anchor.BN(params.basePriceLamports),
        new anchor.BN(params.slopeBps),
        params.maxBonusBps,
        new anchor.BN(params.saleDurationSec),
        new anchor.BN(params.earlyRewardCapLamports),
        new anchor.BN(params.maxRebatePerBuyerLamports),
        params.rebateRateBps,
        new anchor.BN(params.hardCapLamports),
        new anchor.BN(params.minContributionLamports)
      )
      .accounts({
        authority: authority.publicKey,
        tokenSale: salePda,
        treasury: params.treasury,
        protocolWallet: params.protocolWallet,
        validatorPool: params.validatorPool,
        earlyRewardPool: poolPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([authority])
      .rpc();
  }

  async activateSale(authority: Keypair, saleId: number) {
    const [salePda] = this.tokenSalePda(saleId);
    return this.program.methods
      .activateSale()
      .accounts({
        authority: authority.publicKey,
        tokenSale: salePda,
      } as any)
      .signers([authority])
      .rpc();
  }

  async contribute(
    buyer: Keypair,
    saleId: number,
    amountLamports: number,
    contributionIndex?: number
  ) {
    const [salePda] = this.tokenSalePda(saleId);
    const [poolPda] = this.earlyRewardPoolPda(salePda);
    const [receiptPda] = this.buyerReceiptPda(salePda, buyer.publicKey);
    const saleAccount = await this.program.account.tokenSale.fetch(salePda);
    const nextIndex = contributionIndex ?? ((saleAccount?.contributionCount || 0) + 1);
    const idxBytes = Buffer.alloc(8);
    idxBytes.writeBigUInt64LE(BigInt(nextIndex), 0);
    const [contribPda] = this.contributionPda(salePda, buyer.publicKey, nextIndex);
    return this.program.methods
      .contribute(
        new anchor.BN(amountLamports),
        new anchor.BN(nextIndex),
        Array.from(idxBytes) as any
      )
      .accounts({
        buyer: buyer.publicKey,
        tokenSale: salePda,
        treasury: (saleAccount as any)?.treasury,
        protocolWallet: (saleAccount as any)?.protocolWallet,
        validatorPool: (saleAccount as any)?.validatorPool,
        earlyRewardPool: poolPda,
        contribution: contribPda,
        buyerReceipt: receiptPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([buyer])
      .rpc();
  }

  async claimRebate(buyer: Keypair, saleId: number) {
    const [salePda] = this.tokenSalePda(saleId);
    const [poolPda] = this.earlyRewardPoolPda(salePda);
    const [receiptPda] = this.buyerReceiptPda(salePda, buyer.publicKey);
    return this.program.methods
      .claimRebate()
      .accounts({
        buyer: buyer.publicKey,
        tokenSale: salePda,
        earlyRewardPool: poolPda,
        buyerReceipt: receiptPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([buyer])
      .rpc();
  }

  async finalizeSale(authority: Keypair, saleId: number) {
    const [salePda] = this.tokenSalePda(saleId);
    return this.program.methods
      .finalizeSale()
      .accounts({
        authority: authority.publicKey,
        tokenSale: salePda,
      } as any)
      .signers([authority])
      .rpc();
  }

  async migrateLiquidity(authority: Keypair, saleId: number) {
    const [salePda] = this.tokenSalePda(saleId);
    return this.program.methods
      .migrateLiquidity()
      .accounts({
        authority: authority.publicKey,
        tokenSale: salePda,
      } as any)
      .signers([authority])
      .rpc();
  }

  async cancelSale(authority: Keypair, saleId: number) {
    const [salePda] = this.tokenSalePda(saleId);
    return this.program.methods
      .cancelSale()
      .accounts({
        authority: authority.publicKey,
        tokenSale: salePda,
      } as any)
      .signers([authority])
      .rpc();
  }

  async pauseSale(authority: Keypair, saleId: number) {
    const [salePda] = this.tokenSalePda(saleId);
    return this.program.methods
      .setSalePause(true)
      .accounts({
        authority: authority.publicKey,
        tokenSale: salePda,
      } as any)
      .signers([authority])
      .rpc();
  }

  async resumeSale(authority: Keypair, saleId: number) {
    const [salePda] = this.tokenSalePda(saleId);
    return this.program.methods
      .setSalePause(false)
      .accounts({
        authority: authority.publicKey,
        tokenSale: salePda,
      } as any)
      .signers([authority])
      .rpc();
  }

  // ==========================================================================
  // Fetch Helpers
  // ==========================================================================

  async fetchTokenSale(saleId: number): Promise<TokenSaleData | null> {
    const [salePda] = this.tokenSalePda(saleId);
    try {
      const raw = await this.program.account.tokenSale.fetch(salePda);
      return this.mapTokenSale(raw);
    } catch {
      return null;
    }
  }

  async fetchBuyerReceipt(
    saleId: number,
    buyer: PublicKey
  ): Promise<BuyerReceiptData | null> {
    const [salePda] = this.tokenSalePda(saleId);
    const [receiptPda] = this.buyerReceiptPda(salePda, buyer);
    try {
      const raw = await this.program.account.buyerReceipt.fetch(receiptPda);
      return this.mapBuyerReceipt(raw);
    } catch {
      return null;
    }
  }

  async fetchContribution(
    saleId: number,
    buyer: PublicKey,
    index: number
  ): Promise<ContributionData | null> {
    const [salePda] = this.tokenSalePda(saleId);
    const [contribPda] = this.contributionPda(salePda, buyer, index);
    try {
      const raw = await this.program.account.contribution.fetch(contribPda);
      return this.mapContribution(raw);
    } catch {
      return null;
    }
  }

  private mapTokenSale(raw: any): TokenSaleData {
    return {
      authority: raw.authority,
      saleId: raw.saleId.toNumber(),
      status: raw.status,
      basePriceLamports: raw.basePriceLamports.toNumber(),
      slopeBps: raw.slopeBps.toNumber(),
      maxBonusBps: raw.maxBonusBps,
      saleDurationSec: raw.saleDurationSec.toNumber(),
      startTime: raw.startTime.toNumber(),
      endTime: raw.endTime.toNumber(),
      totalRaisedLamports: raw.totalRaisedLamports.toNumber(),
      totalTokensAllocated: raw.totalTokensAllocated.toNumber(),
      contributionCount: raw.contributionCount.toNumber(),
      treasury: raw.treasury,
      protocolWallet: raw.protocolWallet,
      validatorPool: raw.validatorPool,
      earlyRewardCapLamports: raw.earlyRewardCapLamports.toNumber(),
      earlyRewardDistributedLamports:
        raw.earlyRewardDistributedLamports.toNumber(),
      maxRebatePerBuyerLamports: raw.maxRebatePerBuyerLamports.toNumber(),
      rebateRateBps: raw.rebateRateBps,
      hardCapLamports: raw.hardCapLamports.toNumber(),
      minContributionLamports: raw.minContributionLamports.toNumber(),
    };
  }

  private mapContribution(raw: any): ContributionData {
    return {
      sale: raw.sale,
      buyer: raw.buyer,
      amountLamports: raw.amountLamports.toNumber(),
      baseTokens: raw.baseTokens.toNumber(),
      bonusTokens: raw.bonusTokens.toNumber(),
      totalTokens: raw.totalTokens.toNumber(),
      bonusBps: raw.bonusBps,
      priceAtContribution: raw.priceAtContribution.toNumber(),
      contributionIndex: raw.contributionIndex.toNumber(),
      createdAt: raw.createdAt.toNumber(),
    };
  }

  private mapBuyerReceipt(raw: any): BuyerReceiptData {
    return {
      sale: raw.sale,
      buyer: raw.buyer,
      totalContributedLamports: raw.totalContributedLamports.toNumber(),
      totalTokensAllocated: raw.totalTokensAllocated.toNumber(),
      rebateClaimedLamports: raw.rebateClaimedLamports.toNumber(),
      rebateClaimStatus: raw.rebateClaimStatus,
      lastUpdatedAt: raw.lastUpdatedAt.toNumber(),
    };
  }

  /**
   * Generate a Solana explorer URL for a transaction or account.
   */
  getExplorerUrl(
    signatureOrAddress: string,
    cluster: "mainnet-beta" | "devnet" | "localnet" = "devnet"
  ): string {
    const base =
      cluster === "mainnet-beta"
        ? "https://explorer.solana.com"
        : `https://explorer.solana.com/?cluster=${cluster}`;
    return `${base}/tx/${signatureOrAddress}`;
  }
}
