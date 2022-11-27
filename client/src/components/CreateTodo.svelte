<script>
  export let addTodo;

  let todo = { text: "", completed: false, archived: false };

  const onInput = (event) => {
    todo.text = event.target.value;
  };

  const onKeyDown = (e) => {
    if (e.key !== "Enter") return;

    if (!todo.text.length) {
      alert("Please enter a todo");
    } else {
      addTodo(todo);
      todo.text = "";
    }
  };
</script>

<input type="text" placeholder="Add Todo" on:input={onInput} on:keydown={onKeyDown} bind:value={todo.text}>

<style>
    input {
        box-sizing: border-box;
        width: 100%;
        max-width: 400px;
        padding: 10px;
        margin-bottom: 1rem;
        border: none;
        border-radius: 1rem;
        box-shadow: 0 1px 3px rgba(16, 24, 40, 0.1), 0 1px 2px rgba(16, 24, 40, 0.06);
    }
</style>