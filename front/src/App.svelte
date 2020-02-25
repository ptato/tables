<script>
    import Header from './Header.svelte';
    import Card from './Card.svelte';
    import Home from './Home.svelte';
    import Animations from './Animations.svelte';

    let currentPage;
</script>

<Header bind:currentPage={currentPage}/>
<main>
    {#if currentPage === '/anim'}
        <Animations/>
    {:else}
        <Home />
    {/if}
</main>

<style>
main {
    background-color: #fafafa;
    height: calc(100vh - 32px); 

    box-sizing: border-box;
    font-family: Roboto, sans-serif;
}


</style>
