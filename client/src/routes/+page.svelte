<script lang="ts">
  import { crossfade, fade } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { cubicOut } from "svelte/easing";
  import GoTrashcan from "svelte-icons/go/GoTrashcan.svelte";
  import GoArchive from "svelte-icons/go/GoArchive.svelte";
  import GoPencil from "svelte-icons/go/GoPencil.svelte";
  import CreateTodoInput from "../components/CreateTodoInput.svelte";

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
      ...todos, {...todo}
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
</script>

<section>
  <div>
    <div>
      <CreateTodoInput createTodo={createTodo} />
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
            <div>
              <button class="icon" on:click={() => archiveTodo(todo.id)}>
                <GoPencil />
              </button>
              <button class="icon" on:click={() => archiveTodo(todo.id)}>
                <GoArchive />
              </button>
              <button class="icon" on:click={() => deleteTodo(todo.id)}>
                <GoTrashcan />
              </button>
            </div>
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
            <div>
              <button class="icon" on:click={() => archiveTodo(todo.id)}>
                <GoPencil />
              </button>
              <button class="icon" on:click={() => archiveTodo(todo.id)}>
                <GoArchive />
              </button>
              <button class="icon" on:click={() => deleteTodo(todo.id)}>
                <GoTrashcan />
              </button>
            </div>
          </li>
        {/each}
      </ul>
    {:catch error}
      <p>Something went wrong: {error.message}</p>
    {/await}
  </div>
</section>

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

    li > div:last-of-type {
        width: 100px;
        margin: 0;
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    hr {
        margin: 20px 0;
        border: 0;
        border-bottom: 2px solid #cbd5e1;
    }

    .icon {
        height: 16px;
        width: 16px;
        color: #dc2626;
        margin-left: auto;
        background: none;
        border: none;
        cursor: pointer;
    }
</style>