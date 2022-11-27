<script>
  import Todos from "../components/Todos.svelte";

  let todos = [
    { id: 1, text: "Get good at Warzone 2.0", completed: true, archived: false },
    { id: 2, text: "Win a Warzone 2.0 game", completed: false, archived: false },
    { id: 3, text: "Try a microwaved sandwich", completed: true, archived: false },
    { id: 4, text: "Make a mental note to never microwave sandwiches again", completed: false, archived: false }
  ];
</script>

<Todos todos={todos} />

<style>
    :global(*) {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }


    :global(body) {
        background: #F8FAFC;
    }
</style>