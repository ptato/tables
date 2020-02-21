<script>
    import { onMount } from 'svelte';
    export let images = [ ];

    let columns = [ [], [], [], [] ];
    for (let imageIndex = 0; imageIndex < images.length; imageIndex++) {
        columns[imageIndex % 4].push(images[imageIndex]);
    }

    let imageList;

    onMount(async () => {
        imageList.querySelectorAll('img').forEach(img => {
            img.onload = _ => {
                img.parentElement.style.height = `${img.offsetHeight+1}px`;
            }
        });
    });

    function clickImage(event) {
        let img = this.querySelector('img');
        let back = this.querySelector('.img-back');
        if (img.style.display === 'none') {
            img.style.display = 'block';
            back.style.display = 'none';
        } else {
            back.style.display = 'block';
            img.style.display = 'none';
        }
    }
</script>

<div class="image-list" bind:this={imageList}>
    {#each columns as column, columnIndex}
        <div class="column">
        {#each column as { src, alt }, imageIndex }
            <div class="row" on:click={clickImage}>
                <img {src} {alt}>
                <div class="img-back">
                    <h1>Hola</h1>
                    <p>Arquitecto e Ingeniero</p>
                    <p>Buena peñita</p>
                </div>
            </div>
        {/each}
        </div>
    {/each}
</div>

<style>
.image-list {
    display: flex;
    perspective: 1000px;
}
.image-list .column {
    display: flex;
    flex-direction: column;
    width: 25%;
    margin: 0 8px;
}
.row {
    margin: 8px 0;
    box-sizing: border-box;
    border: 1px solid black;
    width: 100%;

    transition-property: filter box-shadow transform;
    transition-duration: 0.25s;
}
.row:hover {
    filter: brightness(90%);
    box-shadow: 0 0 4px lightgray;
}
.row:active {
    transform: scale(0.95);
}

.image-list img {
    width: 100%;
    backface-visibility: hidden;
}
.image-list .img-back {
    display: none;
    padding: 8px;
    height: 100%;
    backface-visibility: hidden;
}

@keyframes flip {
    0% {
        transform: rotateY(0deg);
    }
    100% {
        transform: rotateY(180deg);
    }
}
</style>
