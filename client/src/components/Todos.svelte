<script>
  import Controls from "./Controls.svelte";
  import Todo from "./Todo.svelte";

  export let todos;
</script>

<section>
  <h2>Your To-Do's</h2>

  <Controls />

  {#if todos}
    <ul>
      {#each todos as todo}
          <Todo {todo} />
      {/each}
    </ul>
  {/if}
</section>

<style>
    section {
        width: 100%;
        max-width: 960px;
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 1rem;
    }

    h2 {
        width: 100%;
        text-align: left;
    }

    ul {
        margin-top: 0;
        width: 100%;
        list-style: none;
        padding: 0;
    }
</style>