<script>
    export let title = "Card title";
</script>

<a class="card">
    <slot name="media"/>
    <div class="card-content">
        <span class="card-title">{title}</span>
        <span class="card-subtitle">Secondary text</span>
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
