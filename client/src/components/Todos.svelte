<script>
  import Controls from "./Controls.svelte";
  import Todo from "./Todo.svelte";

  export let todos;


</script>

<section>
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
        max-width: 960px;
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 1rem;
    }

    ul {
        margin-top: 0;
        width: 100%;
        list-style: none;
        padding: 0;
    }
</style>