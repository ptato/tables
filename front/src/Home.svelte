<script>
    import ImageList from './ImageList.svelte';
    import { onMount } from 'svelte';

    const images = [];
    for (let i = 0; i < 20; i++) {
        let src = 'https://placedog.net/500?random&idx='+i;
        images.push({src: src, alt: 'PLACEHOLDER'});
    }

    let canvas;

    onMount(_ => {
    console.log(canvas);
        canvas.width = canvas.offsetWidth;
        canvas.height = canvas.offsetHeight;
        let c = canvas.getContext('2d');
        c.fillStyle = 'black';
        c.strokeRect(90, 10, 90, 90);
        c.fillRect(120, 30, 30, 30);
        c.strokeRect(270, 10, 90, 90);
        c.fillRect(300, 30, 30, 30);
        c.fillRect(180, 120, 20, 40);
        c.fillRect(30, 190, 400, 40);
    });
</script>

<div class="home-container">
    <ImageList {images}/>
    <canvas bind:this={canvas} width="100" height="100"></canvas>
</div>

<style>
.home-container {
    height: 100%;
    width: 100%;
    display: flex;
    align-items: flex-start;
}
canvas {
    width: 50%;
    height: 100%;
}
</style>
