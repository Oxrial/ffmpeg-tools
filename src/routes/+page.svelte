<script>
	import { invoke } from '@tauri-apps/api/core'
	import { SortableList, SortableItem, sortItems } from '@rodrigodagostino/svelte-sortable-list'
	import { Card, Button, Badge, Icon, Tooltip } from 'yesvelte'
	let folderPath = ''
	let files = []
	let sortedFiles = []

	async function selectFolder() {
		try {
			folderPath = await invoke('select_folder')
			files = await invoke('scan_flv_files', { path: folderPath })
			sortedFiles = files.map((f) => ({
				name: f.substring(f.lastIndexOf('/') + 1),
				id: folderPath + f,
				delete: false
			}))
			console.log('🚀 ~ selectFolder ~ files:', files)
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
	<Button color="info" onclick={selectFolder}>选择文件夹 {folderPath}</Button>
	<div>
		<Card class="flv-list">
			<h2>扫描到的 FLV 文件</h2>
			<!-- 使用 SortableList 实现拖拽排序 -->
			<SortableList hasBoundaries on:sort={handleSort}>
				{#each sortedFiles as file, index (file.id)}
					<div>
						<SortableItem {...file} {index}>
							<li class="flv-item">
								<Badge ghost color="cyan">{file.name}</Badge>
								<Badge ghost color="cyan">{file.id}</Badge>
								<Tooltip placement="top" trigger="click">{file.id}</Tooltip>
								<Icon
									name="square-rounded-x"
									on:click={() => {
										file.delete ? undoDelete(index) : deleteFile(index)
									}}>{file.delete ? '撤销' : '删除'}</Icon
								>
							</li>
						</SortableItem>
					</div>
				{/each}
			</SortableList>
		</Card>
		<Button onclick={confirmAndMerge}>确认排序并合并视频</Button>
	</div>
</main>

<style lang="scss">
	main {
		padding: 20px;
		font-family: Arial, sans-serif;
		:global(.flv-list) {
			padding: 20px;
			min-height: calc(100vh - 25rem);
			:global(.flv-item) {
				display: flex;
				justify-content: space-evenly;
				align-items: center;
			}
		}
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
		li {
			display: flex;
			justify-content: space-around;
		}
	}
</style>
