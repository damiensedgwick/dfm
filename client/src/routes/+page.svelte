<script lang="ts">
  import { crossfade, fade } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { cubicOut } from "svelte/easing";
  import CreateTodoInput from "../components/CreateTodoInput.svelte";
  import TodoControls from "../components/TodoControls.svelte";
  import ArchivedTodos from "../components/ArchivedTodos.svelte";

  const [send, receive] = crossfade({
    duration: d => Math.sqrt(d * 350),
    easing: cubicOut,
    fallback: fade
  });

  type Todo = {
    id?: number;
    title: string;
    completed: boolean;
    archived: boolean;
  }

  let todos: Todo[] = [];

  const fetchTodos = (async () => {
    const response = await fetch("http://localhost:8080/api/v1/todos");

    return todos = await response.json();
  })();

  async function createTodo(todo) {
    todos = [
      ...todos, { ...todo }
    ];

    await fetch(`http://localhost:8080/api/v1/todos`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({ ...todo })
    });
  }

  async function completeTodo(id) {
    const todo = todos.find(todo => todo.id === id);

    await fetch(`http://localhost:8080/api/v1/todos/${id}`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({ ...todo, completed: !todo.completed })
    });
  }

  async function archiveTodo(id) {
    const todo = todos.find(todo => todo.id === id);

    await fetch(`http://localhost:8080/api/v1/todos/${id}`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({ ...todo, archived: !todo.archived })
    });
  }

  async function deleteTodo(id) {
    await fetch(`http://localhost:8080/api/v1/todos/${id}`, {
      method: "DELETE"
    });

    todos = todos.filter(todo => todo.id !== id);
  }

  let viewArchived = false;

  function toggleArchived() {
    viewArchived = !viewArchived;
  }
</script>

<section>
  <div>
    <div>
      <CreateTodoInput createTodo={createTodo} />
      <button on:click={toggleArchived}>View Archived</button>
    </div>

    {#await fetchTodos}
      <p>Loading...</p>
    {:then value}
      <ul>
        {#each todos.filter(todo => !todo.completed && !todo.archived).reverse() as todo (todo.id)}
          <li animate:flip={{duration: 500, easing: cubicOut}} in:receive={{key: todo.id}} out:send={{key: todo.id}}>
            <div>
              <input type="checkbox" bind:checked={todo.completed} on:click={() => completeTodo(todo.id)}>
              {todo.title}
            </div>
            <TodoControls todo={todo} archiveTodo={archiveTodo} deleteTodo={deleteTodo} />
          </li>
        {/each}
      </ul>
      <hr>
      <ul>
        {#each todos.filter(todo => todo.completed && !todo.archived).reverse() as todo (todo.id)}
          <li animate:flip={{duration: 500, easing: cubicOut }} in:receive={{key: todo.id}} out:send={{key: todo.id}}>
            <div>
              <input type="checkbox" bind:checked={todo.completed} on:click={() => completeTodo(todo.id)}>
              {todo.title}
            </div>
            <TodoControls todo={todo} archiveTodo={archiveTodo} deleteTodo={deleteTodo} />
          </li>
        {/each}
      </ul>
    {:catch error}
      <p>Something went wrong: {error.message}</p>
    {/await}
  </div>
</section>

{#if viewArchived}
  <ArchivedTodos todos={todos.filter((todo) => todo.archived)} archiveTodo={archiveTodo} deleteTodo={deleteTodo}
                 toggleArchived={toggleArchived} />
{/if}

<style>
    :global(*) {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }

    :global(body) {
        background: #f1f5f9;
        font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
        font-weight: 300;
        font-size: 16px;
    }

    section {
        height: 100vh;
        width: 100vw;
        padding: 0 12px;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    div {
        width: 100%;
        max-width: 960px;
        margin-bottom: 18px;
    }

    div > div {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-between;
    }

    @media screen and (min-width: 768px) {
        div > div {
            flex-direction: row;
        }
    }

    button {
        background: #fff;
        border: 1px solid #e2e8f0;
        border-radius: 12px;
        padding: 10px 20px;
        font-size: 14px;
        font-weight: 400;
        cursor: pointer;
        box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);
        opacity: 0.75;
    }

    button:hover {
        opacity: 1;
    }

    ul {
        padding: 0;
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

    input {
        margin-right: 12px;
        cursor: pointer;
        accent-color: #059669;
    }

    input:checked {
        border: 1px solid #059669;
        box-shadow: 0 0 5px #059669;
    }

    li > div:first-of-type {
        margin: 0;
        display: flex;
        flex: 1;
        align-items: center;
    }

    hr {
        margin: 20px 0;
        border: 0;
        border-bottom: 2px solid #cbd5e1;
    }
</style>