<script>
    import Header from './Header.svelte';
    import Card from './Card.svelte';
	let name = 'world';
</script>

<Header/>
<main>
    <Card title="Dog card">
        <img slot="media" src="https://placedog.net/500/300?random" alt="Good dog">
    </Card>
    <Card/>
</main>

<style>
main {
    background-color: #fafafa;
    height: 100%;

    padding: 8px;
    font-family: Roboto, sans-serif;
}
</style>
