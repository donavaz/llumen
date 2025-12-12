import { encryptString, decryptString } from '$lib/crypto';
import type { ProviderConfig, ProviderStatus, ProviderType, ModelInfo } from './types';

const STORAGE_KEY = 'llumen_providers';

class ProvidersStore {
	private configs = $state<Map<ProviderType, ProviderConfig>>(new Map());
	private statuses = $state<Map<ProviderType, ProviderStatus>>(new Map());
	private models = $state<Map<ProviderType, ModelInfo[]>>(new Map());

	constructor() {
		if (typeof window !== 'undefined') {
			this.load();
		}
	}

	async load() {
		const stored = localStorage.getItem(STORAGE_KEY);
		if (!stored) return;

		try {
			const encrypted = JSON.parse(stored);
			const decrypted = await decryptString(encrypted);
			const data = JSON.parse(decrypted);

			this.configs = new Map(Object.entries(data.configs || {}));
			this.statuses = new Map(Object.entries(data.statuses || {}).map(([k, v]: [string, any]) => [
				k as ProviderType,
				{
					...v,
					lastTested: v.lastTested ? new Date(v.lastTested) : undefined
				}
			]));
			this.models = new Map(Object.entries(data.models || {}));
		} catch (e) {
			console.error('Failed to load provider configs:', e);
		}
	}

	async save() {
		const data = {
			configs: Object.fromEntries(this.configs),
			statuses: Object.fromEntries(
				Array.from(this.statuses.entries()).map(([k, v]) => [
					k,
					{
						...v,
						lastTested: v.lastTested?.toISOString()
					}
				])
			),
			models: Object.fromEntries(this.models)
		};

		const json = JSON.stringify(data);
		const encrypted = await encryptString(json);
		localStorage.setItem(STORAGE_KEY, JSON.stringify(encrypted));
	}

	getConfig(provider: ProviderType): ProviderConfig | undefined {
		return this.configs.get(provider);
	}

	async setConfig(provider: ProviderType, config: ProviderConfig) {
		this.configs.set(provider, config);
		await this.save();
	}

	async removeConfig(provider: ProviderType) {
		this.configs.delete(provider);
		this.statuses.delete(provider);
		this.models.delete(provider);
		await this.save();
	}

	getStatus(provider: ProviderType): ProviderStatus | undefined {
		return this.statuses.get(provider);
	}

	async setStatus(provider: ProviderType, status: ProviderStatus) {
		this.statuses.set(provider, status);
		await this.save();
	}

	getModels(provider: ProviderType): ModelInfo[] {
		return this.models.get(provider) || [];
	}

	async setModels(provider: ProviderType, models: ModelInfo[]) {
		this.models.set(provider, models);
		await this.save();
	}

	getAllModels(): ModelInfo[] {
		return Array.from(this.models.values()).flat();
	}

	getConfiguredProviders(): ProviderType[] {
		return Array.from(this.configs.keys());
	}

	async clearAll() {
		this.configs.clear();
		this.statuses.clear();
		this.models.clear();
		localStorage.removeItem(STORAGE_KEY);
	}
}

export const providersStore = new ProvidersStore();
