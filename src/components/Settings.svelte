<script>
	import { createEventDispatcher } from 'svelte';

	export let settings;
	export let selectFolder;

	const dispatch = createEventDispatcher();

	function handleToggle(key) {
		settings[key] = !settings[key];
		dispatch('change', settings);
	}

	function handleConcurrentChange(delta) {
		const newValue = settings.maxConcurrent + delta;
		if (newValue >= 1 && newValue <= 8) {
			settings.maxConcurrent = newValue;
			dispatch('change', settings);
		}
	}

	async function handleSelectFolder() {
		await selectFolder();
		dispatch('change', settings);
	}
</script>

<div class="settings-panel">
	<div class="settings-content">
		<h2>Sanitization Options</h2>

		<div class="setting-group">
			<label class="setting-item">
				<input
					type="checkbox"
					checked={settings.removeMetadata}
					on:change={() => handleToggle('removeMetadata')}
				/>
				<span>Remove Metadata</span>
			</label>
			<p class="setting-desc">Removes author, creation date, and document properties</p>
		</div>

		<div class="setting-group">
			<label class="setting-item">
				<input
					type="checkbox"
					checked={settings.removeScripts}
					on:change={() => handleToggle('removeScripts')}
				/>
				<span>Remove Scripts</span>
			</label>
			<p class="setting-desc">Disables JavaScript and interactive elements</p>
		</div>

		<div class="setting-group">
			<label class="setting-item">
				<input
					type="checkbox"
					checked={settings.removeEmbeddedFiles}
					on:change={() => handleToggle('removeEmbeddedFiles')}
				/>
				<span>Remove Embedded Files</span>
			</label>
			<p class="setting-desc">Removes attachments and embedded objects</p>
		</div>

		<div class="divider"></div>

		<div class="setting-group">
			<label class="setting-item">
				<input
					type="checkbox"
					checked={settings.stripExternalLinks}
					on:change={() => handleToggle('stripExternalLinks')}
				/>
				<span>Strip External Links</span>
			</label>
			<p class="setting-desc">Removes URLs and external references</p>
		</div>

		<div class="setting-group">
			<label class="setting-item">
				<input
					type="checkbox"
					checked={settings.fontSubsetting}
					on:change={() => handleToggle('fontSubsetting')}
				/>
				<span>Font Subsetting</span>
			</label>
			<p class="setting-desc">Embeds only used glyphs to reduce file size</p>
		</div>

		<div class="setting-group">
			<label class="setting-item">
				<input
					type="checkbox"
					checked={settings.compressImages}
					on:change={() => handleToggle('compressImages')}
				/>
				<span>Compress Images</span>
			</label>
			<p class="setting-desc">Reduces image quality for smaller files</p>
		</div>

		<div class="divider"></div>

		<h2>Advanced</h2>

		<div class="setting-group">
			<label class="setting-label">
				Output Folder:
				<div class="folder-selector">
					<input
						type="text"
						value={settings.outputFolder}
						readonly
						placeholder="Click to select..."
					/>
					<button on:click={handleSelectFolder} class="select-folder-btn">
						Browse
					</button>
				</div>
			</label>
			<p class="setting-desc">Original PDFs will be moved here after processing</p>
		</div>

		<div class="setting-group">
			<label class="setting-label">
				Concurrent Processing:
				<div class="concurrent-control">
					<button on:click={() => handleConcurrentChange(-1)} class="ctrl-btn">
						−
					</button>
					<span class="concurrent-value">{settings.maxConcurrent}</span>
					<button on:click={() => handleConcurrentChange(1)} class="ctrl-btn">
						+
					</button>
				</div>
			</label>
			<p class="setting-desc">Number of files to process simultaneously (1-8)</p>
		</div>
	</div>
</div>

<style>
	.settings-panel {
		width: 300px;
		background: white;
		border-left: 1px solid #e0e0e0;
		overflow-y: auto;
		padding: 20px;
	}

	.settings-content {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	h2 {
		font-size: 14px;
		font-weight: 600;
		color: #333;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin-top: 8px;
	}

	.setting-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.setting-item {
		display: flex;
		align-items: center;
		gap: 8px;
		cursor: pointer;
		font-size: 13px;
		font-weight: 500;
		color: #333;
	}

	.setting-item input {
		cursor: pointer;
	}

	.setting-label {
		display: flex;
		flex-direction: column;
		gap: 8px;
		font-size: 13px;
		font-weight: 500;
		color: #333;
	}

	.setting-desc {
		font-size: 11px;
		color: #999;
		margin: 0;
		margin-left: 24px;
	}

	.divider {
		height: 1px;
		background: #e0e0e0;
		margin: 8px 0;
	}

	.folder-selector {
		display: flex;
		gap: 8px;
		margin-top: 8px;
	}

	.folder-selector input {
		flex: 1;
		padding: 6px 8px;
		border: 1px solid #ddd;
		border-radius: 4px;
		font-size: 12px;
		background: #f5f5f5;
	}

	.select-folder-btn {
		padding: 6px 12px;
		background: #667eea;
		color: white;
		border: none;
		border-radius: 4px;
		font-size: 12px;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s;
		white-space: nowrap;
	}

	.select-folder-btn:hover {
		background: #5568d3;
	}

	.concurrent-control {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-top: 8px;
	}

	.ctrl-btn {
		width: 28px;
		height: 28px;
		border: 1px solid #ddd;
		background: white;
		border-radius: 4px;
		cursor: pointer;
		font-size: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.2s;
	}

	.ctrl-btn:hover {
		background: #f5f5f5;
		border-color: #999;
	}

	.concurrent-value {
		min-width: 30px;
		text-align: center;
		font-weight: 600;
		color: #667eea;
	}
</style>
