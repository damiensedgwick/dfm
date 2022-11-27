<script>
  import CreateTodo from "./CreateTodo.svelte";

  export let addTodo;
</script>

<section>
  <div>
    <button>Active</button>
    <button>Completed</button>
    <button>Archived</button>
  </div>

  <CreateTodo addTodo={addTodo} />
</section>

<style>
  section {
      width: 100%;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
  }

  button {
      box-sizing: border-box;
      width: 125px;
      padding: 5px;
      margin-bottom: 1rem;
      background: #FFFFFF;
      border: none;
      border-radius: 1rem;
      box-shadow: 0 1px 3px rgba(16, 24, 40, 0.1), 0 1px 2px rgba(16, 24, 40, 0.06);
  }
</style>