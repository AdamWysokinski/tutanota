import type { Aes128Key, Aes256Key } from "./Aes.js"
import { aesDecrypt, aesEncrypt, getKeyLengthBytes, KEY_LENGTH_BYTES_AES_128, KEY_LENGTH_BYTES_AES_256 } from "./Aes.js"
import { bitArrayToUint8Array, fixedIv, uint8ArrayToBitArray } from "../misc/Utils.js"
import { assertNotNull, concat, hexToUint8Array, uint8ArrayToHex } from "@tutao/tutanota-utils"
import { hexToRsaPrivateKey, hexToRsaPublicKey, rsaPrivateKeyToHex } from "./Rsa.js"
import type { RsaKeyPair, RsaPrivateKey } from "./RsaKeyPair.js"
import { PQKeyPairs } from "./PQKeyPairs.js"
import { hexToEccPrivateKey, hexToEccPublicKey } from "./Ecc.js"
import { hexToKyberPrivateKey, hexToKyberPublicKey } from "./Liboqs/KyberKeyPair.js"

export function encryptKey(encryptionKey: Aes128Key | Aes256Key, keyToBeEncrypted: Aes128Key | Aes256Key): Uint8Array {
	const keyLength = getKeyLengthBytes(encryptionKey)
	if (keyLength === KEY_LENGTH_BYTES_AES_128) {
		return aesEncrypt(encryptionKey, bitArrayToUint8Array(keyToBeEncrypted), fixedIv, false, false).slice(fixedIv.length)
	} else if (keyLength === KEY_LENGTH_BYTES_AES_256) {
		return aesEncrypt(encryptionKey, bitArrayToUint8Array(keyToBeEncrypted), undefined, false)
	} else {
		throw new Error(`invalid AES key length (must be 128-bit or 256-bit, got ${keyLength} bytes instead)`)
	}
}

export function decryptKey(encryptionKey: Aes128Key | Aes256Key, keyToBeDecrypted: Uint8Array): Aes128Key | Aes256Key {
	const keyLength = getKeyLengthBytes(encryptionKey)
	if (keyLength === KEY_LENGTH_BYTES_AES_128) {
		return uint8ArrayToBitArray(aesDecrypt(encryptionKey, concat(fixedIv, keyToBeDecrypted), false))
	} else if (keyLength === KEY_LENGTH_BYTES_AES_256) {
		return uint8ArrayToBitArray(aesDecrypt(encryptionKey, keyToBeDecrypted, false))
	} else {
		throw new Error(`invalid AES key length (must be 128-bit or 256-bit, got ${keyLength} bytes instead)`)
	}
}

export function aes256DecryptLegacyRecoveryKey(encryptionKey: Aes256Key, keyToBeDecrypted: Uint8Array): Aes256Key {
	// legacy case: recovery code without IV/mac
	if (keyToBeDecrypted.length === KEY_LENGTH_BYTES_AES_128) {
		return uint8ArrayToBitArray(aesDecrypt(encryptionKey, concat(fixedIv, keyToBeDecrypted), false))
	} else {
		return decryptKey(encryptionKey, keyToBeDecrypted)
	}
}

export function encryptRsaKey(encryptionKey: Aes128Key | Aes256Key, privateKey: RsaPrivateKey, iv?: Uint8Array): Uint8Array {
	return aesEncrypt(encryptionKey, hexToUint8Array(rsaPrivateKeyToHex(privateKey)), iv, true, false)
}

export function decryptRsaKey(encryptionKey: Aes128Key | Aes256Key, encryptedPrivateKey: Uint8Array): RsaPrivateKey {
	return hexToRsaPrivateKey(uint8ArrayToHex(aesDecrypt(encryptionKey, encryptedPrivateKey, true)))
}

export function decryptKeyPair(
	encryptionKey: Aes128Key | Aes256Key,
	keyPair: {
		pubEccKey: null | Uint8Array
		pubKyberKey: null | Uint8Array
		pubRsaKey: null | Uint8Array
		symEncPrivEccKey: null | Uint8Array
		symEncPrivKyberKey: null | Uint8Array
		symEncPrivRsaKey: null | Uint8Array
	},
): RsaKeyPair | PQKeyPairs {
	if (keyPair.symEncPrivRsaKey) {
		const rsaPublicKey = hexToRsaPublicKey(uint8ArrayToHex(assertNotNull(keyPair.pubRsaKey)))
		const rsaPrivateKey = hexToRsaPrivateKey(uint8ArrayToHex(aesDecrypt(encryptionKey, keyPair.symEncPrivRsaKey, true)))

		return { publicKey: rsaPublicKey, privateKey: rsaPrivateKey }
	} else {
		const eccPublicKey = hexToEccPublicKey(uint8ArrayToHex(assertNotNull(keyPair.pubEccKey)))
		const eccPrivateKey = hexToEccPrivateKey(uint8ArrayToHex(aesDecrypt(encryptionKey, assertNotNull(keyPair.symEncPrivEccKey))))
		const kyberPublicKey = hexToKyberPublicKey(uint8ArrayToHex(assertNotNull(keyPair.pubKyberKey)))
		const kyberPrivateKey = hexToKyberPrivateKey(uint8ArrayToHex(aesDecrypt(encryptionKey, assertNotNull(keyPair.symEncPrivKyberKey))))

		return new PQKeyPairs(
			{ publicKey: eccPublicKey, privateKey: eccPrivateKey },
			{
				publicKey: kyberPublicKey,
				privateKey: kyberPrivateKey,
			},
		)
	}
}
