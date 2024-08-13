import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        //2. Convert image to generic file.
        //3. Upload image

        const image = await readFile("/Users/nashback/Q3T_SOL_nash_back/solana-starter/ts/cluster1/assets/generug.png");

        const genericFile = createGenericFile(image, "rug", {contentType: "image/png"});

        const [myUri] = await umi.uploader.upload([genericFile]);


        console.log("Your image URI: ", myUri);

        // Your image URI:  https://arweave.net/nWQ-EAIzcD5fczb-M2skLS-s1-R6_er2OklicNHRSZI
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
