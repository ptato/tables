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
            <img {src} {alt} on:click={clickImage}>
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
</style>
