<script>
	import { createEventDispatcher } from 'svelte';

	export let file;

	const dispatch = createEventDispatcher();

	function formatSize(bytes) {
		if (!bytes) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
	}

	function toggleSelect() {
		dispatch('update', { ...file, selected: !file.selected });
	}

	function handleStop() {
		dispatch('update', { ...file, status: 'stopped' });
	}

	function handleRemove() {
		dispatch('remove', file.id);
	}
</script>

<div class="file-row" class:processing={file.status === 'processing'}>
	<div class="progress-bar" class:active={file.status === 'processing'}>
		<div class="progress-fill" style="width: {file.progress}%"></div>
	</div>

	<div class="row-content">
		<div class="left-section">
			<label class="checkbox-label">
				<input
					type="checkbox"
					checked={file.selected}
					on:change={toggleSelect}
					disabled={file.status === 'processing'}
				/>
			</label>
			<div class="file-icon">📄</div>
			<div class="file-info">
				<div class="file-name">{file.name}</div>
				{#if file.status === 'error'}
					<div class="file-error">{file.error}</div>
				{/if}
			</div>
		</div>

		<div class="middle-section">
			<div class="size-info">
				<span>{formatSize(file.size)}</span>
				{#if file.outputSize !== null}
					<span class="arrow">→</span>
					<span>{formatSize(file.outputSize)}</span>
				{/if}
			</div>
		</div>

		<div class="right-section">
			{#if file.status === 'processing'}
				<button class="action-btn stop-btn" on:click={handleStop} title="Stop processing">
					■
				</button>
			{:else}
				<button class="action-btn remove-btn" on:click={handleRemove} title="Remove file">
					✕
				</button>
			{/if}
		</div>
	</div>
</div>

<style>
	.file-row {
		position: relative;
		margin: 0 8px 4px;
		border-radius: 6px;
		background: white;
		border: 1px solid #e0e0e0;
		overflow: hidden;
	}

	.progress-bar {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 100%;
		opacity: 0;
		background: rgba(102, 126, 234, 0.1);
		transition: opacity 0.2s;
		pointer-events: none;
	}

	.progress-bar.active {
		opacity: 1;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #667eea, #764ba2);
		width: 0%;
		transition: width 0.3s ease;
		opacity: 0.3;
	}

	.row-content {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		position: relative;
		z-index: 1;
	}

	.left-section {
		display: flex;
		align-items: center;
		gap: 12px;
		flex: 1;
		min-width: 0;
	}

	.checkbox-label {
		display: flex;
		cursor: pointer;
	}

	.checkbox-label input {
		cursor: pointer;
	}

	.checkbox-label input:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.file-icon {
		font-size: 20px;
		flex-shrink: 0;
	}

	.file-info {
		flex: 1;
		min-width: 0;
	}

	.file-name {
		font-size: 14px;
		font-weight: 500;
		color: #333;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.file-error {
		font-size: 12px;
		color: #d32f2f;
		margin-top: 4px;
	}

	.middle-section {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.size-info {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 12px;
		color: #666;
		white-space: nowrap;
		min-width: fit-content;
	}

	.size-info .arrow {
		color: #ccc;
	}

	.right-section {
		display: flex;
		gap: 8px;
	}

	.action-btn {
		width: 28px;
		height: 28px;
		border: 1px solid #ddd;
		border-radius: 4px;
		background: white;
		cursor: pointer;
		font-size: 14px;
		transition: all 0.2s;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.action-btn:hover {
		background: #f5f5f5;
		border-color: #999;
	}

	.remove-btn:hover {
		color: #d32f2f;
		border-color: #d32f2f;
	}

	.stop-btn:hover {
		color: #ff9800;
		border-color: #ff9800;
	}

	.file-row.processing {
		background: #fafafa;
	}
</style>
