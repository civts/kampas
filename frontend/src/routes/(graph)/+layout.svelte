<script lang="ts">
	import '../../style/app.scss';
</script>

<slot />

<style>
	:global(body) {
		margin: 0;
		background-color: var(--graph-bg);
	}
</style>
