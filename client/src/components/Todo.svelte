<script>
  import GoTrashcan from 'svelte-icons/go/GoTrashcan.svelte'

  export let todo;
</script>

<li>
  <input type="checkbox" bind:checked={todo.completed}>
  <span>{todo.text}</span>
  <div class="controls">
    <button class="icon"><GoTrashcan /></button>
  </div>
</li>

<style>
    li > input {
        margin-right: 1em;
        cursor: pointer;
    }

    li {
        display: flex;
        align-items: center;
        padding: 10px;
        margin-bottom: 1rem;
        background: #FFFFFF;
        border-radius: 8px;
        box-shadow: 0 1px 3px rgba(16, 24, 40, 0.1), 0 1px 2px rgba(16, 24, 40, 0.06);
    }

    div {
        margin-left: auto;
    }

    button {
        cursor: pointer;
        border: none;
        background: none;
    }

    .icon {
        color: #94A3B8;
        height: 15px;
    }
</style>