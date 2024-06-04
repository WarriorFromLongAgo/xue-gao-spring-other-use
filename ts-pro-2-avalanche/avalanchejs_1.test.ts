import {base58check} from './base58/base58';
import {secp256k1} from './crypto';
import * as address from './utils/address';

describe('Avalanche AVAX', () => {
    it(' create address 1 ', () => {
        const key: string = '24jUJ9vZexUM6expyMcT48LBx27k1m7xpraoV62oSQAHdziao5';
        console.log("key ", key)
        const privKey = base58check.decode(key);
        const pubKey = secp256k1.getPublicKey(privKey);

        const addrBytes = secp256k1.publicKeyBytesToAddress(pubKey);

        const addr = address.format('X', 'avax', addrBytes);
        console.log(" addr ", addr)
        console.log(" address.parse(addr) ", address.parse(addr));
    });
});

