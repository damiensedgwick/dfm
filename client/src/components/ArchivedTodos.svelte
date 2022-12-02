<script>
  import { crossfade, fade } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { cubicOut } from "svelte/easing";
  import TodoControls from "./TodoControls.svelte";

  const [send, receive] = crossfade({
    duration: d => Math.sqrt(d * 350),
    easing: cubicOut,
    fallback: fade
  });

  export let todos;
  export let archiveTodo;
  export let deleteTodo;
</script>

<section>
  <div>
    <ul>
      {#each todos.reverse() as todo (todo.id)}
        <li animate:flip={{duration: 500, easing: cubicOut }} in:receive={{key: todo.id}} out:send={{key: todo.id}}>
          {todo.title}
          <TodoControls todo={todo} archiveTodo={archiveTodo} deleteTodo={deleteTodo} />
        </li>
      {/each}
    </ul>
  </div>
</section>

<style>
    section {
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        padding: 0 12px;
        display: flex;
        justify-content: center;
        align-items: center;
        border-top: 4px solid #475569;
        max-height: 250px;
        overflow-y: auto;
    }

    div {
        width: 100%;
        max-width: 960px;
        margin-bottom: 18px;
    }

    ul {
        margin-top: 75px;
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
</style>