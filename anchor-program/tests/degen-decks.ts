import * as anchor from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram, Transaction, sendAndConfirmRawTransaction, LAMPORTS_PER_SOL } from "@solana/web3.js"
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

    // Helper functions
    const findPDA = (seeds: Array<any>, programId = program.programId) => {
        return anchor.web3.PublicKey.findProgramAddressSync(
            seeds,
            programId
        );
    }

    const sendSOL = async (from: PublicKey, to: PublicKey, lamports: number, signer: Keypair) => {
        const tx = new Transaction();
        tx.add(
            SystemProgram.transfer({
                fromPubkey: from,
                toPubkey: to,
                lamports: lamports
            })
        );
        const { blockhash } = await connection.getLatestBlockhash();
        tx.recentBlockhash = blockhash;
        tx.feePayer = signer.publicKey;
        tx.sign(signer);
        const rawTx = tx.serialize();

        await sendAndConfirmRawTransaction(
            connection,
            rawTx
        );
        await new Promise((resolve) => setTimeout(resolve, 500));
        console.log(`Sent ${lamports} SOL from ${from} to ${to}`)
    }

    const fundWSOLATA = async (payerKp: Keypair, ata: Account, lamports: number) => {
        const tx = new Transaction();
        tx.add(
            SystemProgram.transfer({
                fromPubkey: payerKp.publicKey,
                toPubkey: ata.address,
                lamports: lamports
            })
        );
        tx.add(
            createSyncNativeInstruction(
                ata.address,
                TOKEN_PROGRAM_ID
            )
        );
        const { blockhash } = await connection.getLatestBlockhash();
        tx.recentBlockhash = blockhash;
        tx.feePayer = payerKp.publicKey;
        tx.sign(payerKp);
        const rawTx = tx.serialize();

        await sendAndConfirmRawTransaction(
            connection,
            rawTx
        );
        await new Promise((resolve) => setTimeout(resolve, 500));
        console.log(`Funded ${ata.address} with ${lamports} SOL`)
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
    const feeWallet = user1.publicKey;
    const user2 = Keypair.generate();
    const user3 = Keypair.generate();
    const randomUser = Keypair.generate();



    // PDAs
    let config: PublicKey;
    let programData: PublicKey;

    let userProfile1: PublicKey;
    let userProfile2: PublicKey;
    let userProfile3: PublicKey;

    let userAta1: Account;
    let userAta2: Account;
    let userAta3: Account;

    // Game variables
    const entryStake = 0.1 * LAMPORTS_PER_SOL;
    const noPlayers = 3;
    const waitTime = new BN(60);

    const game = findPDA([
        Buffer.from(GAME_SEED, "utf-8"),
        new BN(seed1).toArrayLike(Buffer, "le", 8),
        user1.publicKey.toBytes()
    ])[0];
    const feeWsolAta = getAssociatedTokenAddressSync(
        WSOL,
        feeWallet,
        true
    );
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
        // await connection.requestAirdrop(randomUser.publicKey, 10 * LAMPORTS_PER_SOL);
        // await connection.requestAirdrop(user1.publicKey, 10 * LAMPORTS_PER_SOL);
        // await connection.requestAirdrop(user2.publicKey, 5 * LAMPORTS_PER_SOL);
        // await new Promise((resolve) => setTimeout(resolve, 1000)); // Wait for airdrops

        // funder players WSOL accounts with 0.5 SOL
        await sendSOL(user1.publicKey, user2.publicKey, 0.3 * LAMPORTS_PER_SOL, user1.payer);
        await sendSOL(user1.publicKey, user3.publicKey, 0.3 * LAMPORTS_PER_SOL, user1.payer);

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
        userAta3 = await getOrCreateAssociatedTokenAccount(
            connection,
            user3,
            WSOL,
            user3.publicKey
        );

        // fund players WSOL accounts with 0.2 SOL
        await fundWSOLATA(user1.payer, userAta1, 0.2 * LAMPORTS_PER_SOL);
        await fundWSOLATA(user2, userAta2, 0.2 * LAMPORTS_PER_SOL);
        await fundWSOLATA(user3, userAta3, 0.2 * LAMPORTS_PER_SOL);

        userProfile1 = findPDA([Buffer.from(PROFILE_SEED, "utf-8"), user1.publicKey.toBytes()])[0];
        userProfile2 = findPDA([Buffer.from(PROFILE_SEED, "utf-8"), user2.publicKey.toBytes()])[0];
        userProfile3 = findPDA([Buffer.from(PROFILE_SEED, "utf-8"), user3.publicKey.toBytes()])[0];


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
            const tx = await program.methods
                .initialize(platformFee, allow_mints)
                .accountsStrict({
                    admin: user1.publicKey,
                    config: config,
                    feeWallet: feeWallet,
                    programData: programData,
                    systemProgram: SystemProgram.programId,
                })
                .signers([user1.payer])
                .rpc();
            console.log("Initialize Config Transaction: ", tx);

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
                        feeWallet: feeWallet,
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
        const username1 = "Godwin";
        const username2 = "Rustoshidev";
        const username3 = "Toly";

        it("Should initialize user profiles", async () => {
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

            const tx3 = await program.methods
                .initializeProfile(username3)
                .accountsStrict({
                    signer: user3.publicKey,
                    profile: userProfile3,
                    systemProgram: SystemProgram.programId,
                })
                .signers([user3])
                .rpc();

            console.log("Initialize Profile Transaction1: ", tx1);
            console.log("Initialize Profile Transaction2: ", tx2);
            console.log("Initialize Profile Transaction3: ", tx3);

            const profileAccount1 = await program.account.profile.fetch(userProfile1);
            const profileAccount2 = await program.account.profile.fetch(userProfile2);
            const profileAccount3 = await program.account.profile.fetch(userProfile3);

            expect(profileAccount1.username).to.equal(username1, "Username does not match");
            expect(profileAccount2.username).to.equal(username2, "Username does not match");
            expect(profileAccount3.username).to.equal(username3, "Username does not match");
        });
    });

    describe("> Initialize Game", () => {
        it("Should initialize Game Room with WSOL", async () => {

            let ataInfo = await getAccount(connection, userAta1.address);
            const ataBalance = Number(ataInfo.amount);

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
            console.log("Initialize Game transaction: ", tx);


            const gameAccount = await program.account.game.fetch(game);
            const gameVaultInfo = await getAccount(connection, gameVault);
            ataInfo = await getAccount(connection, userAta1.address);
            const ataBalanceAfter = Number(ataInfo.amount);

            console.log(`Entry stake: ${entryStake}`);
            console.log(`ATA balance after initialization: ${ataBalanceAfter}`);
            console.log(`Game vault amount: ${gameVaultInfo.amount}`);

            expect(Number(ataBalanceAfter)).to.equal(ataBalance - entryStake, "ATA balance after initialization does not match");
            expect(Number(gameVaultInfo.amount)).to.equal(entryStake, "Game vault amount does not match");
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
        });
    });

    describe("> User 2 Joins Game", () => {
        it("User 2 Should Join game", async () => {
            let gameAccount = await program.account.game.fetch(game);
            let ataInfo = await getAccount(connection, userAta2.address);
            const ataBalance = Number(ataInfo.amount);

            const tx = await program.methods
                .joinGame()
                .accountsStrict({
                    signer: user2.publicKey,
                    profile: userProfile2,
                    game: game,
                    gameVault: gameVault,
                    stakeMint: WSOL,
                    userAta: userAta2.address,
                    config: config,
                    oracleQueue: new PublicKey("Cuj97ggrhhidhbu39TijNVqE74xvKJ69gDervRUXAxGh"),
                    programIdentity: findPDA([Buffer.from("identity", "utf-8")])[0],
                    vrfProgram: new PublicKey("Vrf1RNUjXmQGjmQrQLvJHs9SNkvDJEsRVFPkfSQUwGz"),
                    slotHashes: new PublicKey("SysvarS1otHashes111111111111111111111111111"),
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .signers([user2])
                .rpc();
            console.log("Join transaction: ", tx);

            gameAccount = await program.account.game.fetch(game);
            ataInfo = await getAccount(connection, userAta2.address);
            const gameVaultInfo = await getAccount(connection, gameVault);

            console.log(`ATA balance after joining: ${ataInfo.amount}`);
            console.log(`Game vault amount: ${gameVaultInfo.amount}`);
            expect(Number(ataInfo.amount)).to.equal(ataBalance - gameAccount.entryStake.toNumber(), "Balance after joining does not match");
            expect(Number(gameVaultInfo.amount)).to.equal(gameAccount.entryStake.toNumber() * 2, "Game vault amount does not match");
            expect(gameAccount.players.length).to.equal(2, "Players length does not match");
            // console.info(gameAccount);
            // console.log(gameAccount.players[0]);
            // console.log(gameAccount.players[1]);
        });
    });

    describe("> User 2 Exits Game", () => {
        it("User 2 Should Exit game", async () => {
            let gameAccount = await program.account.game.fetch(game);
            const ataInfo = await getAccount(connection, userAta2.address);

            const tx = await program.methods
                .exitGame()
                .accountsStrict({
                    signer: user2.publicKey,
                    profile: userProfile2,
                    game: game,
                    gameVault: gameVault,
                    stakeMint: WSOL,
                    userAta: userAta2.address,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .signers([user2])
                .rpc();
            console.log("Exit transaction: ", tx);

            gameAccount = await program.account.game.fetch(game);
            const gameVaultInfo = await getAccount(connection, gameVault);

            expect(Number(gameVaultInfo.amount)).to.equal(gameAccount.entryStake.toNumber(), "Game vault amount does not match");
            expect(gameAccount.players.length).to.equal(1, "Players length does not match");
            // console.info(gameAccount);
            // console.log(gameAccount.players[0]);
            // console.log(gameAccount.players[1]);
        });
    });

    describe("> User 2 and 3 joins Game", () => {
        it("User 2 Should Join game", async () => {
            let gameAccount = await program.account.game.fetch(game);
            let ataInfo = await getAccount(connection, userAta2.address);
            const ataBalance = Number(ataInfo.amount);

            const tx = await program.methods
                .joinGame()
                .accountsStrict({
                    signer: user2.publicKey,
                    profile: userProfile2,
                    game: game,
                    gameVault: gameVault,
                    stakeMint: WSOL,
                    userAta: userAta2.address,
                    config: config,
                    oracleQueue: new PublicKey("Cuj97ggrhhidhbu39TijNVqE74xvKJ69gDervRUXAxGh"),
                    programIdentity: findPDA([Buffer.from("identity", "utf-8")])[0],
                    vrfProgram: new PublicKey("Vrf1RNUjXmQGjmQrQLvJHs9SNkvDJEsRVFPkfSQUwGz"),
                    slotHashes: new PublicKey("SysvarS1otHashes111111111111111111111111111"),
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .signers([user2])
                .rpc();
            console.log("Join transaction: ", tx);

            gameAccount = await program.account.game.fetch(game);
            ataInfo = await getAccount(connection, userAta2.address);
            const gameVaultInfo = await getAccount(connection, gameVault);

            console.log(`ATA balance after joining: ${ataInfo.amount}`);
            console.log(`Game vault amount: ${gameVaultInfo.amount}`);
            expect(Number(ataInfo.amount)).to.equal(ataBalance - gameAccount.entryStake.toNumber(), "Balance after joining does not match");
            expect(Number(gameVaultInfo.amount)).to.equal(gameAccount.entryStake.toNumber() * 2, "Game vault amount does not match");
            expect(gameAccount.players.length).to.equal(2, "Players length does not match");
            // console.info(gameAccount);
            // console.log(gameAccount.players[0]);
            // console.log(gameAccount.players[1]);
        });


        it("User 3 Should Join game", async () => {
            let gameAccount = await program.account.game.fetch(game);
            let ataInfo = await getAccount(connection, userAta3.address);
            const ataBalance = Number(ataInfo.amount);

            const tx = await program.methods
                .joinGame()
                .accountsStrict({
                    signer: user3.publicKey,
                    profile: userProfile3,
                    game: game,
                    gameVault: gameVault,
                    stakeMint: WSOL,
                    userAta: userAta3.address,
                    config: config,
                    oracleQueue: new PublicKey("Cuj97ggrhhidhbu39TijNVqE74xvKJ69gDervRUXAxGh"),
                    programIdentity: findPDA([Buffer.from("identity", "utf-8")])[0],
                    vrfProgram: new PublicKey("Vrf1RNUjXmQGjmQrQLvJHs9SNkvDJEsRVFPkfSQUwGz"),
                    slotHashes: new PublicKey("SysvarS1otHashes111111111111111111111111111"),
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .signers([user3])
                .rpc();
            console.log("Join transaction: ", tx);

            // wait for VRF
            await new Promise((resolve) => setTimeout(resolve, 3000));

            gameAccount = await program.account.game.fetch(game);
            ataInfo = await getAccount(connection, userAta3.address);
            const gameVaultInfo = await getAccount(connection, gameVault);

            console.log(`ATA balance after joining: ${ataInfo.amount}`);
            console.log(`Game vault amount: ${gameVaultInfo.amount}`);
            expect(Number(ataInfo.amount)).to.equal(ataBalance - gameAccount.entryStake.toNumber(), "Balance after joining does not match");
            expect(Number(gameVaultInfo.amount)).to.equal(gameAccount.entryStake.toNumber() * 3, "Game vault amount does not match");
            expect(gameAccount.randomSeed).to.not.equal(null, "Random seed was not generated");
            expect(gameAccount.players.length).to.equal(3, "Players length does not match");
            // console.info(gameAccount);
            // console.log(gameAccount.players[0]);
            // console.log(gameAccount.players[1]);
        });
    });

}); 