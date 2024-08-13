import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://arweave.net/nWQ-EAIzcD5fczb-M2skLS-s1-R6_er2OklicNHRSZI";
        const metadata = {
            name: "Bat rug",
            symbol: "BR",
            description: "Batmanrug",
            image,
            attributes: [
                {trait_type: 'hero', value: 'Batman'},
                {trait_type: 'quality', value: 'Legendary'},
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image,
                    },
                ]
            },
            creators: [keypair.publicKey],
        };
        const myUri = await umi.uploader.uploadJson(metadata);

        console.log("Your metadata URI: ", myUri);

        // Your metadata URI:  https://arweave.net/MRIL1Fyiu1be3-dncPOji4dtVI0JiqRQazXJGj54r78
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
