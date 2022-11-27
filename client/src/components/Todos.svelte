<script>
  import Controls from "./Controls.svelte";
  import Todo from "./Todo.svelte";

  export let todos;

  let archiveTodoToggle = (id) => {
    todos = todos.map((todo) => {
      if (todo.id === id) {
        todo.archived = !todo.archived;
      }

      return todo;
    });
  };

  let showActive = true;
  let showCompleted = false;
  let showArchived = false;

  let filterTodos = (todos) => {
    if (showActive) {
      todos = todos.filter((todo) => !todo.completed);
    }

    if (showCompleted) {
      todos = todos.filter((todo) => todo.completed);
    }

    if (showArchived) {
      todos = todos.filter((todo) => todo.archived);
    }

    return todos;
  };

  let addTodo = (todo) => {
    todos = [...todos, { id: todos.length + 1, ...todo }];
  };
</script>

<section>
  <h2>Your To-Do's</h2>

  <Controls addTodo={addTodo} />

  {#if todos}
    <ul>
      {#each filterTodos(todos) as todo}
        <Todo {todo} archiveTodoToggle={archiveTodoToggle} />
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