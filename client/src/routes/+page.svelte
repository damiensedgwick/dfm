<script lang="ts">
  import { crossfade, fade } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { cubicOut } from "svelte/easing";
  import GoTrashcan from 'svelte-icons/go/GoTrashcan.svelte'
  import CreateTodoInput from "../components/CreateTodoInput.svelte";

  const [send, receive] = crossfade({
    duration: d => Math.sqrt(d * 350),
    easing: cubicOut,
    fallback: fade
  });

  type Todo = {
    id: number;
    title: string;
    completed: boolean;
    archived: boolean;
  }

  let todos: Todo[] = [
    {
      id: 1,
      title: "win a game of warzone 2.0",
      completed: false,
      archived: false
    },
    {
      id: 2,
      title: "buy some cheese",
      completed: false,
      archived: false
    },
    {
      id: 3,
      title: "actually complete a side project",
      completed: false,
      archived: false
    }
  ];
  let completed: Todo[] = [];

  function complete(id) {
    if (completed.includes(id)) {
      completed = completed.filter(completedId => completedId !== id);
    } else {
      completed = [...completed, id];
    }
  }

  function archive(id) {
    todos = todos.filter(todo => todo.id !== id);
  }

  let createTodo = (todo) => {
    todos = [...todos, { id: todos.length + 1, ...todo }];
  };
</script>

<section>
  <div>
    <div>
      <h1>Your To-Dos</h1>
      <CreateTodoInput createTodo={createTodo} />
    </div>

    <ul>
      {#each todos.filter(todo => !completed.includes(todo.id)).reverse() as todo (todo.id)}
        <li animate:flip={{duration: 500, easing: cubicOut}} in:receive={{key: todo.id}} out:send={{key: todo.id}}>
          <input type="checkbox" bind:checked={todo.completed} on:click={() => complete(todo.id)}>
          {todo.title}
          <button class="icon" on:click={() => archive(todo.id)}><GoTrashcan /></button>
        </li>
      {/each}
    </ul>

    <hr>

    <ul>
      {#each todos.filter(todo => completed.includes(todo.id)).reverse() as todo (todo.id)}
        <li animate:flip={{duration: 500, easing: cubicOut }} in:receive={{key: todo.id}} out:send={{key: todo.id}}>
          <input type="checkbox" bind:checked={todo.completed} on:click={() => complete(todo.id)}>
          {todo.title}
          <button class="icon" on:click={() => archive(todo.id)}><GoTrashcan /></button>
        </li>
      {/each}
    </ul>
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
        display: flex;
        justify-content: center;
        align-items: center;
    }

    div {
        width: 100%;
        max-width: 960px;
        margin-bottom: 12px;
    }

    div > div {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    h1 {
        font-size: 2.5rem;
        font-weight: 300;
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
        background: white;
        border-radius: 12px;
        box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);
    }

    input {
        margin-right: 12px;
        cursor: pointer;
        accent-color: #059669;
    }

    li > input:checked {
        border: 1px solid #059669;
        box-shadow: 0 0 10px #719ece;
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