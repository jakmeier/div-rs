<script>
    let elements = [];
    function hello() {
        alert("hello");
    }
    function addRandomElement() {
        const colNum = Math.floor(Math.random() * Math.pow(256, 3));
        const color = "#" + ("00000" + colNum.toString(16)).slice(-6);
        console.log(color);
        elements = [...elements, { color }];
    }
</script>

<style>
    .random-elements {
        display: grid;
        grid-template-columns: repeat(6, 1fr);
    }
    .random-elements > p {
        width: 50px;
        height: 50px;
    }
</style>

<div>
    <h2>Hello Svelte Component</h2>
    <button on:click={hello}>Hi!</button>
    <button on:click={addRandomElement}>Add random element</button>
    <section class="random-elements">
        {#each elements as el}
            <p style="background-color: {el.color}" />
        {/each}
    </section>
</div>
