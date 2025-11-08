import * as anchor from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram, Transaction, sendAndConfirmRawTransaction } from "@solana/web3.js"
import { Program } from "@coral-xyz/anchor";
import { DegenDecks } from "../target/types/degen_decks";
import { assert, expect } from "chai";
import { BN } from "bn.js";
import { Account, ASSOCIATED_TOKEN_PROGRAM_ID, createSyncNativeInstruction, getAccount, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
    delegateBufferPdaFromDelegatedAccountAndOwnerProgram,
    DELEGATION_PROGRAM_ID,
    delegationMetadataPdaFromDelegatedAccount,
    delegationRecordPdaFromDelegatedAccount,
} from "@magicblock-labs/ephemeral-rollups-sdk";

describe("Degen Decks", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.DegenDecks as Program<DegenDecks>;
    const connection = provider.connection;

    // helper funtions
    const findPDA = (seeds: Array<any>, programId = program.programId) => {
        return anchor.web3.PublicKey.findProgramAddressSync(
            seeds,
            programId
        );
    }

    // PDA Seeds Contants
    const CONFIG_SEED = "CONFIG";
    const PROFILE_SEED = "PROFILE";
    const GAME_SEED = "GAME";


    // Game seeds
    const seed1 = new BN(Date.now());

    // Mint Accounts
    const WSOL = new PublicKey("So11111111111111111111111111111111111111112"); //wrapped sol
    const USDC = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v") // USDC

    // Accounts
    const user1 = provider.wallet;
    const user2 = Keypair.generate();
    const randomUser = Keypair.generate();


    // PDAs
    let config: PublicKey;
    let programData: PublicKey;
    let userProfile1: PublicKey;
    let userProfile2: PublicKey;
    let userAta1: Account;
    let userAta2: Account;

    const game = findPDA([
        Buffer.from(GAME_SEED, "utf-8"),
        new BN(seed1).toArrayLike(Buffer, "le", 8),
        user1.publicKey.toBytes()
    ])[0];
    const gameVault = getAssociatedTokenAddressSync(
        WSOL,
        game,
        true
    );
    const bufferGame = delegateBufferPdaFromDelegatedAccountAndOwnerProgram(
        game,
        program.programId
    );
    const metadataGame = delegationMetadataPdaFromDelegatedAccount(
        game
    );
    const recordGame = delegationRecordPdaFromDelegatedAccount(
        game
    );


    before(async () => {
        // Airdrop SOL
        await connection.requestAirdrop(randomUser.publicKey, 5_000_000_000);
        await connection.requestAirdrop(user1.publicKey, 5_000_000_000);
        await connection.requestAirdrop(user2.publicKey, 5_000_000_000);
        await new Promise((resolve) => setTimeout(resolve, 500)); // Wait for airdrops

        // Derive PDAs
        config = findPDA([Buffer.from(CONFIG_SEED, "utf-8")])[0];
        userAta1 = await getOrCreateAssociatedTokenAccount(
            connection,
            user1.payer,
            WSOL,
            user1.publicKey
        );
        userAta2 = await getOrCreateAssociatedTokenAccount(
            connection,
            user2,
            WSOL,
            user2.publicKey
        );

        userProfile1 = findPDA([Buffer.from(PROFILE_SEED, "utf-8"), user1.publicKey.toBytes()])[0];
        userProfile2 = findPDA([Buffer.from(PROFILE_SEED, "utf-8"), user2.publicKey.toBytes()])[0];


        const BPF_LOADER_UPGRADEABLE_PROGRAM_ID = new PublicKey("BPFLoaderUpgradeab1e11111111111111111111111");
        programData = findPDA([program.programId.toBuffer()], BPF_LOADER_UPGRADEABLE_PROGRAM_ID)[0]
        // Verify ProgramData exists after deployment
        const programDataAccount = await connection.getAccountInfo(programData);
        assert.ok(programDataAccount, "ProgramData should exist after deployment");

    });

    describe("> Initialize Config", () => {
        const platformFee = 500; // 5%
        const allow_mints = [
            WSOL,
            USDC
        ];
        it("Should initialize the config", async () => {
            try {
                const tx = await program.methods
                    .initialize(platformFee, allow_mints)
                    .accountsStrict({
                        admin: user1.publicKey,
                        config: config,
                        programData: programData,
                        systemProgram: SystemProgram.programId,
                    })
                    .signers([user1.payer])
                    .rpc();
                console.log("Initialize transaction: ", tx);
            } catch (error: any) {
                console.error(`Oops, something went wrong: ${error}`);
            }

            const configAccount = await program.account.config.fetch(config);
            expect(configAccount.platformFee).to.equal(platformFee, "Platform fees do not match");
            expect(configAccount.allowedMints).to.deep.equal(allow_mints, "Allowed mints do not match");
        });

        it("Only admin should initialize", async () => {
            const platformFee = 1000;
            try {
                const tx = await program.methods
                    .initialize(platformFee, allow_mints)
                    .accountsStrict({
                        admin: randomUser.publicKey,
                        config: config,
                        programData: programData,
                        systemProgram: SystemProgram.programId,
                    })
                    .signers([randomUser])
                    .rpc();
                console.log("Initialize transaction", tx);
                expect.fail("Expect instruction to throw");
            } catch (error: any) {
                expect(error.message).to.match(/You Are Not Unauthorized/i);
            }
        });
    });

    describe("> Initialize Profile", () => {
        const username1 = "Godwyn";
        const username2 = "Rustoshi";

        it("Should initialize user profiles", async () => {
            try {
                const tx1 = await program.methods
                    .initializeProfile(username1)
                    .accountsStrict({
                        signer: user1.publicKey,
                        profile: userProfile1,
                        systemProgram: SystemProgram.programId,
                    })
                    .signers([user1.payer])
                    .rpc();

                const tx2 = await program.methods
                    .initializeProfile(username2)
                    .accountsStrict({
                        signer: user2.publicKey,
                        profile: userProfile2,
                        systemProgram: SystemProgram.programId,
                    })
                    .signers([user2])
                    .rpc();
                console.log("Initialize transaction1: ", tx1);
                console.log("Initialize transaction2: ", tx2);
            } catch (error: any) {
                console.error(`Oops, something went wrong: ${error}`);
            }

            const profileAccount1 = await program.account.profile.fetch(userProfile1);
            const profileAccount2 = await program.account.profile.fetch(userProfile2);

            expect(profileAccount1.username).to.equal(username1, "Username does not match");
            expect(profileAccount2.username).to.equal(username2, "Username does not match");
        });
    });

    describe("> Initialize Game", () => {
        it("Should initialize Game Room with WSOL", async () => {
            const entryStake = 5_000_000_000 // 5 wSol
            const noPlayers = 2;
            const waitTime = new BN(60 * 1000);



            const ataInfo = await getAccount(connection, userAta1.address);
            if (ataInfo.amount < entryStake) {
                const tx = new Transaction();
                tx.add(
                    SystemProgram.transfer({
                        fromPubkey: user1.publicKey,
                        toPubkey: userAta1.address,
                        lamports: entryStake
                    })
                );
                tx.add(
                    createSyncNativeInstruction(
                        userAta1.address,
                        TOKEN_PROGRAM_ID
                    )
                );
                try {
                    const { blockhash } = await connection.getLatestBlockhash();
                    tx.recentBlockhash = blockhash;
                    tx.feePayer = user1.publicKey;
                    tx.sign(user1.payer);
                    const rawTx = tx.serialize();

                    await sendAndConfirmRawTransaction(
                        connection,
                        rawTx
                    );
                    console.log("Funded ATA")
                } catch (error: any) {
                    console.log("Error funding ATA", error);
                }
            }

            try {
                const tx = await program.methods
                    .initializeGame(
                        seed1,
                        new BN(entryStake),
                        noPlayers,
                        waitTime
                    )
                    .accountsStrict({
                        signer: user1.publicKey,
                        profile: userProfile1,
                        game: game,
                        gameVault: gameVault,
                        stakeMint: WSOL,
                        userAta: userAta1.address,
                        config: config,
                        tokenProgram: TOKEN_PROGRAM_ID,
                        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                        systemProgram: SystemProgram.programId
                    })
                    .signers([user1.payer])
                    .rpc();
                console.log("Initialize transaction: ", tx);
            } catch (error: any) {
                console.error(`Oops, something went wrong: ${error}`);
            }

            const gameAccount = await program.account.game.fetch(game);
            expect(gameAccount.owner.toBase58()).to.equal(user1.publicKey.toBase58(), "Owner does not match");
            expect(gameAccount.entryStake.toNumber()).to.equal(entryStake, "Entry stake does not match");
            expect(gameAccount.gameVault.toBase58()).to.equal(gameVault.toBase58(), "Game vault does not match");
            expect(gameAccount.stakeMint.toBase58()).to.equal(WSOL.toBase58(), "Stake mint does not match");
            expect(gameAccount.noPlayers).to.equal(noPlayers, "No players does not match");
            expect(gameAccount.playerTurn).to.equal(0, "Player turn does not match");
            expect(gameAccount.callCard).to.equal(null, "Call card does not match");
            expect(gameAccount.waitTime.toNumber()).to.equal(waitTime.toNumber(), "Wait time does not match");
            expect(gameAccount.seed.toNumber()).to.equal(seed1.toNumber(), "Seed does not match");
            expect(gameAccount.randomSeed).to.equal(null, "Random seed does not match");
            expect(gameAccount.delegated).to.equal(false, "Delegated does not match");
            expect(gameAccount.started).to.equal(false, "Started does not match");
            expect(gameAccount.ended).to.equal(false, "Ended does not match");
            expect(gameAccount.createdAt).to.equal(gameAccount.createdAt, "Created at does not match");
            expect(gameAccount.startedAt).to.equal(null, "Started at does not match");
            expect(gameAccount.endedAt).to.equal(null, "Ended at does not match");
            expect(gameAccount.bump).to.equal(255, "Bump does not match");
        });
    });

    describe("> Join Game", () => {

        it("User 2 Should Join game", async () => {
            try {
                const tx = await program.methods
                    .joinGame()
                    .accountsStrict({
                        signer: user1.publicKey,
                        profile: userProfile1,
                        game: game,
                        gameVault: gameVault,
                        stakeMint: WSOL,
                        userAta: userAta1.address,
                        config: config,
                        tokenProgram: TOKEN_PROGRAM_ID,
                        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                        systemProgram: SystemProgram.programId,
                        delegationProgram: DELEGATION_PROGRAM_ID,
                        ownerProgram: program.programId,
                        bufferGame: bufferGame,
                        delegationMetadataGame: metadataGame,
                        delegationRecordGame: recordGame
                    })
                    .signers([user2])
                    .rpc();
                console.log("Initialize transaction: ", tx);
            } catch (error: any) {
                console.error(`Oops, something went wrong: ${error}`);
            }

            const gameAccount = await program.account.game.fetch(game);
            console.info(gameAccount);
        });
    });

}); 