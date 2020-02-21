<script>
    export let images = [ ];

    let columns = [ [], [], [], [] ];
    for (let imageIndex = 0; imageIndex < images.length; imageIndex++) {
        columns[imageIndex % 4].push(images[imageIndex]);
    }

    function clickImage(event) {
        console.log(this);
    }
</script>

<div class="image-list">
    {#each columns as column, columnIndex}
        <div class="column">
        {#each column as { src, alt }, imageIndex }
            <div class="flip-card" ontouchstart="this.classList.toggle('hover');">
                <div class="flip-card-inner">
                    <div class="flip-card-front">
                        <img {src} {alt} on:click={clickImage}>
                    </div>
                    <div class="flip-card-back">
                        <h1>Hola</h1>
                        <p>Arquitecto e Ingeniero</p>
                        <p>Buena peñita</p>
                    </div>
                </div>
            </div>
        {/each}
        </div>
    {/each}
</div>

<style>
.image-list {
    display: flex;
}
.image-list .column {
    display: flex;
    flex-direction: column;
    width: 25%;
    margin: 0 8px;
}
.image-list img {
    margin: 8px 0;
    box-sizing: border-box;
    width: 100%;

    transition-property: filter box-shadow;
    transition-duration: 0.25s;
}
.image-list img:hover {
    filter: brightness(90%);
    box-shadow: 0 0 4px lightgray;
}

.flip-card {
    perspective: 1000px;
}
.flip-card:hover .flip-card-inner, .flip-card.hover .flip-card-inner {
    transform: rotateY(180deg);
}
.flip-card, .flip-card-front, .flip-card-back {
    width: 100%;
}
.flip-card-inner {
    transition: 0.6s;
    transform-style: preserve-3d;
    position: relative;
}
.flip-card-front, .flip-card-back {
    backface-visibility: hidden;
    /*position: absolute;
    top: 0;
    left: 0;*/
}
.flip-card-front {
    z-index: 2;
    transform: rotateY(0deg);
}
.flip-card-back {
    transform: rotateY(180deg);
}
</style>
