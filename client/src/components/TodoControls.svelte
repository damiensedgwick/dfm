<script>
  import GoTrashcan from "svelte-icons/go/GoTrashcan.svelte";
  import GoArchive from "svelte-icons/go/GoArchive.svelte";
  import GoPencil from "svelte-icons/go/GoPencil.svelte";

  export let todo;
  export let archiveTodo;
  export let deleteTodo;
</script>

<div>
  <button class="icon">
    <GoPencil />
  </button>
  <button class="icon" on:click={() => archiveTodo(todo.id)}>
    <GoArchive />
  </button>
  <button class="icon" on:click={() => deleteTodo(todo.id)}>
    <GoTrashcan />
  </button>
</div>

<style>
    .icon {
        height: 16px;
        width: 16px;
        color: #dc2626;
        margin-left: auto;
        background: none;
        border: none;
        cursor: pointer;
    }

    div {
        width: 100px;
        margin: 0;
        display: flex;
        align-items: center;
        justify-content: space-between;
    }
</style>