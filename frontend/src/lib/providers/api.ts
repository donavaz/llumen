import type { ProviderConfig, ModelInfo } from './types';

const API_BASE = '/api/provider';

export async function testConnection(config: ProviderConfig): Promise<{ success: boolean; error?: string }> {
	const response = await fetch(`${API_BASE}/test`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${localStorage.getItem('token') || ''}`
		},
		body: JSON.stringify({
			provider_config: {
				provider_type: config.providerType,
				api_key: config.apiKey,
				base_url: config.baseUrl
			}
		})
	});

	if (!response.ok) {
		throw new Error('Failed to test connection');
	}

	return response.json();
}

export async function listModels(config: ProviderConfig): Promise<ModelInfo[]> {
	const response = await fetch(`${API_BASE}/models`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${localStorage.getItem('token') || ''}`
		},
		body: JSON.stringify({
			provider_config: {
				provider_type: config.providerType,
				api_key: config.apiKey,
				base_url: config.baseUrl
			}
		})
	});

	if (!response.ok) {
		throw new Error('Failed to list models');
	}

	const data = await response.json();
	return data.models || [];
}
