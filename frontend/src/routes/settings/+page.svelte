<script lang="ts">
	import { providersStore } from '$lib/providers/store.svelte';
	import { testConnection, listModels } from '$lib/providers/api';
	import { PROVIDER_INFO, type ProviderType } from '$lib/providers/types';
	import { onMount } from 'svelte';

	let selectedProvider: ProviderType = 'openai';
	let apiKeys: Record<ProviderType, string> = {
		openai: '',
		anthropic: '',
		google: '',
		openrouter: ''
	};
	let showKeys: Record<ProviderType, boolean> = {
		openai: false,
		anthropic: false,
		google: false,
		openrouter: false
	};
	let testing: Record<ProviderType, boolean> = {
		openai: false,
		anthropic: false,
		google: false,
		openrouter: false
	};
	let autoSelectBest = $state(true);
	let showTokenUsage = $state(true);
	let streamResponses = $state(true);

	onMount(async () => {
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
				const timeSince = status.lastTested ? getTimeSince(status.lastTested) : '';
				return `Connected ${timeSince}`;
			case 'failed':
				return `Connection failed${status.error ? `: ${status.error}` : ''}`;
			default:
				return 'Not tested';
		}
	}

	function getTimeSince(date: Date): string {
		const seconds = Math.floor((Date.now() - date.getTime()) / 1000);

		if (seconds < 60) return `${seconds}s ago`;
		if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`;
		if (seconds < 86400) return `${Math.floor(seconds / 3600)}h ago`;
		return `${Math.floor(seconds / 86400)}d ago`;
	}

	async function handleClearAll() {
		if (confirm('Are you sure you want to clear all API keys and settings?')) {
			await providersStore.clearAll();
			apiKeys = {
				openai: '',
				anthropic: '',
				google: '',
				openrouter: ''
			};
		}
	}

	function getModelBadges(provider: ProviderType): string[] {
		const models = providersStore.getModels(provider);
		if (models.length === 0) return [];

		const badges: Set<string> = new Set();
		models.forEach(model => {
			if (model.capabilities.text) badges.add('text');
			if (model.capabilities.vision) badges.add('vision');
			if (model.capabilities.imageGen) badges.add('image-gen');
			if (model.capabilities.files) badges.add('files');
			if (model.capabilities.audio) badges.add('audio');
		});

		return Array.from(badges);
	}
</script>

<div class="settings-page">
	<div class="settings-header">
		<h1>Settings</h1>
		<p>Configure AI providers and manage your API keys</p>
	</div>

	<div class="settings-content">
		<section class="api-keys-section">
			<h2>API Keys</h2>
			<p class="section-description">
				Add your API keys to enable multiple AI providers. Keys are encrypted and stored locally
				in your browser.
			</p>

			{#each Object.entries(PROVIDER_INFO) as [providerKey, info]}
				{@const provider = providerKey as ProviderType}
				<div class="provider-card" style="border-left: 4px solid {info.color}">
					<div class="provider-header">
						<div class="provider-title">
							<span class="provider-icon" style="font-size: 24px">{info.icon}</span>
							<h3>{info.name}</h3>
						</div>
						<span class="status-badge" class:active={providersStore.getStatus(provider)?.status === 'active'}
							class:failed={providersStore.getStatus(provider)?.status === 'failed'}>
							{getStatusIcon(provider)} {getStatusText(provider)}
						</span>
					</div>

					<div class="api-key-input-group">
						<div class="input-wrapper">
							<input
								type={showKeys[provider] ? 'text' : 'password'}
								bind:value={apiKeys[provider]}
								placeholder="Enter API key..."
								class="api-key-input"
							/>
							<button
								class="toggle-visibility"
								onclick={() => (showKeys[provider] = !showKeys[provider])}
								aria-label="Toggle visibility"
							>
								{showKeys[provider] ? '👁️' : '👁️‍🗨️'}
							</button>
						</div>
						<button
							class="test-button"
							onclick={() => handleTestConnection(provider)}
							disabled={!apiKeys[provider] || testing[provider]}
						>
							{testing[provider] ? 'Testing...' : 'Test Connection'}
						</button>
					</div>

					{#if providersStore.getModels(provider).length > 0}
						<div class="models-info">
							<span class="models-count">
								{providersStore.getModels(provider).length} models available
							</span>
							<div class="capability-badges">
								{#each getModelBadges(provider) as badge}
									<span class="badge">{badge}</span>
								{/each}
							</div>
						</div>
					{/if}

					{#if providersStore.getStatus(provider)?.status === 'failed'}
						<div class="error-message">
							{providersStore.getStatus(provider)?.error || 'Connection failed'}
						</div>
					{/if}
				</div>
			{/each}
		</section>

		<section class="preferences-section">
			<h2>Preferences</h2>

			<label class="checkbox-label">
				<input type="checkbox" bind:checked={autoSelectBest} />
				<span>Auto-select best model for task</span>
			</label>

			<label class="checkbox-label">
				<input type="checkbox" bind:checked={showTokenUsage} />
				<span>Show token usage</span>
			</label>

			<label class="checkbox-label">
				<input type="checkbox" bind:checked={streamResponses} />
				<span>Stream responses</span>
			</label>
		</section>

		<section class="danger-section">
			<h2>Danger Zone</h2>
			<button class="danger-button" onclick={handleClearAll}>Clear All Settings</button>
		</section>
	</div>
</div>

<style>
	.settings-page {
		max-width: 900px;
		margin: 0 auto;
		padding: 2rem;
	}

	.settings-header {
		margin-bottom: 3rem;
	}

	.settings-header h1 {
		font-size: 2rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
		color: var(--text-primary, #1a1a1a);
	}

	.settings-header p {
		color: var(--text-secondary, #666);
		font-size: 1rem;
	}

	.settings-content {
		display: flex;
		flex-direction: column;
		gap: 3rem;
	}

	section {
		background: var(--surface, #ffffff);
		border-radius: 12px;
		padding: 2rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	section h2 {
		font-size: 1.25rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
		color: var(--text-primary, #1a1a1a);
	}

	.section-description {
		color: var(--text-secondary, #666);
		margin-bottom: 1.5rem;
		font-size: 0.9rem;
	}

	.provider-card {
		background: var(--surface-elevated, #f9fafb);
		border-radius: 8px;
		padding: 1.5rem;
		margin-bottom: 1rem;
	}

	.provider-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.provider-title {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.provider-title h3 {
		font-size: 1.1rem;
		font-weight: 600;
		margin: 0;
		color: var(--text-primary, #1a1a1a);
	}

	.status-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 12px;
		font-size: 0.85rem;
		background: var(--surface, #e5e7eb);
		color: var(--text-secondary, #666);
	}

	.status-badge.active {
		background: #d1fae5;
		color: #065f46;
	}

	.status-badge.failed {
		background: #fee2e2;
		color: #991b1b;
	}

	.api-key-input-group {
		display: flex;
		gap: 0.75rem;
		margin-bottom: 0.75rem;
	}

	.input-wrapper {
		position: relative;
		flex: 1;
	}

	.api-key-input {
		width: 100%;
		padding: 0.75rem;
		padding-right: 3rem;
		border: 1px solid var(--border, #d1d5db);
		border-radius: 6px;
		font-size: 0.95rem;
		font-family: 'Monaco', 'Courier New', monospace;
	}

	.api-key-input:focus {
		outline: none;
		border-color: var(--primary, #3b82f6);
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.toggle-visibility {
		position: absolute;
		right: 0.5rem;
		top: 50%;
		transform: translateY(-50%);
		background: none;
		border: none;
		cursor: pointer;
		padding: 0.5rem;
		font-size: 1.2rem;
	}

	.test-button {
		padding: 0.75rem 1.5rem;
		background: var(--primary, #3b82f6);
		color: white;
		border: none;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		white-space: nowrap;
	}

	.test-button:hover:not(:disabled) {
		background: var(--primary-hover, #2563eb);
	}

	.test-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.models-info {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 0.75rem;
		padding-top: 0.75rem;
		border-top: 1px solid var(--border, #e5e7eb);
		font-size: 0.9rem;
	}

	.models-count {
		color: var(--text-secondary, #666);
	}

	.capability-badges {
		display: flex;
		gap: 0.5rem;
	}

	.badge {
		padding: 0.25rem 0.5rem;
		background: var(--surface, #e5e7eb);
		border-radius: 4px;
		font-size: 0.75rem;
		color: var(--text-secondary, #666);
	}

	.error-message {
		margin-top: 0.75rem;
		padding: 0.75rem;
		background: #fee2e2;
		border-radius: 6px;
		color: #991b1b;
		font-size: 0.9rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 0;
		cursor: pointer;
	}

	.checkbox-label input[type='checkbox'] {
		width: 1.25rem;
		height: 1.25rem;
		cursor: pointer;
	}

	.checkbox-label span {
		font-size: 1rem;
		color: var(--text-primary, #1a1a1a);
	}

	.danger-section {
		border: 1px solid #fecaca;
	}

	.danger-button {
		padding: 0.75rem 1.5rem;
		background: #dc2626;
		color: white;
		border: none;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
	}

	.danger-button:hover {
		background: #b91c1c;
	}
</style>
