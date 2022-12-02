<script>
  import Drawer from 'svelte-drawer-component'
  import TodoControls from "./TodoControls.svelte";

  export let todos;
  export let archiveTodo;
  export let deleteTodo;
  export let toggleArchived;
</script>

<section>
  <Drawer {open} size='250px' placement='bottom' on:clickAway={toggleArchived}>
    <div>
      {#if todos.length}
        <ul>
          {#each todos.reverse() as todo (todo.id)}
            <li>
              {todo.title}
              <TodoControls todo={todo} archiveTodo={archiveTodo} deleteTodo={deleteTodo} />
            </li>
          {/each}
        </ul>
      {:else}
        <p>Nothing to see here!</p>
      {/if}
    </div>
  </Drawer>
</section>

<style>
    div {
        width: 100%;
        max-width: 960px;
        margin-bottom: 18px;
    }

    ul {
        margin-top: 25px;
        padding: 0 12px;
        list-style: none;
    }

    li {
        margin: 12px 0;
        padding: 12px;
        display: flex;
        align-items: center;
        justify-content: space-between;
        background: white;
        border-radius: 12px;
        box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);
    }

    p {
        text-align: center;
        padding-top: 12px;
    }
</style>