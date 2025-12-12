const ENCRYPTION_KEY_NAME = 'llumen_encryption_key';

async function getOrCreateEncryptionKey(): Promise<CryptoKey> {
	const stored = localStorage.getItem(ENCRYPTION_KEY_NAME);

	if (stored) {
		const keyData = JSON.parse(stored);
		return await crypto.subtle.importKey(
			'jwk',
			keyData,
			{ name: 'AES-GCM', length: 256 },
			true,
			['encrypt', 'decrypt']
		);
	}

	const key = await crypto.subtle.generateKey(
		{ name: 'AES-GCM', length: 256 },
		true,
		['encrypt', 'decrypt']
	);

	const exported = await crypto.subtle.exportKey('jwk', key);
	localStorage.setItem(ENCRYPTION_KEY_NAME, JSON.stringify(exported));

	return key;
}

export async function encryptString(plaintext: string): Promise<string> {
	const key = await getOrCreateEncryptionKey();
	const iv = crypto.getRandomValues(new Uint8Array(12));
	const encoded = new TextEncoder().encode(plaintext);

	const ciphertext = await crypto.subtle.encrypt(
		{ name: 'AES-GCM', iv },
		key,
		encoded
	);

	const combined = new Uint8Array(iv.length + ciphertext.byteLength);
	combined.set(iv, 0);
	combined.set(new Uint8Array(ciphertext), iv.length);

	return btoa(String.fromCharCode(...combined));
}

export async function decryptString(encrypted: string): Promise<string> {
	const key = await getOrCreateEncryptionKey();
	const combined = Uint8Array.from(atob(encrypted), c => c.charCodeAt(0));

	const iv = combined.slice(0, 12);
	const ciphertext = combined.slice(12);

	const decrypted = await crypto.subtle.decrypt(
		{ name: 'AES-GCM', iv },
		key,
		ciphertext
	);

	return new TextDecoder().decode(decrypted);
}
