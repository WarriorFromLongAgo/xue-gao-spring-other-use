const crypto = require('crypto');
const bs58 = require('bs58');

describe('Avalanche AVAX', function () {
    describe('create address', function () {
        it(' by base58Check ', function () {
            // 使用示例
            const version = Buffer.from('00', 'hex'); // 版本字节，例如比特币地址的版本字节
            const payload = Buffer.from('61626364', 'hex'); // 示例负载数据
            console.log("version = ", version.toString('hex'));
            console.log("payload = ", payload.toString('hex'));
            const encoded = base58CheckEncode(payload, version);
            // 输出Base58Check编码的字符串
            console.log("encoded = ", encoded);

            const decoded = base58CheckDecode(encoded);
            // 打印解码后的版本和负载
            console.log(`Version: ${decoded.version.toString('hex')}`);
            // 将解码后的负载转换为 ASCII 字符串
            console.log(`Payload: ${decoded.payload.toString('hex')}`);
        });
    });
});


function base58CheckEncode(payload, version) {
    const fullData = Buffer.concat([version, payload]);
    const hash = crypto.createHash('sha256').update(crypto.createHash('sha256').update(fullData).digest()).digest();
    const checksum = hash.slice(0, 4);
    const fullDataWithChecksum = Buffer.concat([fullData, checksum]);
    return bs58.encode(fullDataWithChecksum);
}

function base58CheckDecode(encoded) {
    const buffer = bs58.decode(encoded);
    const version = buffer.slice(0, 1);
    const payload = buffer.slice(1, -4);
    console.log("base58CheckDecode version = ", version.toString('hex'));
    console.log("base58CheckDecode payload = ", payload.toString('hex'));

    const checkSum = buffer.slice(-4);

    const hash = crypto.createHash('sha256').update(
        crypto.createHash('sha256').update(Buffer.concat([version, payload])).digest()
    ).digest();
    const newCheckSum = hash.slice(0, 4);

    // 比较校验和是否匹配
    if (checkSum.length !== newCheckSum.length ||
        !checkSum.every((val, index) => val === newCheckSum[index])) {
        throw new Error('Checksum does not match');
    }

    return {version, payload};
}