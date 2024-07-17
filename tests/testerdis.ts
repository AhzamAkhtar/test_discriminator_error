import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {Testerdis, IDL} from "../target/types/testerdis";
import {PublicKey, Commitment, Keypair, SystemProgram} from "@solana/web3.js"
import * as bs58 from "bs58";
import {
    ASSOCIATED_TOKEN_PROGRAM_ID as associatedTokenProgram,
    TOKEN_PROGRAM_ID as tokenProgram,
    createMint,
    createAccount,
    mintTo,
    getAssociatedTokenAddress,
    TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token"

describe("testerdis", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const programId = new PublicKey("FJ6ZsJXSxXQjrTh7N9RetoFi8AyfLKYQhQPnvAfyhwVU");
    const program = new anchor.Program<Testerdis>(IDL, programId, anchor.getProvider());

    const initializer = Keypair.fromSecretKey(bs58.decode(wallet));

    it("Is initialized!", async () => {
        // Add your test here.
        try {
            const [profilePda] = PublicKey.findProgramAddressSync(
                [Buffer.from("USER_TAG")],
                program.programId
            );

            const [queueePda] = PublicKey.findProgramAddressSync(
                [Buffer.from("WITHDRAW_QUEUE_SEED")],
                program.programId
            );

            const tx = await program.methods.initializeUser(
                "serialization",
                "age_ser",
                "gender",
                "profile_url_new",
                "new_descp",
                "new_country"
            ).accounts({
                userProfileVault: profilePda,
                authority: initializer.publicKey,
                withdrawQueueHeader : queueePda,
                systemProgram: SystemProgram.programId,
            })
                .signers([
                    initializer
                ])
                .rpc({
                    skipPreflight: true

                })

            console.log("Your transaction signature", tx);
        } catch (e) {
            console
                .log(e)
        }

    });

    it("fetch user_account", async () => {
        try {
            const user_account = await program.account.userProfileVault.all()
            console.log(user_account)
        } catch (e) {
            console.log(e)
        }
    })
});
