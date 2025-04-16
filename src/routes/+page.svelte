<script>
	import 'carbon-components-svelte/css/white.css'
	import { invoke } from '@tauri-apps/api/core'
	import { SortableList, SortableItem, sortItems } from '@rodrigodagostino/svelte-sortable-list'
	import { Button, breakpoints } from 'carbon-components-svelte'

	let folderPath = ''
	let files = []
	let sortedFiles = []

	async function selectFolder() {
		try {
			folderPath = await invoke('select_folder')
			files = await invoke('scan_flv_files', { path: folderPath })
			sortedFiles = files.map((f) => ({ name: f.substring(f.lastIndexOf('/') + 1), id: f, delete: false }))
		} catch (error) {
			alert(`Error: ${error}`)
		}
	}

	function deleteFile(index) {
		sortedFiles[index].delete = true
		sortedFiles = sortedFiles
	}

	function undoDelete(index) {
		sortedFiles[index].delete = false
		sortedFiles = sortedFiles
	}

	async function confirmAndMerge() {
		try {
			await invoke('generate_filelist_and_merge', {
				files: sortedFiles.filter((s) => !s.delete).map((s) => s.id),
				folderPath
			})
			alert('视频合并完成！')
		} catch (error) {
			alert(`Error: ${error}`)
		}
	}
	// 拖拽排序配置
	function handleSort(e) {
		const { prevItemIndex, nextItemIndex } = e.detail
		sortedFiles = sortItems(sortedFiles, prevItemIndex, nextItemIndex)
	}
</script>

<main>
	<h1>FLV 文件扫描与合并工具</h1>

	<button on:click={selectFolder}>选择文件夹</button>
	<p>当前文件夹: {folderPath}</p>

	<div>
		<Button>Primary button</Button>
		<h2>扫描到的 FLV 文件</h2>

		<!-- 使用 SortableList 实现拖拽排序 -->
		<SortableList on:sort={handleSort}>
			{#each sortedFiles as file, index (file.id)}
				<SortableItem {...file} {index}>
					<li>
						<span>{file.name}</span><span>{file.id}</span>
						<button
							on:click={() => {
								file.delete ? undoDelete(index) : deleteFile(index)
							}}>{file.delete ? '撤销' : '删除'}</button
						>
					</li>
				</SortableItem>
			{/each}
		</SortableList>

		<button on:click={confirmAndMerge}>确认排序并合并视频</button>
	</div>
</main>

<style lang="scss">
	main {
		padding: 20px;
		font-family: Arial, sans-serif;
	}
	button {
		margin: 5px;
		padding: 10px;
		background-color: #007bff;
		color: white;
		border: none;
		cursor: pointer;
	}
	button:hover {
		background-color: #0056b3;
	}
	ul {
		list-style-type: none;
		padding: 0;
	}
</style>
