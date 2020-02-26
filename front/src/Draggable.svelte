<script>
import { onMount } from 'svelte';
export let x = 0, y = 0;
export let w = "auto", h = "auto";
let draggable;
onMount(_ => {
    draggable.style.top = y + "px";
    draggable.style.left = x + "px";
    draggable.style['max-width'] = w;
    draggable.style['max-height'] = h;

    let pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
    let header = draggable.querySelector('draggable-header');
    if (header) {
        header.onmousedown = dragMouseDown;
    } else {
        draggable.onmousedown = dragMouseDown;
    }

    function dragMouseDown(e) {
        e = e || window.event;
        e.preventDefault();

        pos3 = e.clientX;
        pos4 = e.clientY;

        document.onmouseup = closeDragElement;
        document.onmousemove = elementDrag;
    }

    function elementDrag(e) {
        e = e || window.event;
        e.preventDefault();

        pos1 = pos3 - e.clientX;
        pos2 = pos4 - e.clientY;
        pos3 = e.clientX;
        pos4 = e.clientY;

        draggable.style.top = (draggable.offsetTop - pos2) + "px";
        draggable.style.left = (draggable.offsetLeft - pos1) + "px";
    }

    function closeDragElement() {
        document.onmouseup = null;
        document.onmousemove = null;
    }
});
</script>

<div bind:this={draggable} class="draggable">
    <div class="draggable-header">
        <div class="close-button">x</div>
    </div>
    <div class="draggable-content">
        <slot/>
    </div>
</div>

<style>
.draggable {
    position: absolute;
    z-index: 1;
    background-color: #f1f1f1;
    border: 1px solid #000000;
    
    display: flex;
    flex-direction: column;
}

.draggable-header {
    padding: 10ps;
    cursor: move;
    z-index: 2;
    background-color: #7e6bed;
    color: #ffffff;
    border-bottom: 1px solid #000000;

    display: flex;
}

.close-button {
    border-left: 1px solid black;
    margin-left: auto;
    padding: 0 4px;
    box-sizing: border-box;
}

.draggable-content {
    overflow-y: scroll;
}
</style>
