import { Aes256Key, AesKey } from "@tutao/tutanota-crypto"
import type { Base64 } from "@tutao/tutanota-utils"
import type { TypeModel } from "./EntityTypes.js"

/**
 * This is a wrapper for commonly used crypto functions, easier to inject/swap implementations and test.
 */
export interface CryptoFunctions {
	aesEncrypt(key: AesKey, bytes: Uint8Array, iv?: Uint8Array, usePadding?: boolean, useMac?: boolean): Uint8Array

	aesDecrypt(key: AesKey, encryptedBytes: Uint8Array, usePadding: boolean): Uint8Array

	unauthenticatedAesDecrypt(key: Aes256Key, encryptedBytes: Uint8Array, usePadding: boolean): Uint8Array

	decryptKey(encryptionKey: AesKey, key: Uint8Array): AesKey

	bytesToKey(bytes: Uint8Array): BitArray

	base64ToKey(base64: Base64): BitArray

	verifySignature(pubKeyPem: string, data: Uint8Array, signature: Uint8Array): boolean

	randomBytes(nbrOfBytes: number): Uint8Array

	aes256RandomKey(): Aes256Key

	decryptAndMapToInstance<T>(model: TypeModel, instance: Record<string, any>, sk: AesKey | null): Promise<T>
}
