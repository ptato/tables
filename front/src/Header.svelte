<script>
    export let currentPage = "/";
    function setCurrentPage(page) {
        currentPage = page;
        console.log(`Current page is ${page}`);
    }
</script>

<header>
    <img src="/favicon.png" alt="ttt Icon">
    <h1>ttt</h1>

    <a href="javascript:;" on:click={ _ => currentPage = '/' }>Home</a>
    <a href="javascript:;" on:click={ _ => currentPage = '/anim' }>Anim</a>
</header>

<style>
header {
    height: 32px;
    background-color: black;
    
    font-family: monospace;

    box-shadow: 0 0 5px black;

    display: flex;
    align-items: center;
}
header img {
    margin-left: 16px;
    margin-right: 8px;

    max-height: 16px;
}
header h1 {
    margin: 0;
    color: white;
    display: block;

    font-size: 1rem;
}
header a {
    padding-left: 16px;
    color: white;
}
</style>
