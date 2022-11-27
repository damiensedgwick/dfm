<script lang="ts">
  export let createTodo;

  let todo = { title: "", completed: false, archived: false };

  const onInput = (event) => {
    todo.title = event.target.value;
  };

  const onKeyDown = (e) => {
    if (e.key !== "Enter") return;

    if (!todo.title.length) {
      alert("Please enter a todo");
    } else {
      createTodo(todo);
      todo.title = "";
    }
  };
</script>

<input type="text" placeholder="Create Todo" on:input={onInput} on:keydown={onKeyDown} bind:value={todo.title}>

<style>
    input {
        box-sizing: border-box;
        width: 100%;
        max-width: 400px;
        padding: 12px;
        border: none;
        border-radius: 12px;
        box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);
    }
</style>