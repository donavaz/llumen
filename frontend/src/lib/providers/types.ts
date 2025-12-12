export type ProviderType = 'openai' | 'anthropic' | 'google' | 'openrouter';

export interface ProviderConfig {
	providerType: ProviderType;
	apiKey: string;
	baseUrl?: string;
}

export interface ModelInfo {
	id: string;
	displayName: string;
	provider: ProviderType;
	capabilities: ModelCapabilities;
	pricing?: ModelPricing;
	contextWindow?: number;
}

export interface ModelCapabilities {
	text: boolean;
	vision: boolean;
	imageGen: boolean;
	files: boolean;
	audio: boolean;
}

export interface ModelPricing {
	inputCostPer1m: number;
	outputCostPer1m: number;
}

export interface ProviderStatus {
	provider: ProviderType;
	status: 'active' | 'failed' | 'untested';
	lastTested?: Date;
	error?: string;
}

export const PROVIDER_INFO: Record<ProviderType, { name: string; icon: string; color: string }> = {
	openai: {
		name: 'OpenAI',
		icon: '🤖',
		color: '#10a37f'
	},
	anthropic: {
		name: 'Anthropic',
		icon: '🧠',
		color: '#d97757'
	},
	google: {
		name: 'Google AI',
		icon: '✨',
		color: '#4285f4'
	},
	openrouter: {
		name: 'OpenRouter',
		icon: '🔀',
		color: '#8b5cf6'
	}
};
