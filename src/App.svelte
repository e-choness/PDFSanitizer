<script>
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { open } from '@tauri-apps/plugin-dialog';
	import FileList from './components/FileList.svelte';
	import Settings from './components/Settings.svelte';
	import './App.css';

	let files = [];
	let settings = {
		removeMetadata: true,
		removeScripts: true,
		removeEmbeddedFiles: true,
		compressImages: false,
		highCompression: false,
		stripExternalLinks: false,
		fontSubsetting: false,
		maxConcurrent: 4,
		outputFolder: '',
	};
	let dragActive = false;

	onMount(async () => {
		try {
			const saved = await invoke('load_settings');
			settings = { ...settings, ...saved };
		} catch (e) {
			console.error('Failed to load settings:', e);
		}

		// Listen for file completion events
		const unlistenComplete = await listen('file_complete', (event) => {
			const { id, output_size } = event.payload;
			files = files.map(f =>
				f.id === id ? { ...f, status: 'done', progress: 100, outputSize: output_size } : f
			);
		});

		// Listen for file error events
		const unlistenError = await listen('file_error', (event) => {
			const { id, error } = event.payload;
			files = files.map(f =>
				f.id === id ? { ...f, status: 'error', error } : f
			);
		});

		// Listen for progress events
		const unlistenProgress = await listen('file_progress', (event) => {
			const { id, progress } = event.payload;
			files = files.map(f =>
				f.id === id ? { ...f, progress, status: 'processing' } : f
			);
		});

		// Use Tauri's drag-drop event to get real file paths
		const unlistenDrop = await listen('tauri://drag-drop', (event) => {
			dragActive = false;
			const paths = event.payload.paths;
			if (!paths) return;
			addFilePaths(paths);
		});

		const unlistenDragOver = await listen('tauri://drag-over', () => {
			dragActive = true;
		});

		const unlistenDragLeave = await listen('tauri://drag-leave', () => {
			dragActive = false;
		});

		return () => {
			unlistenComplete();
			unlistenError();
			unlistenProgress();
			unlistenDrop();
			unlistenDragOver();
			unlistenDragLeave();
		};
	});

	function handleDragEnter(e) {
		e.preventDefault();
		e.stopPropagation();
	}

	function handleDragLeave(e) {
		e.preventDefault();
		e.stopPropagation();
	}

	function handleDragOver(e) {
		e.preventDefault();
		e.stopPropagation();
	}

	function handleDrop(e) {
		e.preventDefault();
		e.stopPropagation();
		// Handled by tauri://drag-drop listener in onMount
	}

	function addFilePaths(paths) {
		const pdfs = paths.filter(p => p.toLowerCase().endsWith('.pdf'));
		const newFiles = pdfs.map(p => ({
			id: Math.random(),
			name: p.split(/[\\/]/).pop(),
			path: p,
			size: 0,
			progress: 0,
			status: 'pending',
			error: null,
			outputSize: null,
			selected: true,
		}));
		files = [...files, ...newFiles];
	}

	async function addFiles() {
		try {
			const selected = await open({
				multiple: true,
				filters: [{ name: 'PDF', extensions: ['pdf'] }],
			});
			if (!selected) return;
			const paths = Array.isArray(selected) ? selected : [selected];
			addFilePaths(paths);
		} catch (e) {
			console.error('Failed to open file picker:', e);
		}
	}

	async function selectFolder() {
		try {
			const folder = await open({ directory: true, multiple: false });
			if (folder) {
				settings.outputFolder = folder;
				await invoke('save_settings', { settings });
			}
		} catch (e) {
			console.error('Failed to select folder:', e);
		}
	}

	async function startProcessing() {
		const selectedFiles = files.filter(f => f.selected && f.status === 'pending');
		if (selectedFiles.length === 0) return;

		try {
			await invoke('process_files', {
				files: selectedFiles.map(f => ({ id: f.id, path: f.path })),
				settings,
			});

			// Update file statuses
			files = files.map(f =>
				selectedFiles.find(sf => sf.id === f.id)
					? { ...f, status: 'processing' }
					: f
			);
		} catch (e) {
			console.error('Failed to start processing:', e);
		}
	}

	function handleFileUpdate(e) {
		const updated = e.detail;
		files = files.map(f => f.id === updated.id ? { ...f, ...updated } : f);
	}

	function handleSettingsChange(e) {
		settings = e.detail;
		invoke('save_settings', { settings }).catch(console.error);
	}
</script>

<div class="container">
	<div class="main-content">
		<FileList
			bind:files
			on:update={handleFileUpdate}
			{dragActive}
			{handleDragEnter}
			{handleDragLeave}
			{handleDragOver}
			{handleDrop}
			{addFiles}
			on:startProcessing={startProcessing}
			on:selectFolder={selectFolder}
		/>
		<Settings
			bind:settings
			{selectFolder}
			on:change={handleSettingsChange}
		/>
	</div>
</div>

<style>
	:global(*) {
		margin: 0;
		padding: 0;
		box-sizing: border-box;
	}

	:global(body) {
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
		background: #f5f5f5;
		color: #333;
	}

	.container {
		display: flex;
		height: 100vh;
	}

	.main-content {
		display: flex;
		flex-direction: column;
		flex: 1;
	}
</style>
