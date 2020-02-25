<script>
    let buttonRotated = false;
    let contentExpanded = false;
    function expand(event) {
        buttonRotated = !buttonRotated;
        contentExpanded = !contentExpanded;
    }
</script>

<div id="header">
	<div id="button">This is a Button
		<div class="content" class:expanded={contentExpanded}>
            This is the Hidden Div
		</div>
	</div>
    <div style="text-align: center; width: 200px;">
        <a href="javascript:;" on:click={expand}>
            <img src="https://image.flaticon.com/icons/svg/60/60781.svg"
                 alt="Expand button" width="16" class:rotated={buttonRotated}>
        </a>
    </div>
    <button>abcd</button>
    <button>wxyz</button>
</div>

<style>
#header #button {width:200px; background:#eee}

 .expanded {
    opacity:1 !important;
    height: 150px !important;
    padding: 8px !important;    
}

#header #button .content {
    opacity:0;
    clear: both;
    height: 0;
    padding: 0 8px;
    overflow: hidden;
    
    transition: all .3s ease .15s;
    
    border-top: 1px solid #EEEEEE;
    border-left: 1px solid #EEEEEE;
    border-right: 1px solid #EEEEEE;
    border-bottom: 1px solid #EEEEEE;
    
    border-radius: 0px 7px 7px 7px;
    box-shadow: 0px 2px 2px #DDDDDD;
    background: #FFF;
}

.rotated {
    transform: rotate(-180deg);
}
img {
    transition: transform 0.3s ease 0.05s;
}

</style>

