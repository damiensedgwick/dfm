<script>
  import CreateTodo from "./CreateTodo.svelte";
</script>

<section>
  <div>
    <button>Active</button>
    <button>Completed</button>
    <button>Archived</button>
  </div>

  <CreateTodo />
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
      border: 1px solid #ccc;
      border-radius: 1rem;
  }
</style>