<script>
    export let title = "";
    export let subtitle = "";
</script>

<a class="card" href="">
    <slot name="media"/>
    <div class="card-content">
        <span class="card-title">{title}</span>
        <span class="card-subtitle">{subtitle}</span>
    </div>
</a>

<style>
.card {
    max-width: 300px;

    background-color: #ffffff;
    border: 1px solid #e0e0e0;
    border-radius: 4px;

    display: inline-block;
    text-decoration: none;

    transition-property: filter box-shadow;
    transition-duration: 0.25s;
}
.card:hover {
    filter: brightness(90%);
    box-shadow: 0 0 4px lightgray;
}
.card :global(img) {
    max-width: 100%;
}
.card .card-content {
    padding: 8px 8px;
}
.card .card-title {
    font-size: 1rem;
    display: block;
    font-weight: bold;
    color: black;

    overflow: hidden;
    text-overflow: ellipsis;
}
.card .card-subtitle {
    font-size: 0.8rem;
    color: #888;
    display: block;
}
</style>
