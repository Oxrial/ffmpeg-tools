<script>
	import tabler from 'yesvelte/css/tabler.min.css?url'
	let { children } = $props()
</script>

<svelte:head>
	<link rel="stylesheet" href={tabler} />
</svelte:head>

{@render children()}
