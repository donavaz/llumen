<script lang="ts">
	import { providersStore } from '$lib/providers/store.svelte';
	import { PROVIDER_INFO, type ProviderType } from '$lib/providers/types';
	import { LoaderCircle, ChevronDown } from '@lucide/svelte';

	let { value = $bindable<string | undefined>(), disabled = false } = $props();

	let open = $state(false);
	let selectedModelName = $state('Select Model');

	const allModels = $derived(providersStore.getAllModels());
	const groupedModels = $derived(
		allModels.reduce((acc, model) => {
			if (!acc[model.provider]) {
				acc[model.provider] = [];
			}
			acc[model.provider].push(model);
			return acc;
		}, {} as Record<ProviderType, typeof allModels>)
	);

	$effect(() => {
		if (value) {
			const model = allModels.find(m => m.id === value);
			if (model) {
				selectedModelName = model.displayName;
			}
		}
	});

	function selectModel(modelId: string, modelName: string) {
		value = modelId;
		selectedModelName = modelName;
		open = false;
	}

	function toggleDropdown() {
		if (!disabled && allModels.length > 0) {
			open = !open;
		}
	}

	function getCapabilityBadges(model: typeof allModels[0]): string[] {
		const badges: string[] = [];
		if (model.capabilities.vision) badges.push('👁️');
		if (model.capabilities.imageGen) badges.push('🎨');
		if (model.capabilities.files) badges.push('📎');
		if (model.capabilities.audio) badges.push('🔊');
		return badges;
	}
</script>

<div class="model-selector" class:disabled>
	<button
		type="button"
		class="selector-button"
		onclick={toggleDropdown}
		{disabled}
		class:open
	>
		<span class="selected-text">
			{selectedModelName}
		</span>
		<ChevronDown size={16} class="chevron" class:rotated={open} />
	</button>

	{#if open}
		<div class="dropdown">
			{#if allModels.length === 0}
				<div class="empty-state">
					<p>No models available</p>
					<p class="hint">Configure API keys in Settings</p>
				</div>
			{:else}
				{#each Object.entries(groupedModels) as [providerKey, models]}
					{@const provider = providerKey as ProviderType}
					{@const info = PROVIDER_INFO[provider]}
					<div class="provider-group">
						<div class="provider-header" style="color: {info.color}">
							<span class="provider-icon">{info.icon}</span>
							<span class="provider-name">{info.name}</span>
						</div>
						<div class="models-list">
							{#each models as model}
								<button
									type="button"
									class="model-item"
									class:selected={value === model.id}
									onclick={() => selectModel(model.id, model.displayName)}
								>
									<div class="model-info">
										<span class="model-name">{model.displayName}</span>
										{#if model.contextWindow}
											<span class="model-context">{(model.contextWindow / 1000).toFixed(0)}K</span>
										{/if}
									</div>
									<div class="model-badges">
										{#each getCapabilityBadges(model) as badge}
											<span class="badge">{badge}</span>
										{/each}
									</div>
								</button>
							{/each}
						</div>
					</div>
				{/each}
			{/if}
		</div>
	{/if}
</div>

{#if open}
	<button
		type="button"
		class="backdrop"
		onclick={() => (open = false)}
		aria-label="Close dropdown"
	></button>
{/if}

<style>
	.model-selector {
		position: relative;
		width: 100%;
		max-width: 300px;
	}

	.selector-button {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: var(--surface, #ffffff);
		border: 1px solid var(--border, #d1d5db);
		border-radius: 8px;
		font-size: 0.95rem;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.selector-button:hover:not(:disabled) {
		border-color: var(--primary, #3b82f6);
	}

	.selector-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.selector-button.open {
		border-color: var(--primary, #3b82f6);
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.selected-text {
		flex: 1;
		text-align: left;
		color: var(--text-primary, #1a1a1a);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.chevron {
		transition: transform 0.2s ease;
		color: var(--text-secondary, #666);
	}

	.chevron.rotated {
		transform: rotate(180deg);
	}

	.dropdown {
		position: absolute;
		top: calc(100% + 0.5rem);
		left: 0;
		right: 0;
		max-height: 400px;
		overflow-y: auto;
		background: var(--surface, #ffffff);
		border: 1px solid var(--border, #d1d5db);
		border-radius: 8px;
		box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
		z-index: 1000;
		animation: slideDown 0.15s ease;
	}

	@keyframes slideDown {
		from {
			opacity: 0;
			transform: translateY(-8px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.empty-state {
		padding: 2rem;
		text-align: center;
		color: var(--text-secondary, #666);
	}

	.empty-state p {
		margin: 0.25rem 0;
	}

	.empty-state .hint {
		font-size: 0.85rem;
		color: var(--text-tertiary, #999);
	}

	.provider-group {
		padding: 0.5rem 0;
	}

	.provider-group:not(:last-child) {
		border-bottom: 1px solid var(--border, #e5e7eb);
	}

	.provider-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		font-size: 0.85rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.provider-icon {
		font-size: 1rem;
	}

	.models-list {
		display: flex;
		flex-direction: column;
	}

	.model-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: transparent;
		border: none;
		cursor: pointer;
		text-align: left;
		transition: background 0.1s ease;
	}

	.model-item:hover {
		background: var(--surface-hover, #f3f4f6);
	}

	.model-item.selected {
		background: var(--primary-light, #eff6ff);
	}

	.model-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		flex: 1;
	}

	.model-name {
		font-size: 0.95rem;
		color: var(--text-primary, #1a1a1a);
		font-weight: 500;
	}

	.model-context {
		font-size: 0.8rem;
		color: var(--text-secondary, #666);
	}

	.model-badges {
		display: flex;
		gap: 0.25rem;
	}

	.badge {
		font-size: 0.85rem;
	}

	.backdrop {
		position: fixed;
		inset: 0;
		background: transparent;
		border: none;
		cursor: default;
		z-index: 999;
	}
</style>
