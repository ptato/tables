<script>
import { onMount } from 'svelte';
let draggable;
onMount(_ => {
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
        Click here to move
    </div>
    <slot/>
</div>

<style>
.draggable {
    position: absolute;
    z-index: 1;
    background-color: #f1f1f1;
    border: 1px solid #d3d3d3;
    text-align: center;
}

.draggable-header {
    padding: 10ps;
    cursor: move;
    z-index: 2;
    background-color: #2196F3;
    color: #ffffff;
}
</style>
