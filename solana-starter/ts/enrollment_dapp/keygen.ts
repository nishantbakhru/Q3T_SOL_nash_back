import * as punycode from 'punycode/';

// console.log(punycode.encode('example'));

// const punycode = require('punycode/');


import { Keypair } from "@solana/web3.js";

//Generate a new keypair
let kp = Keypair.generate()
console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}`)
// B8mQxAXdWmfZ3X4fRACWvCtmoofCvEmDeQrPSXBvShqG

console.log(JSON.stringify(Array.from(kp.secretKey)))


