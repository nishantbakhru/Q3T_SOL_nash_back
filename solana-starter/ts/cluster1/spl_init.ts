import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from '@solana/spl-token';
import wallet from "../wba-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed"; //66% of the network must agree on the block
const connection = new Connection("https://api.devnet.solana.com", commitment);

(async () => {
    try {
        // Start here
        const mint = await createMint(connection, keypair, keypair.publicKey, null, 6);
        console.log(`Mint address: ${mint.toBase58()}`);
        // GwCvCVDqS1GJGdwnGSYXWA5ve5WsSqarsiVA3reJRqn3

    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
