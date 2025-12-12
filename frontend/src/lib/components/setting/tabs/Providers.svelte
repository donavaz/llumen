<script lang="ts">
	import { providersStore } from '$lib/providers/store.svelte';
	import { testConnection, listModels } from '$lib/providers/api';
	import { PROVIDER_INFO, type ProviderType } from '$lib/providers/types';

	let apiKeys: Record<ProviderType, string> = $state({
		openai: '',
		anthropic: '',
		google: '',
		openrouter: ''
	});

	let showKeys: Record<ProviderType, boolean> = $state({
		openai: false,
		anthropic: false,
		google: false,
		openrouter: false
	});

	let testing: Record<ProviderType, boolean> = $state({
		openai: false,
		anthropic: false,
		google: false,
		openrouter: false
	});

	$effect(() => {
		const openaiConfig = providersStore.getConfig('openai');
		if (openaiConfig) apiKeys.openai = openaiConfig.apiKey;

		const anthropicConfig = providersStore.getConfig('anthropic');
		if (anthropicConfig) apiKeys.anthropic = anthropicConfig.apiKey;

		const googleConfig = providersStore.getConfig('google');
		if (googleConfig) apiKeys.google = googleConfig.apiKey;

		const openrouterConfig = providersStore.getConfig('openrouter');
		if (openrouterConfig) apiKeys.openrouter = openrouterConfig.apiKey;
	});

	async function handleTestConnection(provider: ProviderType) {
		if (!apiKeys[provider]) return;

		testing[provider] = true;

		try {
			const config = {
				providerType: provider,
				apiKey: apiKeys[provider]
			};

			const result = await testConnection(config);

			if (result.success) {
				await providersStore.setConfig(provider, config);
				await providersStore.setStatus(provider, {
					provider,
					status: 'active',
					lastTested: new Date()
				});

				const models = await listModels(config);
				await providersStore.setModels(provider, models);
			} else {
				await providersStore.setStatus(provider, {
					provider,
					status: 'failed',
					lastTested: new Date(),
					error: result.error
				});
			}
		} catch (error: any) {
			await providersStore.setStatus(provider, {
				provider,
				status: 'failed',
				lastTested: new Date(),
				error: error.message
			});
		} finally {
			testing[provider] = false;
		}
	}

	function getStatusIcon(provider: ProviderType): string {
		const status = providersStore.getStatus(provider);
		if (!status) return '⊙';
		switch (status.status) {
			case 'active':
				return '✓';
			case 'failed':
				return '✗';
			default:
				return '⊙';
		}
	}

	function getStatusText(provider: ProviderType): string {
		const status = providersStore.getStatus(provider);
		if (!status) return 'Not configured';

		switch (status.status) {
			case 'active':
				return 'Connected';
			case 'failed':
				return 'Failed';
			default:
				return 'Untested';
		}
	}
</script>

<div class="providers-content nobar">
	<p class="description">
		Configure AI providers to enable multiple model support. Keys are encrypted locally.
	</p>

	<div class="providers-list">
		{#each Object.entries(PROVIDER_INFO) as [providerKey, info]}
			{@const provider = providerKey as ProviderType}
			<div class="provider-item" style="border-left-color: {info.color}">
				<div class="provider-info">
					<span class="provider-icon">{info.icon}</span>
					<span class="provider-name">{info.name}</span>
					<span
						class="status-badge"
						class:active={providersStore.getStatus(provider)?.status === 'active'}
						class:failed={providersStore.getStatus(provider)?.status === 'failed'}
					>
						{getStatusIcon(provider)} {getStatusText(provider)}
					</span>
				</div>

				<div class="input-group">
					<div class="input-wrapper">
						<input
							type={showKeys[provider] ? 'text' : 'password'}
							bind:value={apiKeys[provider]}
							placeholder="sk-..."
							class="key-input"
						/>
						<button
							class="toggle-btn"
							onclick={() => (showKeys[provider] = !showKeys[provider])}
							type="button"
						>
							{showKeys[provider] ? '👁️' : '👁️‍🗨️'}
						</button>
					</div>
					<button
						class="test-btn"
						onclick={() => handleTestConnection(provider)}
						disabled={!apiKeys[provider] || testing[provider]}
						type="button"
					>
						{testing[provider] ? 'Testing...' : 'Test'}
					</button>
				</div>

				{#if providersStore.getModels(provider).length > 0}
					<div class="models-count">
						{providersStore.getModels(provider).length} models available
					</div>
				{/if}
			</div>
		{/each}
	</div>
</div>

<style>
	.providers-content {
		height: 100%;
		overflow-y: auto;
	}

	.description {
		margin-bottom: 1.5rem;
		font-size: 0.9rem;
		color: var(--text-secondary, #666);
	}

	.providers-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.provider-item {
		padding: 1rem;
		background: var(--surface-elevated, rgba(0, 0, 0, 0.02));
		border-radius: 8px;
		border-left: 3px solid;
	}

	.provider-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.75rem;
	}

	.provider-icon {
		font-size: 1.5rem;
	}

	.provider-name {
		font-weight: 600;
		flex: 1;
	}

	.status-badge {
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		font-size: 0.75rem;
		background: var(--surface, rgba(0, 0, 0, 0.05));
	}

	.status-badge.active {
		background: rgba(16, 185, 129, 0.1);
		color: rgb(5, 150, 105);
	}

	.status-badge.failed {
		background: rgba(239, 68, 68, 0.1);
		color: rgb(220, 38, 38);
	}

	.input-group {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.input-wrapper {
		position: relative;
		flex: 1;
	}

	.key-input {
		width: 100%;
		padding: 0.5rem;
		padding-right: 2.5rem;
		border: 1px solid var(--border, rgba(0, 0, 0, 0.1));
		border-radius: 6px;
		font-size: 0.85rem;
		font-family: monospace;
	}

	.key-input:focus {
		outline: none;
		border-color: var(--primary, #3b82f6);
	}

	.toggle-btn {
		position: absolute;
		right: 0.25rem;
		top: 50%;
		transform: translateY(-50%);
		background: none;
		border: none;
		cursor: pointer;
		padding: 0.25rem;
	}

	.test-btn {
		padding: 0.5rem 1rem;
		background: var(--primary, #3b82f6);
		color: white;
		border: none;
		border-radius: 6px;
		font-size: 0.85rem;
		cursor: pointer;
		white-space: nowrap;
	}

	.test-btn:hover:not(:disabled) {
		opacity: 0.9;
	}

	.test-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.models-count {
		font-size: 0.8rem;
		color: var(--text-secondary, #666);
		padding-top: 0.5rem;
		border-top: 1px solid var(--border, rgba(0, 0, 0, 0.05));
	}
</style>
