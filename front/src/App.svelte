<script>
    import Header from './Header.svelte';
    import Card from './Card.svelte';
    import ImageList from './ImageList.svelte';

    const images = [];
    for (let i = 0; i < 20; i++) {
        let src = 'https://placedog.net/500?random&idx='+i;
        images.push({src: src, alt: 'PLACEHOLDER'});
    }
</script>

<Header/>
<main>
    <ImageList {images}/>
</main>

<style>
main {
    background-color: #fafafa;
    height: 100%;

    padding: 8px;
    font-family: Roboto, sans-serif;
}
</style>
