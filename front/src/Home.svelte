<script>
    import ImageList from './ImageList.svelte';
    import DragBoard from './DragBoard.svelte';
    import { onMount } from 'svelte';

    const images = [];
    for (let i = 0; i < 20; i++) {
        let src = 'https://placedog.net/500?random&idx='+i;
        images.push({src: src, alt: 'PLACEHOLDER'});
    }

</script>

<div class="home-container">
    <ImageList {images}/>
    <DragBoard />
</div>

<style>
.home-container {
    height: 100%;
    width: 100%;
    display: flex;
    align-items: flex-start;
}
</style>
