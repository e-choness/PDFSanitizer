<script>
	import FileRow from './FileRow.svelte';
	import { createEventDispatcher } from 'svelte';

	export let files = [];
	export let dragActive = false;
	export let handleDragEnter = () => {};
	export let handleDragLeave = () => {};
	export let handleDragOver = () => {};
	export let handleDrop = () => {};

	const dispatch = createEventDispatcher();

	let allSelected = true;

	function toggleSelectAll() {
		allSelected = !allSelected;
		files = files.map(f => ({ ...f, selected: allSelected }));
	}

	function removeAllFiles() {
		files = files.filter(f => f.status !== 'pending' && f.status !== 'done');
	}

	function stopAllFiles() {
		files = files.map(f => f.status === 'processing' ? { ...f, status: 'stopped' } : f);
	}

	function handleStartProcessing() {
		const selectedPending = files.filter(f => f.selected && f.status === 'pending');
		if (selectedPending.length === 0) return;
		dispatch('startProcessing');
	}

	function handleFileUpdate(e) {
		const updated = e.detail;
		files = files.map(f => f.id === updated.id ? { ...f, ...updated } : f);
		dispatch('update', updated);
	}

	function handleRemoveFile(e) {
		const fileId = e.detail;
		files = files.filter(f => f.id !== fileId);
	}

	function handleSelectFolderClick() {
		dispatch('selectFolder');
	}
</script>

<div class="file-list-container">
	<div
		class="drag-drop-zone"
		class:active={dragActive}
		on:dragenter={handleDragEnter}
		on:dragleave={handleDragLeave}
		on:dragover={handleDragOver}
		on:drop={handleDrop}
	>
		<div class="zone-header">
			<div class="left-controls">
				<label class="checkbox-label">
					<input
						type="checkbox"
						bind:checked={allSelected}
						on:change={toggleSelectAll}
					/>
					<span>Select All</span>
				</label>
			</div>
			<div class="right-controls">
				<button class="icon-btn remove-btn" title="Remove all" on:click={removeAllFiles}>
					✕
				</button>
				<button class="icon-btn stop-btn" title="Stop all" on:click={stopAllFiles}>
					■
				</button>
			</div>
		</div>

		{#if files.length === 0}
			<div class="empty-state">
				<div class="empty-icon">📄</div>
				<p>Drag and drop PDF files here</p>
				<p class="or">or</p>
				<button class="select-btn" on:click={handleSelectFolderClick}>Select Folder</button>
			</div>
		{:else}
			<div class="file-rows">
				{#each files as file (file.id)}
					<FileRow
						{file}
						on:update={handleFileUpdate}
						on:remove={handleRemoveFile}
					/>
				{/each}
			</div>
		{/if}
	</div>

	<div class="action-bar">
		<button class="convert-btn" on:click={handleStartProcessing} disabled={files.length === 0}>
			Start Converting
		</button>
	</div>
</div>

<style>
	.file-list-container {
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 20px;
		gap: 20px;
	}

	.drag-drop-zone {
		flex: 1;
		border: 2px dashed #ccc;
		border-radius: 8px;
		background: #fafafa;
		transition: all 0.3s ease;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.drag-drop-zone.active {
		border-color: #667eea;
		background: #f0f3ff;
	}

	.zone-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 12px 16px;
		border-bottom: 1px solid #e0e0e0;
		background: white;
	}

	.left-controls,
	.right-controls {
		display: flex;
		gap: 8px;
		align-items: center;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 6px;
		cursor: pointer;
		font-size: 14px;
		font-weight: 500;
		color: #333;
	}

	.checkbox-label input {
		cursor: pointer;
	}

	.icon-btn {
		width: 32px;
		height: 32px;
		border: 1px solid #ddd;
		border-radius: 4px;
		background: white;
		cursor: pointer;
		font-size: 16px;
		transition: all 0.2s;
	}

	.icon-btn:hover {
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

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		flex: 1;
		gap: 12px;
		color: #999;
	}

	.empty-icon {
		font-size: 48px;
		opacity: 0.5;
	}

	.empty-state p {
		font-size: 14px;
	}

	.empty-state p.or {
		font-size: 12px;
		margin-top: 8px;
	}

	.select-btn {
		margin-top: 8px;
		padding: 8px 16px;
		background: #667eea;
		color: white;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		font-size: 14px;
		font-weight: 500;
	}

	.select-btn:hover {
		background: #5568d3;
	}

	.file-rows {
		flex: 1;
		overflow-y: auto;
		padding: 8px 0;
	}

	.action-bar {
		display: flex;
		gap: 12px;
		padding: 16px;
		background: white;
		border-top: 1px solid #e0e0e0;
		border-radius: 8px;
	}

	.convert-btn {
		flex: 1;
		padding: 12px 24px;
		background: #667eea;
		color: white;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		font-size: 16px;
		font-weight: 600;
		transition: background 0.2s;
	}

	.convert-btn:hover:not(:disabled) {
		background: #5568d3;
	}

	.convert-btn:disabled {
		background: #ccc;
		cursor: not-allowed;
	}
</style>
